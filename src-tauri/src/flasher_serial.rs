use anyhow::{anyhow, Context, Result};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use serialport::{SerialPortType, UsbPortInfo};
use std::path::Path;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedPort {
    pub port_name: String,
    pub description: String,
    pub is_esp: bool,
}

pub struct SerialFlasher;

impl SerialFlasher {
    /// Liste tous les ports séries disponibles et met en évidence les puces Espressif
    pub fn list_ports() -> Result<Vec<DetectedPort>> {
        let ports = serialport::available_ports()
            .context("Impossible d'énumérer les ports série")?;

        let mut results = Vec::new();

        for p in ports {
            let mut desc = "Port série générique".to_string();
            let mut is_esp = false;

            if let SerialPortType::UsbPort(UsbPortInfo { vid, pid, product, manufacturer, .. }) = &p.port_type {
                let prod = product.as_deref().unwrap_or("Inconnu");
                let mfg = manufacturer.as_deref().unwrap_or("Inconnu");
                desc = format!("USB: {} ({}) [VID: {:04x}, PID: {:04x}]", prod, mfg, vid, pid);

                // 0x303a = Espressif Systems
                if *vid == 0x303a {
                    is_esp = true;
                    desc = format!("🔥 ESP32-S3 ({})", prod);
                } else if *vid == 0x1a86 || *vid == 0x10c4 || *vid == 0x0403 {
                    // Adaptateurs USB-Série classiques (CH340, CP2102, FTDI)
                    is_esp = true;
                    desc = format!("⚡ Adaptateur Série/USB ({})", prod);
                }
            }

            results.push(DetectedPort {
                port_name: p.port_name,
                description: desc,
                is_esp,
            });
        }

        // Trier avec les ESP32 en premier
        results.sort_by(|a, b| b.is_esp.cmp(&a.is_esp));
        Ok(results)
    }

    /// Vérifie si l'outil esptool est disponible dans le système (fallback)
    pub fn check_esptool() -> Option<String> {
        for cmd in &["esptool", "esptool.py"] {
            if let Ok(output) = Command::new(cmd).arg("version").output() {
                if output.status.success() {
                    let v = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    return Some(format!("{} ({})", cmd, v));
                }
            }
        }
        None
    }

    /// Flashe un binaire USB avec l'adresse mémoire offset spécifiée en pur Rust (espflash)
    /// (0x0000 pour factory, 0x10000 pour app update)
    pub fn flash_usb_bin<F>(port: &str, bin_path: &Path, offset: &str, baud_rate: u32, on_progress: F) -> Result<()>
    where
        F: Fn(u32, &str) + Send + Sync,
    {
        if !bin_path.exists() {
            return Err(anyhow!("Le fichier binaire n'existe pas : {:?}", bin_path));
        }

        let addr = if offset.starts_with("0x") || offset.starts_with("0X") {
            u32::from_str_radix(&offset[2..], 16)
                .with_context(|| format!("Adresse offset hexadécimale invalide : {}", offset))?
        } else {
            offset.parse::<u32>()
                .with_context(|| format!("Adresse offset invalide : {}", offset))?
        };

        let bin_data = std::fs::read(bin_path)
            .with_context(|| format!("Impossible de lire le fichier firmware : {:?}", bin_path))?;

        println!("\n{} Début du flashage USB (Rust natif / espflash) sur {} à {} bauds (offset {})...",
            "🚀".bold(), port.yellow(), baud_rate, offset.magenta());

        on_progress(5, "Connexion à la puce ESP32-S3 (Bootloader)...");

        // 1. Tenter le flashage en pur Rust via espflash
        match Self::flash_via_espflash(port, addr, &bin_data, baud_rate, &on_progress) {
            Ok(()) => {
                println!("{} Flashage USB terminé avec succès via espflash natif !", "✔".green().bold());
                on_progress(100, "Flashage terminé avec succès !");
                return Ok(());
            }
            Err(e) => {
                println!("{} Échec du flashage natif espflash : {}", "⚠".yellow(), e);

                // 2. Si esptool externe est installé sur le système, tenter comme secours
                if let Some(tool_name) = Self::check_esptool() {
                    let cmd = if tool_name.starts_with("esptool.py") { "esptool.py" } else { "esptool" };
                    println!("{} Tentative de secours via {}...", "ℹ".blue(), cmd);
                    on_progress(10, "Tentative de secours via esptool...");
                    return Self::flash_via_esptool_fallback(cmd, port, bin_path, offset, baud_rate, on_progress);
                }

                Err(anyhow!(
                    "Échec du flashage USB : {}\n\nAstuce : Sur les cartes à USB natif (M5Stamp S3, XIAO ESP32-S3), maintenez le bouton BOOT enfoncé lors du branchement USB pour forcer le mode ROM Bootloader.",
                    e
                ))
            }
        }
    }

