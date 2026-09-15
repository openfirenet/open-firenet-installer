use anyhow::{Context, Result};
use colored::*;
use dialoguer::{Input, Password};
use std::io::{Read, Write};
use std::time::Duration;

pub struct WifiSetup;

impl WifiSetup {
    /// Guide l'utilisateur pour configurer le Wi-Fi
    pub fn prompt_and_configure(port_name: &str) -> Result<()> {
        println!("\n{}", "📶 Configuration Wi-Fi du dongle Open-Firenet".cyan().bold());
        println!("Ces informations permettront à la clé de se connecter à votre réseau local.");

        let ssid: String = Input::new()
            .with_prompt("Nom de votre réseau Wi-Fi (SSID)")
            .interact_text()?;

        let password = Password::new()
            .with_prompt("Mot de passe Wi-Fi (WPA2/WPA3)")
            .interact()?;

        println!("\nEnvoi de la configuration sur {}...", port_name.yellow());

        let mut port = serialport::new(port_name, 115200)
            .timeout(Duration::from_millis(500))
            .open()
            .context("Impossible d'ouvrir le port série")?;

        // Envoi d'une trame de configuration standard NVS ou commande console
        let cmd = format!("WIFI_SET:ssid={};pass={};\n", ssid, password);
        port.write_all(cmd.as_bytes())?;
        port.flush()?;

        println!("{} Commande transmise. Le dongle va tenter de se connecter.", "✔".green().bold());
        Ok(())
    }

    /// Moniteur série pour observer les logs du dongle en temps réel
    pub fn monitor_serial(port_name: &str, baud_rate: u32) -> Result<()> {
        println!("\n{} Ouverture du moniteur série sur {} à {} bauds (Ctrl+C pour quitter)...",
            "📟".bold(), port_name.yellow(), baud_rate);

        let mut port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(100))
            .open()
            .context("Impossible d'ouvrir le port série")?;

        let mut buffer = [0u8; 1024];
        loop {
            match port.read(&mut buffer) {
                Ok(n) if n > 0 => {
                    let s = String::from_utf8_lossy(&buffer[..n]);
                    print!("{}", s);
                    let _ = std::io::stdout().flush();
                }
                Ok(_) => {}
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {}
                Err(e) => {
                    println!("\n{} Fin de session série : {}", "ℹ".yellow(), e);
                    break;
                }
            }
        }
        Ok(())
    }
}
