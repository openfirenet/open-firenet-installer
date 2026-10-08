use anyhow::{anyhow, Context, Result};
use colored::*;
use dialoguer::{Input, Password};
use std::io::{Read, Write};
use std::time::{Duration, Instant};

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
        Self::send_credentials(port_name, &ssid, &password, lang)?;
        println!("{} {}", "✔".green().bold(), lang.wifi_sent_success());
        Ok(())
    }

    /// Envoie directement les identifiants Wi-Fi sur le port série
    pub fn send_credentials(port_name: &str, ssid: &str, password: &str, lang: crate::cli_i18n::CliLang) -> Result<()> {
        let mut port = serialport::new(port_name, 115200)
            .timeout(Duration::from_millis(500))
            .open()
            .context(lang.cannot_open_serial())?;

        let cmd = format!("SETWIFI:{}:{}\n", ssid, password);
        port.write_all(cmd.as_bytes())?;
        port.flush()?;
        Ok(())
    }

    /// Sets the password asked for wireless updates, over the serial port; an empty password removes it. The
    /// bridge only accepts this over USB, never over the network.
    pub fn send_ota_password(port_name: &str, password: &str, lang: crate::cli_i18n::CliLang) -> Result<()> {
        let mut port = serialport::new(port_name, 115200)
            .timeout(Duration::from_millis(500))
            .open()
            .context(lang.cannot_open_serial())?;
        // Opening the port restarts many boards (the serial adapter's DTR/RTS lines drive the reset pin), and a
        // command sent while the bridge starts is lost. So the command is sent again until the bridge answers
        // that it has taken it, and nothing is reported as done without that answer.
        let command = format!("SETOTAPASS:{}\n", password);
        let deadline = Instant::now() + Duration::from_secs(15);
        let mut heard = String::new();
        let mut buffer = [0u8; 512];
        while Instant::now() < deadline {
            port.write_all(command.as_bytes())?;
            port.flush()?;
            let listen_until = Instant::now() + Duration::from_millis(1500);
            while Instant::now() < listen_until {
                match port.read(&mut buffer) {
                    Ok(n) if n > 0 => heard.push_str(&String::from_utf8_lossy(&buffer[..n])),
                    Ok(_) => {}
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {}
                    Err(e) => return Err(e.into()),
                }
                if heard.contains("SETOTAPASS OK") {
                    return Ok(());
                }
            }
        }
        Err(anyhow!(lang.ota_password_not_confirmed()))
    }

    /// Moniteur série pour observer les logs du dongle en temps réel
    pub fn monitor_serial(port_name: &str, baud_rate: u32, lang: crate::cli_i18n::CliLang) -> Result<()> {
        println!("\n{}", lang.monitor_opening(port_name, baud_rate));

        let mut port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(100))
            .open()
            .context(lang.cannot_open_serial())?;

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
