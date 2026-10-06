use anyhow::{anyhow, Context, Result};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use md5::{Digest, Md5};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, UdpSocket};
use std::path::Path;
use std::thread;
use std::time::Duration;

use crate::cli_i18n::CliLang;

pub struct OtaFlasher;

impl OtaFlasher {
    /// Update slot of Arduino's default 4 MB partition scheme (the releases use "Minimal SPIFFS", 1,966,080 bytes).
    pub const DEFAULT_SCHEME_SLOT_BYTES: usize = 1_310_720;

    /// Size of the stick's update slot, as reported by its firmware (`device.ota_slot_bytes`), when it reports one.
    fn ota_slot_bytes(ip: &str) -> Option<usize> {
        let url = format!("http://{}/api/state", ip);
        let resp = ureq::get(&url).timeout(Duration::from_millis(1500)).call().ok()?;
        let state: serde_json::Value = resp.into_json().ok()?;
        let slot = state.get("device")?.get("ota_slot_bytes")?.as_u64()?;
        if slot == 0 { None } else { Some(slot as usize) }
    }

    /// Effectue la mise à jour sans fil via le protocole ArduinoOTA en 100% Rust natif
    /// (UDP 3232 pour l'invitation + TCP local pour le streaming du firmware)
    pub fn flash_arduino_ota<F>(ip: &str, bin_path: &Path, lang: CliLang, on_progress: F) -> Result<()>
    where
        F: Fn(u32, &str) + Send + Sync,
    {
        if !bin_path.exists() {
            return Err(anyhow!(lang.file_not_found(bin_path)));
        }

        println!("\n{} {}", "📡".bold(), lang.ota_preparing(ip));
        on_progress(2, lang.reading_firmware());

        let bin_data = std::fs::read(bin_path)
            .with_context(|| lang.cannot_read_firmware(bin_path))?;
        let content_size = bin_data.len();

        let mut hasher = Md5::new();
        hasher.update(&bin_data);
        let file_md5 = hex::encode(hasher.finalize());

        println!("{}", lang.firmware_info(content_size, &file_md5));

        // A stick first flashed with Arduino's default partition scheme has update slots of 1,310,720 bytes: a
        // larger firmware is refused by the stick without a word. Newer firmware reports the slot size.
        if let Some(slot) = Self::ota_slot_bytes(ip) {
            if content_size > slot {
                return Err(anyhow!(lang.ota_slot_too_small(content_size, slot)));
            }
        }

        // 1. Ouvrir le serveur TCP local sur un port éphémère libre
        let tcp_listener = TcpListener::bind("0.0.0.0:0")
            .context(lang.ota_tcp_bind_failed())?;
        let local_port = tcp_listener.local_addr()?.port();
        println!("{}", lang.ota_tcp_listening(local_port));

        // 2. Préparer la socket UDP pour la négociation d'invitation
        let udp_socket = UdpSocket::bind("0.0.0.0:0")
            .context(lang.ota_udp_bind_failed())?;
        udp_socket.set_read_timeout(Some(Duration::from_millis(1500)))?;

        let esp_udp_addr: SocketAddr = format!("{}:3232", ip)
            .parse()
            .with_context(|| lang.invalid_ip(ip))?;

        // 3. Protocole ArduinoOTA : commande 0 (FLASH), port local, taille, MD5
        let invite_msg = format!("0 {} {} {}\n", local_port, content_size, file_md5);

        on_progress(5, &lang.ota_negotiating(ip));
        let pb = ProgressBar::new_spinner();
        pb.set_style(ProgressStyle::default_spinner().template("{spinner:.green} {msg}")?);
        pb.enable_steady_tick(Duration::from_millis(100));
        pb.set_message(lang.ota_connecting());

        let mut ack_ok = false;
        let mut rx_buf = [0u8; 128];

        for attempt in 1..=8 {
            let _ = udp_socket.send_to(invite_msg.as_bytes(), esp_udp_addr);
            match udp_socket.recv_from(&mut rx_buf) {
                Ok((n, _from)) => {
                    let resp = String::from_utf8_lossy(&rx_buf[..n]);
                    if resp.contains("OK") {
                        ack_ok = true;
                        break;
                    } else if resp.contains("AUTH") {
                        return Err(anyhow!(lang.ota_auth_required()));
                    }
                }
                Err(_) => {
                    on_progress(5 + attempt, &lang.ota_waiting_reply(attempt, 8));
                }
            }
        }

        if !ack_ok {
            return Err(anyhow!(lang.ota_no_reply(ip)));
        }

        pb.set_message(lang.ota_waiting_tcp());
        on_progress(15, lang.ota_connected_init());

        // 4. Accepter la connexion TCP de l'ESP32 (l'ESP32 se connecte en retour sur notre port TCP)
        tcp_listener.set_nonblocking(true)?;
        let start_connect = std::time::Instant::now();
        let mut client_stream = None;

        while start_connect.elapsed() < Duration::from_secs(12) {
            match tcp_listener.accept() {
                Ok((stream, peer)) => {
                    println!("{}", lang.ota_tcp_received(&peer.to_string()));
                    client_stream = Some(stream);
                    break;
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(50));
                }
                Err(e) => return Err(anyhow!(lang.ota_tcp_accept_error(&e.to_string()))),
            }
        }

