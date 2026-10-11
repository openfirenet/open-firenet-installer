use tauri::{AppHandle, Emitter, Manager};
use std::path::PathBuf;

pub mod cli_i18n;
pub mod discovery;
pub mod flasher_ota;
pub mod flasher_serial;
pub mod github;
pub mod recorder;
pub mod wifi_setup;

use cli_i18n::CliLang;
use discovery::{DiscoveredDongle, NetworkScanner};
use flasher_serial::{DetectedPort, SerialFlasher};
use github::{GitHubClient, Release};

#[tauri::command]
async fn scan_network(lang: Option<String>) -> Result<Vec<DiscoveredDongle>, String> {
    let lang = CliLang::from_window(lang.as_deref());
    tauri::async_runtime::spawn_blocking(move || {
        let mut devices = NetworkScanner::probe_mdns_hosts(lang);
        if devices.is_empty() {
            devices = NetworkScanner::scan_subnet("192.168.1", lang);
        }
        devices
    })
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_releases(lang: Option<String>) -> Result<Vec<Release>, String> {
    let lang = CliLang::from_window(lang.as_deref());
    tauri::async_runtime::spawn_blocking(move || {
        let client = GitHubClient::new(lang);
        client.list_releases().unwrap_or_default()
    })
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
async fn list_serial_ports(lang: Option<String>) -> Result<Vec<DetectedPort>, String> {
    let lang = CliLang::from_window(lang.as_deref());
    tauri::async_runtime::spawn_blocking(move || {
        SerialFlasher::list_ports(lang).unwrap_or_default()
    })
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
async fn flash_usb_device(
    app: AppHandle,
    port: String,
    release_tag: Option<String>,
    custom_file: Option<String>,
    mode: Option<String>,
    lang: Option<String>,
) -> Result<String, String> {
    let lang = CliLang::from_window(lang.as_deref());
    tauri::async_runtime::spawn_blocking(move || {
        let is_update = mode.as_deref().unwrap_or("update") == "update";
        let offset = if is_update { "0x10000" } else { "0x0000" };

        let bin_path = if let Some(path) = custom_file {
            PathBuf::from(path)
        } else if let Some(tag) = release_tag {
            let client = GitHubClient::new(lang);
            let releases = client.list_releases().map_err(|e| e.to_string())?;
            let release = releases.into_iter().find(|r| r.tag_name == tag)
                .ok_or_else(|| lang.release_not_found(&tag))?;
            let asset = if is_update {
                release.ota_asset()
                    .ok_or_else(|| lang.no_update_binary().to_string())?
            } else {
                release.factory_asset()
                    .ok_or_else(|| lang.no_factory_binary().to_string())?
            };
            let _ = app.emit("flash-status", lang.downloading_verifying());
            let dest = client.download_and_verify_asset(&release, asset).map_err(|e| e.to_string())?;
            dest
        } else {
            return Err(lang.choose_version_or_file().to_string());
        };

        let app_handle = app.clone();
        let _ = app.emit("flash-progress", serde_json::json!({
            "percent": 5,
            "message": lang.usb_flashing_in_progress()
        }));
        SerialFlasher::flash_usb_bin(&port, &bin_path, offset, 921600, lang, move |pct, msg| {
            let _ = app_handle.emit("flash-progress", serde_json::json!({
                "percent": pct,
                "message": msg
            }));
        }).map_err(|e| e.to_string())?;
        let _ = app.emit("flash-status", lang.usb_done());
        Ok(lang.usb_done_restarting().to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
async fn update_ota_device(
    app: AppHandle,
    ip: String,
    release_tag: Option<String>,
    custom_file: Option<String>,
    password: Option<String>,
    lang: Option<String>,
) -> Result<String, String> {
    let lang = CliLang::from_window(lang.as_deref());
    tauri::async_runtime::spawn_blocking(move || {
        let bin_path = if let Some(path) = custom_file {
            PathBuf::from(path)
        } else if let Some(tag) = release_tag {
            let client = GitHubClient::new(lang);
            let releases = client.list_releases().map_err(|e| e.to_string())?;
            let release = releases.into_iter().find(|r| r.tag_name == tag)
                .ok_or_else(|| lang.release_not_found(&tag))?;
            let asset = release.ota_asset()
                .ok_or_else(|| lang.no_update_binary().to_string())?;
            let _ = app.emit("ota-status", lang.downloading_verifying());
            let dest = client.download_and_verify_asset(&release, asset).map_err(|e| e.to_string())?;
            dest
        } else {
            return Err(lang.choose_version_or_file().to_string());
        };

        let app_handle = app.clone();
        let _ = app.emit("ota-progress", serde_json::json!({
            "percent": 5,
            "message": lang.ota_sending_wifi()
        }));
        flasher_ota::OtaFlasher::flash_arduino_ota(&ip, &bin_path, password.as_deref(), lang, move |pct, msg| {
            let _ = app_handle.emit("ota-progress", serde_json::json!({
                "percent": pct,
                "message": msg
            }));
        }).map_err(|e| e.to_string())?;
        let _ = app.emit("ota-status", lang.rebooted_online());
        Ok(lang.ota_success().to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
fn get_app_version(app: tauri::AppHandle) -> String {
    app.package_info().version.to_string()
}

/// A newer published version of the installer, if any. Silent when GitHub cannot be reached: the window must
/// not complain about it at every start without a network.
#[tauri::command]
async fn check_installer_update(app: tauri::AppHandle, lang: Option<String>) -> Option<github::InstallerUpdate> {
    let lang = CliLang::from_window(lang.as_deref());
    let current = app.package_info().version.to_string();
    tauri::async_runtime::spawn_blocking(move || GitHubClient::new(lang).newer_installer(&current).ok().flatten())
        .await
        .ok()
        .flatten()
}

/// The recording in progress, if any: how to stop it and how to hand it a note.
struct Recording {
    stop: std::sync::Arc<std::sync::atomic::AtomicBool>,
    notes: std::sync::mpsc::Sender<String>,
}
static RECORDING: std::sync::Mutex<Option<Recording>> = std::sync::Mutex::new(None);

/// Starts recording a heating cycle in the background and returns the file it writes. Lines go to the window as
/// "record-line" events; "record-done" (changes, file) or "record-error" ends it. The stove is only read.
#[tauri::command]
fn start_recording(app: AppHandle, ip: String, minutes: u64, lang: Option<String>) -> Result<String, String> {
    let lang = CliLang::from_window(lang.as_deref());
    let mut current = RECORDING.lock().map_err(|e| e.to_string())?;
    if current.is_some() {
        return Err(lang.record_already_running().to_string());
    }
    let folder = app.path().document_dir().or_else(|_| app.path().home_dir()).unwrap_or_else(|_| std::path::PathBuf::from("."));
    let path = folder.join(recorder::default_path());
    let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let (send, notes) = std::sync::mpsc::channel::<String>();
    *current = Some(Recording { stop: stop.clone(), notes: send });

    let ip = ip.trim().to_string();
    let file = path.clone();
    std::thread::spawn(move || {
        let result = recorder::record(
            &ip,
            &file,
            std::time::Duration::from_secs(minutes.clamp(1, 24 * 60) * 60),
            std::time::Duration::from_secs(2),
            &notes,
            &|| stop.load(std::sync::atomic::Ordering::Relaxed),
            lang,
            |progress| {
                if let recorder::Progress::Line(line) = progress {
                    if !line.contains(" START ") {
                        let _ = app.emit("record-line", line);
                    }
                }
            },
        );
        if let Ok(mut current) = RECORDING.lock() {
            *current = None;
        }
        match result {
            Ok(changes) => {
                let _ = app.emit("record-done", serde_json::json!({ "changes": changes, "file": file.display().to_string() }));
            }
            Err(e) => {
                let _ = app.emit("record-error", e.to_string());
            }
        }
    });
    Ok(path.display().to_string())
}

/// Adds a note typed by the user to the recording in progress.
#[tauri::command]
fn add_recording_note(note: String) -> bool {
    RECORDING.lock().ok().and_then(|r| r.as_ref().map(|r| r.notes.send(note).is_ok())).unwrap_or(false)
}

/// Asks the recording in progress to end; "record-done" follows within a few seconds.
#[tauri::command]
fn stop_recording() {
    if let Ok(current) = RECORDING.lock() {
        if let Some(recording) = current.as_ref() {
            recording.stop.store(true, std::sync::atomic::Ordering::Relaxed);
        }
    }
}

#[tauri::command]
fn configure_wifi(
    port: String,
    ssid: String,
    password: String,
    lang: Option<String>,
) -> Result<String, String> {
    let lang = CliLang::from_window(lang.as_deref());
    wifi_setup::WifiSetup::send_credentials(&port, &ssid, &password, lang).map_err(|e| e.to_string())?;
    Ok(lang.wifi_config_sent().to_string())
}

/// Whether the bridge at this address asks for an update password; `None` when it could not be asked. The window
/// shows its password field on `Some(true)` and on `None`.
#[tauri::command(async)]
fn ota_password_needed(ip: String) -> Option<bool> {
    flasher_ota::OtaFlasher::password_state(ip.trim())
}

/// Sets (or, with an empty text, removes) the password asked for wireless updates, over the serial port.
#[tauri::command]
fn set_ota_password(port: String, password: String, lang: Option<String>) -> Result<String, String> {
    let lang = CliLang::from_window(lang.as_deref());
    wifi_setup::WifiSetup::send_ota_password(&port, &password, lang).map_err(|e| e.to_string())?;
    Ok(lang.ota_password_sent(password.is_empty()).to_string())
}

/// Opens an address with the system's default program. The window used `cmd /C start <url>`: `start` takes a quoted
/// first argument for a window title, `&` in an address splits the command, and the window has no console since it
/// detaches from it at start. The shell's own call has none of these traps (issue #28).
#[cfg(target_os = "windows")]
fn windows_open(url: &str) -> bool {
    use std::os::windows::ffi::OsStrExt;
    #[link(name = "shell32")]
    extern "system" {
        fn ShellExecuteW(
            hwnd: *mut std::ffi::c_void,
            operation: *const u16,
            file: *const u16,
            parameters: *const u16,
            directory: *const u16,
            show: i32,
        ) -> isize;
    }
    let wide = |text: &str| std::ffi::OsStr::new(text).encode_wide().chain(std::iter::once(0)).collect::<Vec<u16>>();
    let (operation, file) = (wide("open"), wide(url));
    // SW_SHOWNORMAL = 1; a value above 32 means success.
    unsafe { ShellExecuteW(std::ptr::null_mut(), operation.as_ptr(), file.as_ptr(), std::ptr::null(), std::ptr::null(), 1) > 32 }
}

/// Only web addresses are opened: the window passes addresses it built, and this keeps it so.
fn is_web_address(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

#[tauri::command]
fn open_browser_url(url: String, lang: Option<String>) -> Result<(), String> {
    let lang = CliLang::from_window(lang.as_deref());
    if !is_web_address(&url) {
        return Err(lang.cannot_open_browser(&url));
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&url)
            .spawn()
            .map_err(|e| lang.cannot_open_browser(&e.to_string()))?;
    }
    #[cfg(target_os = "windows")]
    {
        if !windows_open(&url) {
            return Err(lang.cannot_open_browser("ShellExecute"));
        }
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&url)
            .spawn()
            .map_err(|e| lang.cannot_open_browser(&e.to_string()))?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            scan_network,
            get_releases,
            list_serial_ports,
            flash_usb_device,
            update_ota_device,
            configure_wifi,
            set_ota_password,
            ota_password_needed,
            open_browser_url,
            get_app_version,
            check_installer_update,
            start_recording,
            add_recording_note,
            stop_recording
        ])
        .run(tauri::generate_context!())
        .expect("failed to start the window");
}
