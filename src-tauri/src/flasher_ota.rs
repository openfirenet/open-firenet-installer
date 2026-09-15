use anyhow::{anyhow, Context, Result};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

pub struct OtaFlasher;

impl OtaFlasher {
    /// Effectue la mise à jour via ArduinoOTA (port TCP 3232)
    pub fn flash_arduino_ota(ip: &str, bin_path: &Path) -> Result<()> {
        if !bin_path.exists() {
            return Err(anyhow!("Le fichier binaire n'existe pas : {:?}", bin_path));
        }

        println!("\n{} Préparation de la mise à jour sans fil (ArduinoOTA) vers {}...", "📡".bold(), ip.cyan());

        // Chercher espota.py dans l'environnement local ou global
        let espota_path = Self::find_espota();

        if let Some(espota) = espota_path {
            println!("Utilisation de espota : {}", espota.display());
            let pb = ProgressBar::new_spinner();
            pb.set_style(ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")?);
            pb.enable_steady_tick(Duration::from_millis(100));
            pb.set_message("Envoi du firmware via Wi-Fi...");

            let status = Command::new("python3")
                .arg(&espota)
                .args(["-i", ip, "-f", bin_path.to_str().unwrap(), "-r"])
                .status()
                .context("Échec de l'exécution de espota.py")?;

            pb.finish_and_clear();

            if status.success() {
                println!("{} Mise à jour OTA réussie ! L'ESP32 redémarre.", "✔".green().bold());
                Self::wait_for_reboot(ip);
                return Ok(());
            } else {
                return Err(anyhow!("espota.py s'est terminé avec une erreur"));
            }
        }

        // Essayer en mode HTTP POST (/update) si disponible
        Self::flash_http_ota(ip, bin_path)
    }

    /// Tente une mise à jour via endpoint HTTP POST /update
    pub fn flash_http_ota(ip: &str, bin_path: &Path) -> Result<()> {
        println!("{} Tentative d'envoi via le serveur Web (HTTP POST /update)...", "🌐".bold());

        let mut file = File::open(bin_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let pb = ProgressBar::new(buffer.len() as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
            .progress_chars("#>-"));

        let url = format!("http://{}/update", ip);
        
        let resp = ureq::post(&url)
            .timeout(Duration::from_secs(60))
            .set("Content-Type", "application/octet-stream")
            .send_bytes(&buffer);

        pb.finish_with_message("Transmission terminée");

        match resp {
            Ok(r) if r.status() == 200 => {
                println!("{} Mise à jour HTTP validée par le poêle ! Redémarrage en cours...", "✔".green().bold());
                Self::wait_for_reboot(ip);
                Ok(())
            }
            Ok(r) => Err(anyhow!("Le poêle a renvoyé le statut HTTP {}", r.status())),
            Err(e) => Err(anyhow!("Erreur réseau lors de la mise à jour OTA : {}", e)),
        }
    }

    /// Attend que le dongle redémarre et réponde à nouveau sur le réseau
    pub fn wait_for_reboot(ip: &str) {
        println!("Attente de la reconnexion au réseau...");
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(200));
        pb.set_message("Redémarrage de la clé Open-Firenet...");

        // Pause initiale de 4 secondes pour laisser le temps de reboot
        thread::sleep(Duration::from_secs(4));

        for _ in 0..20 {
            thread::sleep(Duration::from_secs(1));
            let url = format!("http://{}/api/state", ip);
            if let Ok(resp) = ureq::get(&url).timeout(Duration::from_millis(800)).call() {
                if resp.status() == 200 {
                    pb.finish_and_clear();
                    println!("{} Open-Firenet est de nouveau en ligne et fonctionnel !", "🎉".bold());
                    return;
                }
            }
        }
        pb.finish_and_clear();
        println!("{} Le dongle prend plus de temps à se reconnecter. Vérifiez son adresse IP sur votre box.", "ℹ".yellow());
    }

    fn find_espota() -> Option<std::path::PathBuf> {
        let home = std::env::var("HOME").ok()?;
        let ard_path = std::path::PathBuf::from(home).join(".arduino15").join("packages").join("esp32");
        if ard_path.exists() {
            // Chercher récursivement espota.py
            if let Ok(entries) = std::fs::read_dir(&ard_path) {
                for entry in entries.flatten() {
                    let cand = entry.path().join("tools").join("espota").join("espota.py");
                    if cand.exists() { return Some(cand); }
                }
            }
        }
        None
    }
}
