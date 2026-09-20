use anyhow::{Context, Result};
use colored::*;
use dialoguer::{Input, Password};
use std::io::{Read, Write};
use std::time::Duration;

pub struct WifiSetup;

impl WifiSetup {
    /// Guide l'utilisateur pour configurer le Wi-Fi
    pub fn prompt_and_configure(port_name: &str, lang: crate::cli_i18n::CliLang) -> Result<()> {
        println!("\n{}", lang.wifi_setup_title().cyan().bold());
        println!("{}", lang.wifi_setup_desc());
        println!("{}\n", lang.wifi_native_hint().dimmed());

        let ssid: String = Input::new()
            .with_prompt(lang.wifi_ssid_prompt())
            .interact_text()?;

        let password = Password::new()
            .with_prompt(lang.wifi_pass_prompt())
            .interact()?;

        println!("\n{}", lang.wifi_sending(port_name).yellow());
        Self::send_credentials(port_name, &ssid, &password)?;
        println!("{} {}", "✔".green().bold(), lang.wifi_sent_success());
        Ok(())
    }

    /// Envoie directement les identifiants Wi-Fi sur le port série
    pub fn send_credentials(port_name: &str, ssid: &str, password: &str) -> Result<()> {
        let mut port = serialport::new(port_name, 115200)
            .timeout(Duration::from_millis(500))
            .open()
            .context("Impossible d'ouvrir le port série")?;

        let cmd = format!("SETWIFI:{}:{}\n", ssid, password);
        port.write_all(cmd.as_bytes())?;
        port.flush()?;
        Ok(())
    }

    /// Moniteur série pour observer les logs du dongle en temps réel
    pub fn monitor_serial(port_name: &str, baud_rate: u32, lang: crate::cli_i18n::CliLang) -> Result<()> {
        println!("\n{}", lang.monitor_opening(port_name, baud_rate));

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
                    println!("\n{} {} {}", "ℹ".yellow(), lang.monitor_session_end(), e);
                    break;
                }
            }
        }
        Ok(())
    }
}
