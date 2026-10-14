use anyhow::{anyhow, Context, Result};
use colored::*;
use dialoguer::{Input, Password};
use std::io::{Read, Write};
use std::time::{Duration, Instant};

/// Whether an I/O error indicates that the serial device detached or rebooted.
///
/// On Windows, when an ESP32-S3 with native USB CDC ACM resets, the virtual COM port disappears,
/// causing in-flight and subsequent I/O to fail with `ERROR_BAD_COMMAND` (os error 22),
/// `ERROR_NO_SUCH_DEVICE` (os error 433), `ERROR_DEV_NOT_EXIST` (os error 55),
/// `ERROR_NOT_FOUND` (os error 1168), `ERROR_OPERATION_ABORTED` (os error 995),
/// `ERROR_DEVICE_NOT_CONNECTED` (os error 1167), `ERROR_INVALID_HANDLE` (os error 6),
/// or `ERROR_GEN_FAILURE` (os error 31).
/// On Unix, the kernel reports `ENODEV` (19), `EIO` (5), `BrokenPipe`, or `ConnectionReset`.
fn is_disconnect_error(err: &std::io::Error) -> bool {
    match err.kind() {
        std::io::ErrorKind::BrokenPipe
        | std::io::ErrorKind::ConnectionReset
        | std::io::ErrorKind::ConnectionAborted
        | std::io::ErrorKind::NotConnected
        | std::io::ErrorKind::UnexpectedEof => return true,
        _ => {}
    }
    match err.raw_os_error() {
        Some(6) | Some(22) | Some(31) | Some(55) | Some(433) | Some(995) | Some(1167) | Some(1168) => true,
        #[cfg(unix)]
        Some(5) | Some(19) => true,
        _ => false,
    }
}

/// What became of a command sent over the serial port.
#[derive(Debug, PartialEq, Eq)]
pub enum SendOutcome {
    /// The bridge answered that it took the command.
    Confirmed,
    /// The port went away before any answer, on a board that is an Open Firenet key: a key up to firmware 4.3.0
    /// restarts right after saving, so the command was probably taken, but nothing proves it.
    PortLost,
    /// No answer within the time allowed.
    NoAnswer,
}

/// Whether the serial port is the USB port of an Open Firenet key (the USB identity set by its firmware).
fn is_open_firenet_port(port_name: &str) -> bool {
    use serialport::{SerialPortType, UsbPortInfo};
    serialport::available_ports()
        .map(|ports| {
            ports.iter().any(|p| {
                p.port_name == port_name
                    && matches!(&p.port_type, SerialPortType::UsbPort(UsbPortInfo { vid: 0x303a, pid: 0x819a, .. }))
            })
        })
        .unwrap_or(false)
}

pub struct WifiSetup;

