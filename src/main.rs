mod discovery;
mod flasher_ota;
mod flasher_serial;
mod github;
mod wifi_setup;

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use dialoguer::{Confirm, Select};
use std::path::PathBuf;

use discovery::NetworkScanner;
use flasher_ota::OtaFlasher;
use flasher_serial::SerialFlasher;
use github::GitHubClient;
use wifi_setup::WifiSetup;

#[derive(Parser)]
#[command(
    name = "open-firenet-installer",
    author = "Open-Firenet Community",
    version = "0.1.0",
    about = "Outil universel d'installation, flashage USB et mise à jour OTA pour Open-Firenet"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Scanner le réseau local pour détecter la clé Open-Firenet
    Scan {
        #[arg(short, long)]
        subnet: Option<String>,
    },
    /// Flasher la clé en USB série (ROM bootloader)
    Flash {
        #[arg(short, long)]
        port: Option<String>,
        #[arg(short, long)]
        file: Option<PathBuf>,
        #[arg(short, long)]
        release: Option<String>,
    },
    /// Mettre à jour la clé via Wi-Fi (OTA)
    Ota {
        #[arg(short, long)]
        ip: String,
        #[arg(short, long)]
        file: Option<PathBuf>,
        #[arg(short, long)]
        release: Option<String>,
    },
    /// Configurer les identifiants Wi-Fi via le port USB
    WifiSetup {
        #[arg(short, long)]
        port: Option<String>,
    },
    /// Lister les versions officielles sur GitHub
    ListReleases,
    /// Ouvrir le moniteur série
    Monitor {
        #[arg(short, long)]
        port: Option<String>,
        #[arg(short, long, default_value = "115200")]
        baud: u32,
    },
}

fn print_banner() {
    println!("{}", "╔═══════════════════════════════════════════════════════════════╗".cyan().bold());
    println!("{}", "║                 🔥  OPEN-FIRENET INSTALLER  🔥                ║".cyan().bold());
    println!("{}", "║      Assistant multiplateforme de flash USB et mise à jour    ║".cyan().bold());
    println!("{}", "╚═══════════════════════════════════════════════════════════════╝".cyan().bold());
    println!();
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Scan { subnet }) => cmd_scan(subnet)?,
        Some(Commands::Flash { port, file, release }) => cmd_flash(port, file, release)?,
        Some(Commands::Ota { ip, file, release }) => cmd_ota(&ip, file, release)?,
        Some(Commands::WifiSetup { port }) => cmd_wifi_setup(port)?,
        Some(Commands::ListReleases) => cmd_list_releases()?,
        Some(Commands::Monitor { port, baud }) => cmd_monitor(port, baud)?,
        None => run_interactive_menu()?,
    }

    Ok(())
}

fn run_interactive_menu() -> Result<()> {
    loop {
        print_banner();

        let choices = &[
            "🔍 1. Scanner le réseau local (détecter la clé & état du poêle)",
            "⚡ 2. Flasher la clé en USB (premier flash / réinstallation)",
            "📡 3. Mettre à jour la clé à distance via Wi-Fi (OTA)",
            "📶 4. Configurer le Wi-Fi de la clé (via USB Série)",
            "📦 5. Consulter les versions GitHub (releases & pré-releases)",
            "📟 6. Moniteur Série (voir les logs du poêle en direct)",
            "🚪 7. Quitter",
        ];

        let selection = Select::new()
            .with_prompt("Que souhaitez-vous faire ?")
            .items(choices)
            .default(0)
            .interact()?;

        println!();

        match selection {
            0 => cmd_scan(None)?,
            1 => cmd_flash(None, None, None)?,
            2 => interactive_ota()?,
            3 => cmd_wifi_setup(None)?,
            4 => cmd_list_releases()?,
            5 => cmd_monitor(None, 115200)?,
            6 => {
                println!("Au revoir !");
                break;
            }
            _ => unreachable!(),
        }

        println!("\n{}", "─".repeat(60).dimmed());
        if !Confirm::new().with_prompt("Revenir au menu principal ?").default(true).interact()? {
            break;
        }
        println!();
    }
    Ok(())
}

