use anyhow::{anyhow, Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

const GITHUB_REPO: &str = "openfirenet/open-firenet";
const USER_AGENT: &str = "OpenFirenet-Installer/0.1.0";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseAsset {
    pub name: String,
    pub browser_download_url: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Release {
    pub tag_name: String,
    pub name: Option<String>,
    pub body: Option<String>,
    pub prerelease: bool,
    pub assets: Vec<ReleaseAsset>,
}

impl Release {
    /// Trouve l'asset factory complet pour flash USB
    pub fn factory_asset(&self) -> Option<&ReleaseAsset> {
        self.assets.iter().find(|a| a.name.contains("factory") && a.name.ends_with(".bin"))
            .or_else(|| self.assets.iter().find(|a| a.name.contains("merged") && a.name.ends_with(".bin")))
    }

    /// Trouve l'asset OTA pour mise à jour sans fil
    pub fn ota_asset(&self) -> Option<&ReleaseAsset> {
        self.assets.iter().find(|a| a.name.contains("ota") && a.name.ends_with(".bin"))
            .or_else(|| self.assets.iter().find(|a| a.name.ends_with(".bin") && !a.name.contains("factory")))
    }

    #[allow(dead_code)]
    pub fn sha256_asset(&self) -> Option<&ReleaseAsset> {
        self.assets.iter().find(|a| a.name.to_lowercase().contains("sha256"))
    }
}

pub struct GitHubClient {
    agent: ureq::Agent,
}

impl GitHubClient {
    pub fn new() -> Self {
        Self {
            agent: ureq::builder()
                .user_agent(USER_AGENT)
                .timeout(std::time::Duration::from_secs(15))
                .build(),
        }
    }

    /// Récupère la liste des releases officielles sur GitHub
    pub fn list_releases(&self) -> Result<Vec<Release>> {
        let url = format!("https://api.github.com/repos/{}/releases", GITHUB_REPO);
        let resp = self.agent.get(&url)
            .set("Accept", "application/vnd.github.v3+json")
            .call()
            .context("Échec de la requête vers l'API GitHub")?;

        let releases: Vec<Release> = resp.into_json()
            .context("Impossible de décoder les releases GitHub")?;
        Ok(releases)
    }

    #[allow(dead_code)]
    pub fn get_latest_release(&self) -> Result<Release> {
        let releases = self.list_releases()?;
        releases.into_iter().find(|r| !r.prerelease)
            .ok_or_else(|| anyhow!("Aucune release stable trouvée"))
    }

    #[allow(dead_code)]
    pub fn list_branches(&self) -> Result<Vec<String>> {
        let url = format!("https://api.github.com/repos/{}/branches", GITHUB_REPO);
        let resp = self.agent.get(&url)
            .set("Accept", "application/vnd.github.v3+json")
            .call()
            .context("Échec de la récupération des branches GitHub")?;

        #[derive(Deserialize)]
        struct Branch { name: String }
        let branches: Vec<Branch> = resp.into_json()
            .context("Impossible de parser les branches GitHub")?;
        Ok(branches.into_iter().map(|b| b.name).collect())
    }

    /// Télécharge un asset avec affichage d'une barre de progression
    pub fn download_file(&self, url: &str, dest_path: &Path) -> Result<()> {
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let resp = self.agent.get(url).call()
            .map_err(|e| anyhow!("Erreur lors du téléchargement de {} : {}", url, e))?;

        let total_size = resp.header("Content-Length")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
            .progress_chars("#>-"));

        let mut reader = resp.into_reader();
        let mut file = File::create(dest_path)
            .context("Impossible de créer le fichier de destination")?;

        let mut buffer = [0u8; 8192];
        let mut downloaded: u64 = 0;

        loop {
            let n = reader.read(&mut buffer)?;
            if n == 0 { break; }
            file.write_all(&buffer[..n])?;
            downloaded += n as u64;
            pb.set_position(downloaded);
        }

        pb.finish_with_message("Téléchargement terminé");
        Ok(())
    }

    /// Dossier local de cache des firmwares
    pub fn cache_dir() -> PathBuf {
        let base = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".to_string());
        PathBuf::from(base).join(".cache").join("openfirenet-installer").join("firmware")
    }

    #[allow(dead_code)]
    pub fn compute_sha256(path: &Path) -> Result<String> {
        let mut file = File::open(path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 8192];
        loop {
            let n = file.read(&mut buffer)?;
            if n == 0 { break; }
            hasher.update(&buffer[..n]);
        }
        Ok(hex::encode(hasher.finalize()))
    }
}
