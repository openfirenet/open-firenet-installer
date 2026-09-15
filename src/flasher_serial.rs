use anyhow::{anyhow, Context, Result};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use serialport::{SerialPortType, UsbPortInfo};
use std::path::Path;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
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

    /// Flashe un binaire complet (factory.bin) à l'adresse 0x0 sur le port sélectionné
    pub fn flash_factory_bin(port: &str, bin_path: &Path, baud_rate: u32) -> Result<()> {
        if !bin_path.exists() {
            return Err(anyhow!("Le fichier binaire n'existe pas : {:?}", bin_path));
        }

        let esptool_cmd = Self::check_esptool()
            .ok_or_else(|| anyhow!("esptool n'a pas été trouvé. Veuillez installer esptool (pip install esptool)"))?;

        let tool_name = if esptool_cmd.starts_with("esptool.py") { "esptool.py" } else { "esptool" };

        println!("\n{} Début du flashage via {} sur {} à {} bauds...",
            "🚀".bold(), tool_name.cyan(), port.yellow(), baud_rate);

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
                "0x0000", bin_path.to_str().unwrap(),
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
                } else if line.contains("Writing at") {
                    if let Some(pct) = line.split('(').nth(1).and_then(|s| s.split('%').next()) {
                        pb.set_message(format!("Écriture de la mémoire flash : {}%", pct.trim()));
                    }
                } else if line.contains("Hash of data verified") {
                    pb.set_message("Vérification de l'intégrité MD5... OK");
                }
            }
        }

        let status = child.wait()?;
        pb.finish_and_clear();

        if status.success() {
            println!("{} Flashage USB terminé avec succès ! L'ESP32-S3 redémarre.", "✔".green().bold());
            Ok(())
        } else {
            Err(anyhow!("Le flashage a échoué (code de sortie {})", status))
        }
    }
}