fn cmd_scan(subnet: Option<String>) -> Result<()> {
    println!("{}", "🔍 Recherche de la clé Open-Firenet sur votre réseau...".bold());

    // 1. Essai mDNS
    let mut found = NetworkScanner::probe_mdns_hosts();

    // 2. Essai subnet si non trouvé ou demandé
    if found.is_empty() {
        let prefixes = if let Some(s) = subnet {
            vec![s]
        } else {
            NetworkScanner::detect_local_prefixes()
        };

        for prefix in prefixes {
            let res = NetworkScanner::scan_subnet(&prefix);
            found.extend(res);
            if !found.is_empty() { break; }
        }
    }

    if found.is_empty() {
        println!("{}", "❌ Aucune clé Open-Firenet détectée sur le réseau.".red().bold());
        println!("Conseils :");
        println!("  - Vérifiez que la clé est bien allumée et connectée au Wi-Fi.");
        println!("  - Si c'est un premier démarrage, connectez-vous au point d'accès Wi-Fi 'OpenFirenet-Setup'.");
        println!("  - Ou branchez-la en USB pour effectuer le premier flashage.");
    } else {
        println!("{}", format!("✔ {} clé(s) Open-Firenet trouvée(s) :", found.len()).green().bold());
        for (i, d) in found.iter().enumerate() {
            println!("\n  [{}] Adresse IP     : {}", i + 1, d.ip.cyan().bold());
            println!("      Nom d'hôte     : {}", d.hostname.dimmed());
            println!("      Modèle poêle   : {}", d.stove_model.yellow().bold());
            println!("      État actuel    : {}", d.stove_state.white());
            println!("      Signal Wi-Fi   : {}", d.wifi_rssi);
            println!("      Version firmw. : {}", d.firmware_version);
            println!("      Accès Web      : http://{}/", d.ip);
        }
    }
    Ok(())
}

fn cmd_flash(port: Option<String>, file: Option<PathBuf>, release: Option<String>) -> Result<()> {
    println!("{}", "⚡ Flashage USB série de la clé Open-Firenet".bold());

    let selected_port = choose_serial_port(port)?;
    let bin_path = if let Some(f) = file {
        f
    } else {
        choose_or_download_firmware(true, release)?
    };

    SerialFlasher::flash_factory_bin(&selected_port, &bin_path, 460800)?;
    Ok(())
}

fn cmd_ota(ip: &str, file: Option<PathBuf>, release: Option<String>) -> Result<()> {
    println!("{} Mise à jour Wi-Fi (OTA) vers {}", "📡".bold(), ip.cyan());

    let bin_path = if let Some(f) = file {
        f
    } else {
        choose_or_download_firmware(false, release)?
    };

    OtaFlasher::flash_arduino_ota(ip, &bin_path)?;
    Ok(())
}

fn cmd_wifi_setup(port: Option<String>) -> Result<()> {
    let selected_port = choose_serial_port(port)?;
    WifiSetup::prompt_and_configure(&selected_port)
}

fn choose_serial_port(explicit: Option<String>) -> Result<String> {
    if let Some(p) = explicit {
        return Ok(p);
    }
    let ports = SerialFlasher::list_ports()?;
    if ports.is_empty() {
        anyhow::bail!("❌ Aucun port série USB détecté. Branchez votre ESP32-S3 en USB.");
    }

    let port_items: Vec<String> = ports.iter()
        .map(|p| format!("{} - {}", p.port_name.cyan(), p.description))
        .collect();

    let idx = Select::new()
        .with_prompt("Sélectionnez le port série USB de votre ESP32-S3 :")
        .items(&port_items)
        .default(0)
        .interact()?;

    Ok(ports[idx].port_name.clone())
}

