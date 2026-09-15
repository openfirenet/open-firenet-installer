use tauri::{AppHandle, Emitter};
use std::path::PathBuf;

pub mod discovery;
pub mod flasher_ota;
pub mod flasher_serial;
pub mod github;
pub mod wifi_setup;

use discovery::{DiscoveredDongle, NetworkScanner};
use flasher_serial::{DetectedPort, SerialFlasher};
use github::{GitHubClient, Release};

#[tauri::command]
fn scan_network() -> Result<Vec<DiscoveredDongle>, String> {
    // 1. First probe mDNS
    let mut devices = NetworkScanner::probe_mdns_hosts();
    if devices.is_empty() {
        // 2. Fast subnet scan (default 192.168.1)
        devices = NetworkScanner::scan_subnet("192.168.1");
    }
    Ok(devices)
}

#[tauri::command]
fn get_releases() -> Result<Vec<Release>, String> {
    let client = GitHubClient::new();
    client.list_releases().map_err(|e| e.to_string())
}

#[tauri::command]
fn list_serial_ports() -> Result<Vec<DetectedPort>, String> {
    SerialFlasher::list_ports().map_err(|e| e.to_string())
}

#[tauri::command]
fn flash_usb_device(
    app: AppHandle,
    port: String,
    release_tag: Option<String>,
    custom_file: Option<String>,
) -> Result<String, String> {
    let bin_path = if let Some(path) = custom_file {
        PathBuf::from(path)
    } else if let Some(tag) = release_tag {
        let client = GitHubClient::new();
        let releases = client.list_releases().map_err(|e| e.to_string())?;
        let release = releases.into_iter().find(|r| r.tag_name == tag)
            .ok_or_else(|| format!("Release {} introuvable", tag))?;
        let asset = release.factory_asset()
            .ok_or_else(|| "Aucun binaire factory trouvé pour cette release".to_string())?;
        let dest = GitHubClient::cache_dir().join(&asset.name);
        client.download_file(&asset.browser_download_url, &dest).map_err(|e| e.to_string())?;
        dest
    } else {
        return Err("Veuillez choisir une version ou un fichier local".to_string());
    };

    let _ = app.emit("flash-status", "Flashage en cours sur le port USB...");
    SerialFlasher::flash_factory_bin(&port, &bin_path, 921600).map_err(|e| e.to_string())?;
    let _ = app.emit("flash-status", "Flashage terminé avec succès !");
    Ok("Flashage terminé avec succès !".to_string())
}

#[tauri::command]
fn update_ota_device(
    app: AppHandle,
    ip: String,
    release_tag: Option<String>,
    custom_file: Option<String>,
) -> Result<String, String> {
    let bin_path = if let Some(path) = custom_file {
        PathBuf::from(path)
    } else if let Some(tag) = release_tag {
        let client = GitHubClient::new();
        let releases = client.list_releases().map_err(|e| e.to_string())?;
        let release = releases.into_iter().find(|r| r.tag_name == tag)
            .ok_or_else(|| format!("Release {} introuvable", tag))?;
        let asset = release.ota_asset()
            .ok_or_else(|| "Aucun binaire OTA trouvé pour cette release".to_string())?;
        let dest = GitHubClient::cache_dir().join(&asset.name);
        client.download_file(&asset.browser_download_url, &dest).map_err(|e| e.to_string())?;
        dest
    } else {
        return Err("Veuillez choisir une version ou un fichier local".to_string());
    };

    let _ = app.emit("ota-status", "Envoi du firmware via Wi-Fi...");
    flasher_ota::OtaFlasher::flash_arduino_ota(&ip, &bin_path).map_err(|e| e.to_string())?;
    let _ = app.emit("ota-status", "Mise à jour OTA réussie !");
    Ok("Mise à jour réussie ! Le poêle redémarre.".to_string())
}

#[tauri::command]
fn configure_wifi(
    port: String,
    ssid: String,
    password: String,
) -> Result<String, String> {
    wifi_setup::WifiSetup::send_credentials(&port, &ssid, &password).map_err(|e| e.to_string())?;
    Ok("Configuration Wi-Fi envoyée avec succès !".to_string())
}

#[tauri::command]
fn open_browser_url(url: String) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("Impossible d'ouvrir le navigateur : {}", e))?;
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", &url])
            .spawn()
            .map_err(|e| format!("Impossible d'ouvrir le navigateur : {}", e))?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("Impossible d'ouvrir le navigateur : {}", e))?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            scan_network,
            get_releases,
            list_serial_ports,
            flash_usb_device,
            update_ota_device,
            configure_wifi,
            open_browser_url
        ])
        .run(tauri::generate_context!())
        .expect("erreur lors du lancement de l'interface graphique");
}
