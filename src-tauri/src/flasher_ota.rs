use anyhow::{anyhow, Context, Result};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

const EMBEDDED_ESPOTA: &str = include_str!("espota.py");

pub struct OtaFlasher;

impl OtaFlasher {
    /// Extrait ou localise le script espota.py nécessaire à la mise à jour ArduinoOTA
    pub fn get_espota_script() -> Result<std::path::PathBuf> {
        let base = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".to_string());
        let cache_dir = std::path::PathBuf::from(base).join(".cache").join("openfirenet-installer");
        std::fs::create_dir_all(&cache_dir)?;
        let espota_dest = cache_dir.join("espota.py");
        std::fs::write(&espota_dest, EMBEDDED_ESPOTA)?;
        Ok(espota_dest)
    }

    /// Effectue la mise à jour via ArduinoOTA (port TCP 3232) avec rapport d'avancement
    pub fn flash_arduino_ota<F>(ip: &str, bin_path: &Path, on_progress: F) -> Result<()>
    where
        F: Fn(u32, &str) + Send + Sync,
    {
        if !bin_path.exists() {
            return Err(anyhow!("Le fichier binaire n'existe pas : {:?}", bin_path));
        }

        println!("\n{} Préparation de la mise à jour sans fil (ArduinoOTA) vers {}...", "📡".bold(), ip.cyan());
        on_progress(3, "Préparation du firmware pour l'envoi Wi-Fi...");

        let espota_path = Self::get_espota_script()
            .context("Impossible de préparer le script de mise à jour ArduinoOTA")?;

        println!("Utilisation de espota : {}", espota_path.display());
        on_progress(6, &format!("Connexion au poêle ({}:3232)...", ip));

        let pb = ProgressBar::new_spinner();
        pb.set_style(ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")?);
        pb.enable_steady_tick(Duration::from_millis(100));
        pb.set_message("Connexion au poêle via Wi-Fi...");

        let mut child = Command::new("python3")
            .arg(&espota_path)
            .args(["-i", ip, "-p", "3232", "-f", bin_path.to_str().unwrap(), "-r", "-t", "10"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Échec de l'exécution de python3 avec espota.py. Assurez-vous que Python 3 est installé.")?;

        if let Some(mut stderr) = child.stderr.take() {
            let mut buf = [0u8; 128];
            let mut line_buf = String::new();
            while let Ok(n) = stderr.read(&mut buf) {
                if n == 0 { break; }
                let chunk = String::from_utf8_lossy(&buf[..n]);
                for c in chunk.chars() {
                    if c == '\r' || c == '\n' {
                        if line_buf.contains("Uploading:") {
                            if let Some(pct_str) = line_buf.split(']').nth(1).and_then(|s| s.split('%').next()) {
                                if let Ok(pct) = pct_str.trim().parse::<u32>() {
                                    let msg = format!("Transmission Wi-Fi : {}%", pct);
                                    pb.set_message(msg.clone());
                                    on_progress(pct.min(99), &msg);
                                }
                            }
                        }
                        line_buf.clear();
                    } else {
                        line_buf.push(c);
                    }
                }
            }
        }

        let status = child.wait()?;
        pb.finish_and_clear();

        if status.success() {
            println!("{} Mise à jour OTA réussie ! L'ESP32 redémarre.", "✔".green().bold());
            on_progress(100, "Mise à jour OTA réussie ! Le poêle redémarre.");
            Self::wait_for_reboot(ip);
            Ok(())
        } else {
            Err(anyhow!(
                "La mise à jour Wi-Fi (ArduinoOTA) a échoué. Vérifiez que le poêle est allumé et joignable sur {} (port 3232).",
                ip
            ))
        }
    }

    /// Attend que le dongle redémarre et réponde à nouveau sur le réseau
    pub fn wait_for_reboot(ip: &str) {
        println!("Attente de la reconnexion au réseau...");
        thread::sleep(Duration::from_secs(4));

        for _ in 0..20 {
            thread::sleep(Duration::from_secs(1));
            let url = format!("http://{}/api/state", ip);
            if let Ok(resp) = ureq::get(&url).timeout(Duration::from_millis(800)).call() {
                if resp.status() == 200 {
                    println!("{} Open-Firenet est de nouveau en ligne et fonctionnel !", "🎉".bold());
                    return;
                }
            }
        }
        println!("{} Le dongle prend plus de temps à se reconnecter. Vérifiez son adresse IP sur votre box.", "ℹ".yellow());
    }
}