        // An older firmware does not report its slot size: above Arduino's default slot, name that cause too.
        let default_slot = Some(Self::DEFAULT_SCHEME_SLOT_BYTES).filter(|slot| content_size > *slot);
        let mut stream = client_stream
            .ok_or_else(|| anyhow!(lang.ota_no_tcp_connection(local_port, content_size, default_slot)))?;

        stream.set_nonblocking(false)?;
        stream.set_read_timeout(Some(Duration::from_secs(15)))?;
        stream.set_write_timeout(Some(Duration::from_secs(15)))?;

        // 5. Envoi des blocs de firmware (blocs de 1024 octets pour fluidité réseau)
        let chunk_size = 1024;
        let mut offset = 0;
        let mut ack_buf = [0u8; 32];
        let mut last_pct = 0;

        pb.set_message(lang.ota_sending());

        while offset < content_size {
            let end = (offset + chunk_size).min(content_size);
            let chunk = &bin_data[offset..end];
            stream.write_all(chunk)
                .context(lang.ota_send_error())?;
            offset = end;

            // Lire l'acquittement de l'ESP32 pour respecter le contrôle de flux TCP
            let _ = stream.read(&mut ack_buf);

            let pct = 15 + ((offset as f64 / content_size as f64) * 83.0) as u32;
            if pct != last_pct {
                last_pct = pct;
                let msg = lang.ota_progress(((offset as f64 / content_size as f64) * 100.0) as u32);
                pb.set_message(msg.clone());
                on_progress(pct, &msg);
            }
        }

        // 6. Validation finale par l'ESP32
        pb.set_message(lang.ota_verifying());
        on_progress(99, lang.ota_verifying());

        // Attente de l'acquittement final OK (l'ESP32 valide la flash et le MD5)
        stream.set_read_timeout(Some(Duration::from_secs(8)))?;
        let mut final_buf = [0u8; 64];

        for _ in 0..10 {
            match stream.read(&mut final_buf) {
                Ok(n) if n > 0 => {
                    let s = String::from_utf8_lossy(&final_buf[..n]);
                    if s.contains("OK") {
                        break;
                    }
                }
                _ => break,
            }
        }

        pb.finish_with_message(lang.ota_done());

        // 7. Attente de reconnexion
        let reboot_ok = Self::wait_for_reboot(ip, lang, &on_progress);
        if reboot_ok {
            on_progress(100, lang.rebooted_online());
        }
        Ok(())
    }

    /// Attend que le dongle redémarre et réponde à nouveau sur le réseau
    pub fn wait_for_reboot<F>(ip: &str, lang: CliLang, on_progress: F) -> bool
    where
        F: Fn(u32, &str) + Send + Sync,
    {
        println!("{}", lang.rebooting_waiting());
        on_progress(100, lang.rebooting_waiting());
        thread::sleep(Duration::from_secs(4));

        for attempt in 1..=20 {
            thread::sleep(Duration::from_secs(1));
            on_progress(100, &lang.waiting_reconnect(attempt, 20));
            let url = format!("http://{}/api/state", ip);
            if let Ok(resp) = ureq::get(&url).timeout(Duration::from_millis(800)).call() {
                if resp.status() == 200 {
                    println!("{}", lang.rebooted_online().green());
                    on_progress(100, lang.rebooted_online());
                    return true;
                }
            }
        }
        println!("{} {}", "ℹ".yellow(), lang.reconnect_slow());
        on_progress(100, lang.reconnect_slow());
        false
    }
}
