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

    /// Vérifie si l'outil esptool est disponible dans le système
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

    /// Flashe un binaire USB avec l'adresse mémoire offset spécifiée (0x0000 pour factory, 0x10000 pour app update)
    pub fn flash_usb_bin<F>(port: &str, bin_path: &Path, offset: &str, baud_rate: u32, on_progress: F) -> Result<()>
    where
        F: Fn(u32, &str) + Send + Sync,
    {
        if !bin_path.exists() {
            return Err(anyhow!("Le fichier binaire n'existe pas : {:?}", bin_path));
        }

        let esptool_cmd = Self::check_esptool()
            .ok_or_else(|| anyhow!("esptool n'a pas été trouvé. Veuillez installer esptool (pip install esptool)"))?;

        let tool_name = if esptool_cmd.starts_with("esptool.py") { "esptool.py" } else { "esptool" };

        println!("\n{} Début du flashage via {} sur {} à {} bauds (offset {})...",
            "🚀".bold(), tool_name.cyan(), port.yellow(), baud_rate, offset.magenta());

        on_progress(5, "Connexion à la puce ESP32-S3 (Bootloader)...");

        let pb = ProgressBar::new_spinner();
        pb.set_style(ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")?);
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        pb.set_message("Connexion à l'ESP32-S3 en cours (Reset en mode bootloader)...");

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
                } else if line.contains("Chip is ESP32-S3") {
                    pb.set_message("Puce ESP32-S3 identifiée.");
                    on_progress(15, "Puce ESP32-S3 identifiée.");
                } else if line.contains("Erasing flash") {
                    pb.set_message("Effacement de la mémoire flash...");
                    on_progress(20, "Effacement de la mémoire flash...");
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
            println!("{} Flashage USB terminé avec succès ! La clé redémarre.", "✔".green().bold());
            on_progress(100, "Flashage terminé avec succès ! La clé redémarre.");
            Ok(())
        } else {
            Err(anyhow!("Le flashage a échoué (code de sortie {})", status))
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