    /// Flashage pur Rust via la bibliothèque officielle Espressif espflash
    fn flash_via_espflash<F>(port_name: &str, addr: u32, bin_data: &[u8], baud_rate: u32, on_progress: &F) -> Result<()>
    where
        F: Fn(u32, &str) + Send + Sync,
    {
        use espflash::connection::reset::{ResetAfterOperation, ResetBeforeOperation};
        use espflash::flasher::{Flasher, ProgressCallbacks};
        use espflash::targets::Chip;

        let port_info = serialport::available_ports()
            .ok()
            .and_then(|ports| ports.into_iter().find(|p| p.port_name == port_name))
            .map(|p| match p.port_type {
                serialport::SerialPortType::UsbPort(info) => info,
                _ => UsbPortInfo {
                    vid: 0,
                    pid: 0,
                    serial_number: None,
                    manufacturer: None,
                    product: None,
                },
            })
            .unwrap_or(UsbPortInfo {
                vid: 0,
                pid: 0,
                serial_number: None,
                manufacturer: None,
                product: None,
            });

        let serial_port = serialport::new(port_name, 115_200)
            .flow_control(serialport::FlowControl::None)
            .open_native()
            .map_err(|e| anyhow!("Impossible d'ouvrir le port série {} : {}", port_name, e))?;

        let is_usb_jtag = port_info.pid == 0x1001;
        let before_reset = if is_usb_jtag {
            ResetBeforeOperation::UsbReset
        } else {
            ResetBeforeOperation::DefaultReset
        };

        on_progress(10, "Connexion et synchronisation avec la ROM ESP32-S3...");

        let mut flasher = Flasher::connect(
            serial_port,
            port_info,
            Some(baud_rate),
            true,   // use_stub
            true,   // verify
            false,  // skip
            Some(Chip::Esp32s3),
            ResetAfterOperation::HardReset,
            before_reset,
        ).map_err(|e| anyhow!("Connexion au bootloader impossible ({:?})", e))?;

        on_progress(20, "ESP32-S3 synchronisé. Écriture en mémoire flash...");

        struct ProgressAdapter<'a, CB> {
            callback: &'a CB,
            total: usize,
        }

        impl<'a, CB> ProgressCallbacks for ProgressAdapter<'a, CB>
        where
            CB: Fn(u32, &str) + Send + Sync,
        {
            fn init(&mut self, _addr: u32, total: usize) {
                self.total = total;
                (self.callback)(20, "Préparation de la mémoire flash...");
            }
            fn update(&mut self, current: usize) {
                if self.total > 0 {
                    let pct = ((current as f64 / self.total as f64) * 100.0) as u32;
                    let scaled = 20 + (pct * 78 / 100);
                    let msg = format!("Écriture flash USB : {}%", pct);
                    (self.callback)(scaled.min(98), &msg);
                }
            }
            fn finish(&mut self) {
                (self.callback)(99, "Vérification de l'intégrité MD5... OK");
            }
        }

        let mut adapter = ProgressAdapter {
            callback: on_progress,
            total: bin_data.len(),
        };

        flasher.write_bin_to_flash(addr, bin_data, Some(&mut adapter))
            .map_err(|e| anyhow!("Erreur lors de l'écriture flash : {:?}", e))?;

        Ok(())
    }

    /// Secours via esptool externe si installé
    fn flash_via_esptool_fallback<F>(tool_name: &str, port: &str, bin_path: &Path, offset: &str, baud_rate: u32, on_progress: F) -> Result<()>
    where
        F: Fn(u32, &str) + Send + Sync,
    {
        let pb = ProgressBar::new_spinner();
        pb.set_style(ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")?);
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        pb.set_message("Connexion à l'ESP32-S3 via esptool...");

        let mut child = Command::new(tool_name)
            .args([
                "--chip", "esp32s3",
                "--port", port,
                "--baud", &baud_rate.to_string(),
                "--before", "default_reset",
                "--after", "hard_reset",
                "write_flash",
                "--flash_mode", "dio",
                "--flash_freq", "80m",
                "--flash_size", "4MB",
                offset, bin_path.to_str().unwrap(),
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Échec du lancement d'esptool")?;

        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            for line in reader.lines().flatten() {
                if line.contains("Connecting") {
                    pb.set_message("Connexion à la puce ESP32-S3...");
                    on_progress(10, "Connexion à la puce ESP32-S3...");
                } else if line.contains("Writing at") {
                    if let Some(pct) = line.split('(').nth(1).and_then(|s| s.split('%').next()) {
                        if let Ok(pct_val) = pct.trim().parse::<u32>() {
                            let scaled_pct = 20 + (pct_val * 75 / 100);
                            let msg = format!("Écriture flash USB : {}%", pct_val);
                            pb.set_message(msg.clone());
                            on_progress(scaled_pct.min(98), &msg);
                        }
                    }
                } else if line.contains("Hash of data verified") {
                    pb.set_message("Vérification de l'intégrité MD5... OK");
                    on_progress(99, "Vérification de l'intégrité MD5... OK");
                }
            }
        }

        let status = child.wait()?;
        pb.finish_and_clear();

        if status.success() {
            println!("{} Flashage USB terminé avec succès via esptool !", "✔".green().bold());
            on_progress(100, "Flashage terminé avec succès !");
            Ok(())
        } else {
            Err(anyhow!("Le flashage esptool a échoué (code de sortie {})", status))
        }
    }

    /// Flashe un binaire complet (factory.bin) à l'adresse 0x0000 sur le port sélectionné
    pub fn flash_factory_bin<F>(port: &str, bin_path: &Path, baud_rate: u32, on_progress: F) -> Result<()>
    where
        F: Fn(u32, &str) + Send + Sync,
    {
        Self::flash_usb_bin(port, bin_path, "0x0000", baud_rate, on_progress)
    }
}
