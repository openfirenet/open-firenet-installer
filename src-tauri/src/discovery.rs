use serde::{Deserialize, Serialize};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::cli_i18n::CliLang;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredDongle {
    pub ip: String,
    pub hostname: String,
    pub firmware_version: String,
    pub stove_model: String,
    pub stove_state: String,
    pub wifi_rssi: String,
}

#[derive(Deserialize)]
struct ApiDevice {
    version: Option<String>,
    app_version: Option<String>,
    firmware_version: Option<String>,
    wifi_rssi: Option<i32>,
}

#[derive(Deserialize)]
struct ApiVersion {
    version: Option<String>,
}

#[derive(Deserialize)]
struct ApiStove {
    model_name: Option<String>,
    main_state: Option<i32>,
    state_code: Option<i32>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ApiState {
    device: Option<ApiDevice>,
    stove: Option<ApiStove>,
    model_name: Option<String>,
    model: Option<i32>,
    firmware_version: Option<String>,
    version: Option<String>,
}

pub struct NetworkScanner;

impl NetworkScanner {
    /// Tente de contacter Open Firenet via les noms mDNS par défaut
    pub fn probe_mdns_hosts(lang: CliLang) -> Vec<DiscoveredDongle> {
        let mut results = Vec::new();
        // The bridge announces itself as "open-firenet"; the name without a dash is the one of very old firmwares.
        let candidates = ["open-firenet.local:80", "openfirenet.local:80"];

        for target in &candidates {
            if let Ok(mut addrs) = target.to_socket_addrs() {
                if let Some(addr) = addrs.next() {
                    let ip = addr.ip().to_string();
                    if let Some(dongle) = Self::probe_ip(&ip, lang) {
                        results.push(dongle);
                    }
                }
            }
        }
        results
    }

    /// Sonde une adresse IP spécifique pour vérifier si c'est un dongle Open Firenet
    pub fn probe_ip(ip: &str, lang: CliLang) -> Option<DiscoveredDongle> {
        let socket_addr: SocketAddr = format!("{}:80", ip).parse().ok()?;
        if TcpStream::connect_timeout(&socket_addr, Duration::from_millis(1000)).is_err() {
            return None;
        }

        let url = format!("http://{}/api/state", ip);
        let resp = ureq::get(&url)
            .timeout(Duration::from_millis(1500))
            .call()
            .ok()?;

        if resp.status() == 200 {
            if let Ok(state) = resp.into_json::<ApiState>() {
                let mut fw_ver = state.device.as_ref()
                    .and_then(|d| d.version.clone().or_else(|| d.app_version.clone()).or_else(|| d.firmware_version.clone()))
                    .or_else(|| state.firmware_version.clone())
                    .or_else(|| state.version.clone());

                if fw_ver.is_none() {
                    let ver_url = format!("http://{}/api/version", ip);
                    if let Ok(ver_resp) = ureq::get(&ver_url).timeout(Duration::from_millis(800)).call() {
                        if let Ok(api_ver) = ver_resp.into_json::<ApiVersion>() {
                            fw_ver = api_ver.version;
                        }
                    }
                }

                let fw_ver_str = fw_ver.unwrap_or_else(|| lang.version_unknown().to_string());
                
                let model = state.model_name
                    .or_else(|| state.stove.as_ref().and_then(|s| s.model_name.clone()))
                    .unwrap_or_else(|| "RIKA".to_string());

                let st = state.stove.as_ref()
                    .and_then(|s| s.main_state.or(s.state_code))
                    .map(|code| lang.stove_state(code as i64).to_string())
                    .unwrap_or_else(|| "--".to_string());

                let rssi = state.device.as_ref()
                    .and_then(|d| d.wifi_rssi)
                    .map(|r| format!("{} dBm", r))
                    .unwrap_or_else(|| "--".to_string());

                return Some(DiscoveredDongle {
                    ip: ip.to_string(),
                    hostname: "open-firenet.local".to_string(),
                    firmware_version: fw_ver_str,
                    stove_model: model,
                    stove_state: st,
                    wifi_rssi: rssi,
                });
            }
        }
        None
    }

    /// Effectue un scan rapide en parallèle sur le sous-réseau local (ex: 192.168.1.1..254)
    /// Utilise un pool fixe de 24 workers pour éviter l'épuisement de threads sous Windows.
    pub fn scan_subnet(base_prefix: &str, lang: CliLang) -> Vec<DiscoveredDongle> {
        let results = Arc::new(Mutex::new(Vec::new()));
        let (tx, rx) = std::sync::mpsc::channel();

        for i in 1..=254 {
            let _ = tx.send(format!("{}.{}", base_prefix, i));
        }
        drop(tx);

        let rx = Arc::new(Mutex::new(rx));
        let num_workers = 24;
        let mut workers = Vec::with_capacity(num_workers);

        crate::journal::note(format!("scanning subnet {}.1 - {}.254", base_prefix, base_prefix));

        for _ in 0..num_workers {
            let rx = Arc::clone(&rx);
            let results = Arc::clone(&results);
            workers.push(thread::spawn(move || {
                loop {
                    let ip = match rx.lock().ok().and_then(|r| r.recv().ok()) {
                        Some(ip) => ip,
                        None => break,
                    };
                    let socket_addr: SocketAddr = match format!("{}:80", ip).parse() {
                        Ok(a) => a,
                        Err(_) => continue,
                    };
                    if TcpStream::connect_timeout(&socket_addr, Duration::from_millis(750)).is_ok() {
                        if let Some(dongle) = NetworkScanner::probe_ip(&ip, lang) {
                            if let Ok(mut r) = results.lock() {
                                r.push(dongle);
                            }
                        }
                    }
                }
            }));
        }

        for w in workers {
            let _ = w.join();
        }

        let res = results.lock().unwrap().clone();
        res
    }

    /// Tente de détecter les préfixes réseaux locaux courants (ex: 192.168.1)
    pub fn detect_local_prefixes() -> Vec<String> {
        let mut prefixes = Vec::new();
        if let Ok(socket) = std::net::UdpSocket::bind("0.0.0.0:0") {
            if socket.connect("8.8.8.8:80").is_ok() {
                if let Ok(addr) = socket.local_addr() {
                    if let std::net::IpAddr::V4(ipv4) = addr.ip() {
                        let octets = ipv4.octets();
                        if octets[0] != 127 {
                            let prefix = format!("{}.{}.{}", octets[0], octets[1], octets[2]);
                            prefixes.push(prefix);
                        }
                    }
                }
            }
        }
        for fallback in &["192.168.1", "192.168.0", "192.168.4"] {
            if !prefixes.iter().any(|p| p == fallback) {
                prefixes.push(fallback.to_string());
            }
        }
        prefixes
    }
}