fn interactive_ota() -> Result<()> {
    let mut found = NetworkScanner::probe_mdns_hosts();
    if found.is_empty() {
        for prefix in NetworkScanner::detect_local_prefixes() {
            found.extend(NetworkScanner::scan_subnet(&prefix));
            if !found.is_empty() { break; }
        }
    }

    let ip = if !found.is_empty() {
        let items: Vec<String> = found.iter()
            .map(|d| format!("{} (Poêle {}, Version {})", d.ip, d.stove_model, d.firmware_version))
            .collect();
        let idx = Select::new()
            .with_prompt("Sélectionnez la clé à mettre à jour :")
            .items(&items)
            .default(0)
            .interact()?;
        found[idx].ip.clone()
    } else {
        dialoguer::Input::new()
            .with_prompt("Entrez l'adresse IP de votre clé Open-Firenet")
            .interact_text()?
    };

    cmd_ota(&ip, None, None)
}

fn choose_or_download_firmware(factory: bool, release_tag: Option<String>) -> Result<PathBuf> {
    let gh = GitHubClient::new();
    let cache_dir = GitHubClient::cache_dir();
    std::fs::create_dir_all(&cache_dir)?;

    let rel = if let Some(tag) = release_tag {
        let releases = gh.list_releases()?;
        releases.into_iter().find(|r| r.tag_name == tag)
            .ok_or_else(|| anyhow::anyhow!("Version {} introuvable sur GitHub", tag))?
    } else {
        println!("{}", "Récupération des versions disponibles sur GitHub...".dimmed());
        let releases = gh.list_releases().unwrap_or_default();
        if releases.is_empty() {
            println!("{}", "Aucune release publiée sur GitHub pour le moment.".yellow());
            let path_str: String = dialoguer::Input::new()
                .with_prompt("Chemin vers votre fichier binaire .bin local")
                .interact_text()?;
            return Ok(PathBuf::from(path_str));
        }

        let items: Vec<String> = releases.iter().map(|r| {
            let label = if r.prerelease { " (pré-release/test)" } else { " (stable)" };
            format!("{} - {}{}", r.tag_name, r.name.as_deref().unwrap_or(""), label)
        }).collect();

        let idx = Select::new()
            .with_prompt("Choisissez la version à installer :")
            .items(&items)
            .default(0)
            .interact()?;
        releases[idx].clone()
    };

    let asset = if factory {
        rel.factory_asset()
    } else {
        rel.ota_asset()
    };

    let asset = match asset {
        Some(a) => a,
        None => {
            println!("{}", "Aucun binaire précompilé approprié trouvé dans cette release.".yellow());
            let path_str: String = dialoguer::Input::new()
                .with_prompt("Entrez le chemin vers votre fichier .bin local")
                .interact_text()?;
            return Ok(PathBuf::from(path_str));
        }
    };

    let dest = cache_dir.join(&asset.name);
    if dest.exists() {
        if Confirm::new().with_prompt(format!("Utiliser la version en cache ({}) ?", asset.name)).default(true).interact()? {
            return Ok(dest);
        }
    }

    println!("Téléchargement de {}...", asset.name.cyan());
    gh.download_file(&asset.browser_download_url, &dest)?;
    Ok(dest)
}

fn cmd_list_releases() -> Result<()> {
    let gh = GitHubClient::new();
    println!("{}", "📦 Versions d'Open-Firenet publiées sur GitHub :".bold());

    match gh.list_releases() {
        Ok(releases) if releases.is_empty() => {
            println!("{}", "Aucune release officielle n'est encore publiée sur GitHub.".yellow());
        }
        Ok(releases) => {
            for r in releases {
                let badge = if r.prerelease { "[BÊTA/TEST]".yellow() } else { "[STABLE]".green() };
                println!("\n  {} {} - {}", badge, r.tag_name.bold(), r.name.unwrap_or_default());
                for a in r.assets {
                    let size_mb = a.size as f64 / (1024.0 * 1024.0);
                    println!("    • {} ({:.2} Mo)", a.name.dimmed(), size_mb);
                }
            }
        }
        Err(e) => {
            println!("{} Impossible de contacter l'API GitHub : {}", "❌".red(), e);
        }
    }
    Ok(())
}

fn cmd_monitor(port: Option<String>, baud: u32) -> Result<()> {
    let selected_port = choose_serial_port(port)?;
    WifiSetup::monitor_serial(&selected_port, baud)
}
