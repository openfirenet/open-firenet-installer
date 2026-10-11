use anyhow::{anyhow, Context, Result};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use serialport::{SerialPortType, UsbPortInfo};
use std::path::Path;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

use crate::cli_i18n::CliLang;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedPort {
    pub port_name: String,
    pub description: String,
    pub is_esp: bool,
    /// The port is the one a running Open Firenet firmware presents (the port that goes to the stove): the board
    /// cannot be read nor flashed through it unless it was started with its BOOT button held.
    #[serde(default)]
    pub is_open_firenet: bool,
    /// The port is the chip's own USB (Espressif's vendor id, not a running Open Firenet): it is an ESP32 for sure,
    /// so it can be read without asking. A generic serial adapter can hide any device: reading it restarts that
    /// device, which is only done when the user asks.
    #[serde(default)]
    pub is_espressif: bool,
}

pub struct SerialFlasher;

impl SerialFlasher {
    /// Liste tous les ports séries disponibles et met en évidence les puces Espressif
    pub fn list_ports(lang: CliLang) -> Result<Vec<DetectedPort>> {
        let ports = serialport::available_ports()
            .context(lang.cannot_list_ports())?;

        let mut results = Vec::new();

        for p in ports {
            let mut desc = lang.port_generic().to_string();
            let mut is_esp = false;
            let mut is_open_firenet = false;
            let mut is_espressif = false;

            if let SerialPortType::UsbPort(UsbPortInfo { vid, pid, product, manufacturer, .. }) = &p.port_type {
                let prod = product.as_deref().unwrap_or(lang.unknown());
                let mfg = manufacturer.as_deref().unwrap_or(lang.unknown());
                desc = format!("USB: {} ({}) [VID: {:04x}, PID: {:04x}]", prod, mfg, vid, pid);

                // 0x303a = Espressif Systems
                if *vid == 0x303a && *pid == 0x819a {
                    // The USB identity of the Open Firenet firmware itself (USB.PID in the firmware).
                    is_esp = true;
                    is_open_firenet = true;
                    desc = lang.port_open_firenet().to_string();
                } else if *vid == 0x303a {
                    is_esp = true;
                    is_espressif = true;
                    desc = format!("🔥 ESP32-S3 ({})", prod);
                } else if *vid == 0x1a86 || *vid == 0x10c4 || *vid == 0x0403 {
                    // Adaptateurs USB-Série classiques (CH340, CP2102, FTDI)
                    is_esp = true;
                    desc = lang.port_serial_adapter(prod);
                }
            }

            results.push(DetectedPort {
                port_name: p.port_name,
                description: desc,
                is_esp,
                is_open_firenet,
                is_espressif,
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
    pub fn flash_usb_bin<F>(port: &str, bin_path: &Path, offset: &str, baud_rate: u32, lang: CliLang, on_progress: F) -> Result<()>
    where
        F: Fn(u32, &str) + Send + Sync,
    {
        if !bin_path.exists() {
            return Err(anyhow!(lang.file_not_found(bin_path)));
        }

        let addr = if offset.starts_with("0x") || offset.starts_with("0X") {
            u32::from_str_radix(&offset[2..], 16)
                .with_context(|| lang.invalid_offset(offset))?
        } else {
            offset.parse::<u32>()
                .with_context(|| lang.invalid_offset(offset))?
        };

        let bin_data = std::fs::read(bin_path)
            .with_context(|| lang.cannot_read_firmware(bin_path))?;

        println!("\n{} {}", "🚀".bold(), lang.usb_flash_start(port, baud_rate, offset));

        on_progress(5, lang.usb_connecting_chip());

        // 1. Tenter le flashage en pur Rust via espflash
        match Self::flash_via_espflash(port, addr, &bin_data, baud_rate, lang, &on_progress) {
            Ok(()) => {
                println!("{} {}", "✔".green().bold(), lang.usb_done());
                on_progress(100, lang.usb_done());
                return Ok(());
            }
            Err(e) => {
                println!("{} {}", "⚠".yellow(), lang.usb_native_failed(&e.to_string()));

                // 2. Si esptool externe est installé sur le système, tenter comme secours
                if let Some(tool_name) = Self::check_esptool() {
                    let cmd = if tool_name.starts_with("esptool.py") { "esptool.py" } else { "esptool" };
                    println!("{} {}", "ℹ".blue(), lang.usb_fallback(cmd));
                    on_progress(10, &lang.usb_fallback(cmd));
                    return Self::flash_via_esptool_fallback(cmd, port, bin_path, offset, baud_rate, lang, on_progress);
                }

                Err(anyhow!(lang.usb_flash_failed(&e.to_string())))
            }
        }
    }

    /// Flashage pur Rust via la bibliothèque officielle Espressif espflash
    fn flash_via_espflash<F>(port_name: &str, addr: u32, bin_data: &[u8], baud_rate: u32, lang: CliLang, on_progress: &F) -> Result<()>
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
            .map_err(|e| anyhow!(lang.cannot_open_port(port_name, &e.to_string())))?;

        let is_usb_jtag = port_info.pid == 0x1001;
        let before_reset = if is_usb_jtag {
            ResetBeforeOperation::UsbReset
        } else {
            ResetBeforeOperation::DefaultReset
        };

        on_progress(10, lang.usb_syncing());

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
        ).map_err(|e| anyhow!(lang.bootloader_connect_failed(&format!("{:?}", e))))?;

        on_progress(20, lang.usb_synced_writing());

        struct ProgressAdapter<'a, CB> {
            callback: &'a CB,
            total: usize,
            lang: CliLang,
        }

        impl<'a, CB> ProgressCallbacks for ProgressAdapter<'a, CB>
        where
            CB: Fn(u32, &str) + Send + Sync,
        {
            fn init(&mut self, _addr: u32, total: usize) {
                self.total = total;
                (self.callback)(20, self.lang.usb_preparing_flash());
            }
            fn update(&mut self, current: usize) {
                if self.total > 0 {
                    let pct = ((current as f64 / self.total as f64) * 100.0) as u32;
                    let scaled = 20 + (pct * 78 / 100);
                    let msg = self.lang.usb_write_progress(pct);
                    (self.callback)(scaled.min(98), &msg);
                }
            }
            fn finish(&mut self) {
                (self.callback)(99, self.lang.integrity_ok());
            }
        }

        let mut adapter = ProgressAdapter {
            callback: on_progress,
            total: bin_data.len(),
            lang,
        };

        flasher.write_bin_to_flash(addr, bin_data, Some(&mut adapter))
            .map_err(|e| anyhow!(lang.usb_write_error(&format!("{:?}", e))))?;

        if is_usb_jtag {
            // On a new connection: run right after the write, on the connection that did it, the same sequence
            // left the board in flashing mode (tried on an ESP32-S3 flashed with BOOT held).
            drop(flasher);
            Self::leave_flashing_mode(port_name);
        }
        Ok(())
    }

    /// Restarts an ESP32-S3 flashed through its own USB port into the firmware just written.
    ///
    /// A board put into flashing mode by holding its BOOT button keeps that mode across an ordinary reset: after
    /// the flash it sat there until it was unplugged. This is what Espressif's esptool does for this chip
    /// (targets/esp32s3.py, hard_reset and watchdog_reset): clear the "forced download" bit, and when the boot pin
    /// was low at start, restart through the RTC watchdog, which reads that pin again; otherwise an ordinary reset
    /// is enough. Errors are ignored: at worst the board stays in flashing mode, as it did before.
    fn leave_flashing_mode(port_name: &str) {
        use espflash::connection::reset::{ResetAfterOperation, ResetBeforeOperation};
        use espflash::flasher::Flasher;

        const RTC_CNTL_OPTION1_REG: u32 = 0x6000_812C; // bit 0: download mode forced over USB
        const GPIO_STRAP_REG: u32 = 0x6000_4038; // bit 3 clear: the boot pin was low at start
        const RTC_CNTL_WDTCONFIG0_REG: u32 = 0x6000_8098;
        const RTC_CNTL_WDTCONFIG1_REG: u32 = 0x6000_809C;
        const RTC_CNTL_WDTWPROTECT_REG: u32 = 0x6000_80B0;
        const RTC_CNTL_WDT_WKEY: u32 = 0x50D8_3AA1;

        let Some(port_info) = serialport::available_ports().ok().and_then(|ports| {
            ports.into_iter().find(|p| p.port_name == port_name).and_then(|p| match p.port_type {
                SerialPortType::UsbPort(info) => Some(info),
                _ => None,
            })
        }) else {
            return;
        };
        let Ok(serial) = serialport::new(port_name, 115_200).flow_control(serialport::FlowControl::None).open_native() else {
            return;
        };
        let Ok(mut flasher) = Flasher::connect(
            serial,
            port_info,
            Some(115_200),
            true,
            false,
            false,
            None,
            ResetAfterOperation::NoReset,
            ResetBeforeOperation::UsbReset,
        ) else {
            return;
        };
        // The register addresses below are those of the ESP32-S3: nothing is written to another chip.
        if flasher.chip() != espflash::targets::Chip::Esp32s3 {
            return;
        }
        let connection = flasher.connection();
        let forced = connection.read_reg(RTC_CNTL_OPTION1_REG).map(|v| v & 0x1 != 0).unwrap_or(false);
        let _ = connection.write_reg(RTC_CNTL_OPTION1_REG, 0, Some(0x1));
        let boot_pin_was_low = connection.read_reg(GPIO_STRAP_REG).map(|strap| strap & (1 << 3) == 0).unwrap_or(false);
        if boot_pin_was_low && !forced {
            let _ = connection.write_reg(RTC_CNTL_WDTWPROTECT_REG, RTC_CNTL_WDT_WKEY, None); // unlock
            let _ = connection.write_reg(RTC_CNTL_WDTCONFIG1_REG, 2000, None); // timeout
            let _ = connection.write_reg(RTC_CNTL_WDTCONFIG0_REG, (1 << 31) | (5 << 28) | (1 << 8) | 2, None); // enable
            let _ = connection.write_reg(RTC_CNTL_WDTWPROTECT_REG, 0, None); // lock
            std::thread::sleep(std::time::Duration::from_millis(800));
        } else {
            let _ = connection.reset();
        }
    }

    /// Secours via esptool externe si installé
    fn flash_via_esptool_fallback<F>(tool_name: &str, port: &str, bin_path: &Path, offset: &str, baud_rate: u32, lang: CliLang, on_progress: F) -> Result<()>
    where
        F: Fn(u32, &str) + Send + Sync,
    {
        let pb = ProgressBar::new_spinner();
        pb.set_style(ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")?);
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        pb.set_message(lang.usb_connecting_chip());

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
            .context(lang.esptool_launch_failed())?;

        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            for line in reader.lines().flatten() {
                if line.contains("Connecting") {
                    pb.set_message(lang.usb_connecting_chip());
                    on_progress(10, lang.usb_connecting_chip());
                } else if line.contains("Writing at") {
                    if let Some(pct) = line.split('(').nth(1).and_then(|s| s.split('%').next()) {
                        if let Ok(pct_val) = pct.trim().parse::<u32>() {
                            let scaled_pct = 20 + (pct_val * 75 / 100);
                            let msg = lang.usb_write_progress(pct_val);
                            pb.set_message(msg.clone());
                            on_progress(scaled_pct.min(98), &msg);
                        }
                    }
                } else if line.contains("Hash of data verified") {
                    pb.set_message(lang.integrity_ok());
                    on_progress(99, lang.integrity_ok());
                }
            }
        }

        let status = child.wait()?;
        pb.finish_and_clear();

        if status.success() {
            println!("{} {}", "✔".green().bold(), lang.usb_done());
            on_progress(100, lang.usb_done());
            Ok(())
        } else {
            Err(anyhow!(lang.esptool_failed(&status.to_string())))
        }
    }

    /// Flashe un binaire complet (factory.bin) à l'adresse 0x0000 sur le port sélectionné
    pub fn flash_factory_bin<F>(port: &str, bin_path: &Path, baud_rate: u32, lang: CliLang, on_progress: F) -> Result<()>
    where
        F: Fn(u32, &str) + Send + Sync,
    {
        Self::flash_usb_bin(port, bin_path, "0x0000", baud_rate, lang, on_progress)
    }
}