impl WifiSetup {
    /// Prompts the user for Wi-Fi credentials in the terminal and sends them to the dongle.
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
        let confirmed = Self::send_credentials(port_name, &ssid, &password, lang)?;
        let done = if confirmed { lang.wifi_sent_success() } else { lang.wifi_probably_saved() };
        println!("{} {}", "✔".green().bold(), done);
        Ok(())
    }

    /// Sends a command over the serial port until the bridge answers that it has taken it, and tells what became of it.
    ///
    /// Opening the port restarts many boards (the serial adapter's DTR/RTS lines drive the reset pin), and a command
    /// sent while the bridge starts is lost. So the command is sent again, for 15 seconds at most, and nothing is
    /// reported as done without the bridge's answer. A port that goes away without answering is told apart
    /// (`PortLost`), and only on a board recognised as an Open Firenet key.
    fn send_until_confirmed(port_name: &str, command: &str, confirmation: &str, secrets: &[&str], lang: crate::cli_i18n::CliLang) -> Result<SendOutcome> {
        use crate::journal;
        let shown = journal::mask(command.trim_end(), secrets);
        journal::note(format!("serial {}: open at 115200 baud, to send \"{}\" and wait for \"{}\"", port_name, shown, confirmation));
        let known_key = is_open_firenet_port(port_name);
        journal::note(format!("serial {}: recognised as an Open Firenet key: {}", port_name, known_key));
        let mut port = journal::io(
            &format!("open {}", port_name),
            serialport::new(port_name, 115200)
                .flow_control(serialport::FlowControl::None)
                .timeout(Duration::from_millis(500))
                .open()
                .map_err(std::io::Error::from),
        )
        .context(lang.cannot_open_serial())?;
        // The USB serial port of the key only sends something to the host while the host raises DTR (the line state
        // of its USB CDC). Linux raises it when a port is opened, Windows does not through this library: without it,
        // the bridge's answer never arrives and the command looks lost even though the bridge took it.
        journal::io(&format!("raise DTR {}", port_name), port.write_data_terminal_ready(true).map_err(std::io::Error::from)).ok();
        let deadline = Instant::now() + Duration::from_secs(15);
        let mut heard = String::new();
        let mut buffer = [0u8; 512];
        let mut attempt = 0;
        while Instant::now() < deadline {
            attempt += 1;
            journal::note(format!("serial {}: attempt {}, write {} bytes", port_name, attempt, command.len()));
            if let Err(e) = journal::io(&format!("write {}", port_name), port.write_all(command.as_bytes())) {
                if heard.contains(confirmation) {
                    journal::note(format!("serial {}: confirmation was already received", port_name));
                    return Ok(SendOutcome::Confirmed);
                }
                if attempt > 1 && is_disconnect_error(&e) {
                    journal::note(format!("serial {}: board disconnected/rebooted after earlier attempt: {}", port_name, e));
                    return Ok(if known_key { SendOutcome::PortLost } else { SendOutcome::NoAnswer });
                }
                return Err(e.into());
            }
            if let Err(e) = journal::io(&format!("flush {}", port_name), port.flush()) {
                if heard.contains(confirmation) {
                    journal::note(format!("serial {}: confirmation was already received", port_name));
                    return Ok(SendOutcome::Confirmed);
                }
                if is_disconnect_error(&e) {
                    journal::note(format!("serial {}: board disconnected/rebooted during flush: {}", port_name, e));
                    return Ok(if known_key { SendOutcome::PortLost } else { SendOutcome::NoAnswer });
                }
                return Err(e.into());
            }
            let listen_until = Instant::now() + Duration::from_millis(1500);
            while Instant::now() < listen_until {
                match journal::io(&format!("read {}", port_name), port.read(&mut buffer)) {
                    Ok(n) if n > 0 => {
                        journal::note(format!("serial {}: received \"{}\"", port_name, journal::mask(&journal::printable(&buffer[..n]), secrets)));
                        heard.push_str(&String::from_utf8_lossy(&buffer[..n]));
                        if heard.contains(confirmation) {
                            journal::note(format!("serial {}: confirmation received", port_name));
                            return Ok(SendOutcome::Confirmed);
                        }
                    }
                    Ok(_) => {}
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {}
                    Err(ref e) if is_disconnect_error(e) => {
                        if heard.contains(confirmation) {
                            journal::note(format!("serial {}: confirmation received before disconnect", port_name));
                            return Ok(SendOutcome::Confirmed);
                        }
                        // On ESP32 boards with native USB, rebooting right after accepting the command drops the USB connection.
                        journal::note(format!("serial {}: board disconnected/rebooted during read: {}", port_name, e));
                        return Ok(if known_key { SendOutcome::PortLost } else { SendOutcome::NoAnswer });
                    }
                    Err(e) => return Err(e.into()),
                }
            }
        }
        journal::note(format!("serial {}: no confirmation after {} attempts in 15 s", port_name, attempt));
        Ok(SendOutcome::NoAnswer)
    }

    /// Sends the Wi-Fi name and password over the serial port; the bridge saves them.
    /// Tells whether the bridge confirmed it (`true`) or the port went away without an answer (`false`, "probably saved").
    pub fn send_credentials(port_name: &str, ssid: &str, password: &str, lang: crate::cli_i18n::CliLang) -> Result<bool> {
        let command = format!("\nSETWIFI:{}:{}\n", ssid, password);
        match Self::send_until_confirmed(port_name, &command, "SETWIFI OK", &[ssid, password], lang)? {
            SendOutcome::Confirmed => Ok(true),
            SendOutcome::PortLost => Ok(false),
            SendOutcome::NoAnswer => Err(anyhow!(lang.wifi_not_confirmed())),
        }
    }

    /// Sets the password asked for wireless updates, over the serial port; an empty password removes it. The
    /// bridge only accepts this over USB, never over the network.
    /// Same answer as `send_credentials`: `true` when confirmed, `false` when the port went away without one.
    pub fn send_ota_password(port_name: &str, password: &str, lang: crate::cli_i18n::CliLang) -> Result<bool> {
        let command = format!("\nSETOTAPASS:{}\n", password);
        match Self::send_until_confirmed(port_name, &command, "SETOTAPASS OK", &[password], lang)? {
            SendOutcome::Confirmed => Ok(true),
            SendOutcome::PortLost => Ok(false),
            SendOutcome::NoAnswer => Err(anyhow!(lang.ota_password_not_confirmed())),
        }
    }

    /// Opens the serial monitor to read logs in real time.
    pub fn monitor_serial(port_name: &str, baud_rate: u32, lang: crate::cli_i18n::CliLang) -> Result<()> {
        println!("\n{}", lang.monitor_opening(port_name, baud_rate));

        let mut port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(100))
            .open()
            .context(lang.cannot_open_serial())?;
        let _ = port.write_data_terminal_ready(true);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disconnect_errors_are_detected() {
        assert!(is_disconnect_error(&std::io::Error::from_raw_os_error(22)));
        assert!(is_disconnect_error(&std::io::Error::from_raw_os_error(433)));
        assert!(is_disconnect_error(&std::io::Error::from_raw_os_error(995)));
        assert!(is_disconnect_error(&std::io::Error::from_raw_os_error(1167)));
        assert!(is_disconnect_error(&std::io::Error::from_raw_os_error(6)));
        assert!(is_disconnect_error(&std::io::Error::from_raw_os_error(31)));
        assert!(is_disconnect_error(&std::io::Error::from_raw_os_error(55)));
        assert!(is_disconnect_error(&std::io::Error::from_raw_os_error(1168)));
        assert!(is_disconnect_error(&std::io::Error::new(std::io::ErrorKind::BrokenPipe, "broken pipe")));
        assert!(is_disconnect_error(&std::io::Error::new(std::io::ErrorKind::ConnectionReset, "connection reset")));
        assert!(!is_disconnect_error(&std::io::Error::new(std::io::ErrorKind::TimedOut, "timeout")));
        assert!(!is_disconnect_error(&std::io::Error::new(std::io::ErrorKind::NotFound, "not found")));
    }
}
