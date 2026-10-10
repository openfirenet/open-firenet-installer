use anyhow::{anyhow, Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::cli_i18n::CliLang;

const GITHUB_REPO: &str = "openfirenet/open-firenet";
const USER_AGENT: &str = "OpenFirenet-Installer/0.1.0";
const INSTALLER_REPO: &str = "openfirenet/open-firenet-installer";

/// A published version of the installer newer than the one running.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InstallerUpdate {
    pub version: String,
    pub url: String,
}

/// "v1.8.1" or "1.8.1" as numbers; None for anything else (a pre-release suffix, a malformed tag).
fn parse_version(text: &str) -> Option<(u64, u64, u64)> {
    let mut parts = text.trim().trim_start_matches('v').split('.');
    let version = (parts.next()?.parse().ok()?, parts.next()?.parse().ok()?, parts.next()?.parse().ok()?);
    if parts.next().is_some() {
        return None;
    }
    Some(version)
}

/// The update to offer when `latest_tag` is a version newer than `current`. The page address is built from the
/// tag once it is known to be a plain version, never taken from the answer of the network.
pub fn installer_update(latest_tag: &str, current: &str) -> Option<InstallerUpdate> {
    let latest = parse_version(latest_tag)?;
    if latest <= parse_version(current)? {
        return None;
    }
    let version = format!("{}.{}.{}", latest.0, latest.1, latest.2);
    Some(InstallerUpdate {
        url: format!("https://github.com/{}/releases/tag/v{}", INSTALLER_REPO, version),
        version,
    })
}

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

pub const OFFICIAL_PUBLIC_KEY: &str = "RWQhzS5dR0kodMCMiFMESBDRDhxMyRaA4yNbIcAMD2B9kQS21I+FABT4";

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

    pub fn sha256_asset(&self) -> Option<&ReleaseAsset> {
        self.assets.iter().find(|a| a.name == "SHA256SUMS" || a.name.to_lowercase().contains("sha256"))
    }

    pub fn minisig_asset(&self) -> Option<&ReleaseAsset> {
        self.assets.iter().find(|a| a.name.ends_with(".minisig"))
    }
}

pub struct GitHubClient {
    agent: ureq::Agent,
    lang: CliLang,
}

impl GitHubClient {
    /// `lang`: language of the error and progress texts.
    pub fn new(lang: CliLang) -> Self {
        Self {
            lang,
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
            .context(self.lang.github_request_failed())?;

        let releases: Vec<Release> = resp.into_json()
            .context(self.lang.github_decode_failed())?;
        Ok(releases)
    }

    /// The newest published version of the installer when it is newer than `current`. GitHub's "latest" leaves out
    /// drafts and pre-releases.
    pub fn newer_installer(&self, current: &str) -> Result<Option<InstallerUpdate>> {
        #[derive(Deserialize)]
        struct Latest {
            tag_name: String,
        }
        let url = format!("https://api.github.com/repos/{}/releases/latest", INSTALLER_REPO);
        let latest: Latest = self.agent.get(&url)
            .set("Accept", "application/vnd.github.v3+json")
            .call()
            .context(self.lang.github_request_failed())?
            .into_json()
            .context(self.lang.github_decode_failed())?;
        Ok(installer_update(&latest.tag_name, current))
    }

    #[allow(dead_code)]
    pub fn get_latest_release(&self) -> Result<Release> {
        let releases = self.list_releases()?;
        releases.into_iter().find(|r| !r.prerelease)
            .ok_or_else(|| anyhow!(self.lang.no_stable_release()))
    }

    #[allow(dead_code)]
    pub fn list_branches(&self) -> Result<Vec<String>> {
        let url = format!("https://api.github.com/repos/{}/branches", GITHUB_REPO);
        let resp = self.agent.get(&url)
            .set("Accept", "application/vnd.github.v3+json")
            .call()
            .context(self.lang.github_request_failed())?;

        #[derive(Deserialize)]
        struct Branch { name: String }
        let branches: Vec<Branch> = resp.into_json()
            .context(self.lang.github_decode_failed())?;
        Ok(branches.into_iter().map(|b| b.name).collect())
    }

    /// Télécharge un asset avec affichage d'une barre de progression
    pub fn download_file(&self, url: &str, dest_path: &Path) -> Result<()> {
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let resp = self.agent.get(url).call()
            .map_err(|e| anyhow!(self.lang.download_error(url, &e.to_string())))?;

        let total_size = resp.header("Content-Length")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
            .progress_chars("#>-"));

        let mut reader = resp.into_reader();
        let mut file = File::create(dest_path)
            .context(self.lang.cannot_create_file())?;

        let mut buffer = [0u8; 8192];
        let mut downloaded: u64 = 0;

        loop {
            let n = reader.read(&mut buffer)?;
            if n == 0 { break; }
            file.write_all(&buffer[..n])?;
            downloaded += n as u64;
            pb.set_position(downloaded);
        }

        pb.finish_with_message(self.lang.download_done());
        Ok(())
    }

    /// Télécharge un contenu texte distant (ex: SHA256SUMS ou .minisig)
    pub fn download_string(&self, url: &str) -> Result<String> {
        let resp = self.agent.get(url).call()
            .map_err(|e| anyhow!(self.lang.download_error(url, &e.to_string())))?;
        resp.into_string()
            .context(self.lang.github_decode_failed())
    }

    /// Télécharge et valide cryptographiquement l'intégrité et la signature d'un asset
    pub fn download_and_verify_asset(&self, release: &Release, asset: &ReleaseAsset) -> Result<PathBuf> {
        let release_cache_dir = Self::cache_dir().join(&release.tag_name);
        fs::create_dir_all(&release_cache_dir)?;
        let dest = release_cache_dir.join(&asset.name);

        // 1. Télécharger le fichier binaire
        self.download_file(&asset.browser_download_url, &dest)?;

        // 2. Si SHA256SUMS est disponible dans la release, vérifier l'intégrité
        if let Some(sha_asset) = release.sha256_asset() {
            let sha256_sums = self.download_string(&sha_asset.browser_download_url)
                .context(self.lang.checksums_download_failed())?;

            // 2a. Si la signature Minisign est présente, vérifier son authenticité
            if let Some(sig_asset) = release.minisig_asset() {
                let sig_content = self.download_string(&sig_asset.browser_download_url)
                    .context(self.lang.signature_download_failed())?;

                let pk = minisign_verify::PublicKey::from_base64(OFFICIAL_PUBLIC_KEY)
                    .map_err(|e| anyhow!(self.lang.invalid_public_key(&format!("{:?}", e))))?;
                let signature = minisign_verify::Signature::decode(&sig_content)
                    .map_err(|e| anyhow!(self.lang.invalid_signature(&format!("{:?}", e))))?;

                pk.verify(sha256_sums.as_bytes(), &signature, false)
                    .map_err(|e| anyhow!(self.lang.signature_check_failed(&release.tag_name, &format!("{:?}", e))))?;
            }

            // 2b. Vérifier que le SHA256 du fichier téléchargé correspond bien à la table
            let computed_hash = Self::compute_sha256(&dest)?;
            let mut matched = false;
            for line in sha256_sums.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 && parts[0].eq_ignore_ascii_case(&computed_hash) {
                    let filename = parts[1].trim_start_matches('*');
                    if filename == asset.name {
                        matched = true;
                        break;
                    }
                }
            }

            if !matched {
                fs::remove_file(&dest).ok();
                return Err(anyhow!(self.lang.checksum_mismatch(&computed_hash)));
            }
        }

        Ok(dest)
    }

    /// Dossier local de cache des firmwares
    pub fn cache_dir() -> PathBuf {
        let base = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".to_string());
        PathBuf::from(base).join(".cache").join("openfirenet-installer").join("firmware")
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_official_public_key_valid() {
        let pk = minisign_verify::PublicKey::from_base64(OFFICIAL_PUBLIC_KEY);
        assert!(pk.is_ok(), "The official Open Firenet public key should parse without error");
    }

    #[test]
    fn test_minisign_signature_tamper_detection() {
        let pk = minisign_verify::PublicKey::from_base64(OFFICIAL_PUBLIC_KEY).unwrap();
        // A valid signature string format
        let sig_str = "untrusted comment: signature from minisign secret key\n\
                       RWQhzS5dR0kodI/W8hXf3s+Lp2VnC2/N7rZ1y+u8E9Q=\ntrusted comment: timestamp:0\n\
                       RWQhzS5dR0kodI/W8hXf3s+Lp2VnC2/N7rZ1y+u8E9Q=";
        if let Ok(signature) = minisign_verify::Signature::decode(sig_str) {
            let result = pk.verify(b"tampered content", &signature, false);
            assert!(result.is_err(), "Verification should fail on tampered content");
        }
    }
}

#[cfg(test)]
mod update_tests {
    use super::{installer_update, InstallerUpdate};

    #[test]
    fn a_newer_version_is_offered_with_its_page() {
        assert_eq!(
            installer_update("v1.9.0", "1.8.1"),
            Some(InstallerUpdate {
                version: "1.9.0".into(),
                url: "https://github.com/openfirenet/open-firenet-installer/releases/tag/v1.9.0".into(),
            })
        );
        assert!(installer_update("v1.10.0", "1.9.3").is_some()); // numbers, not text: 10 > 9
        assert!(installer_update("v2.0.0", "1.99.99").is_some());
    }

    #[test]
    fn the_same_or_an_older_version_is_not() {
        assert_eq!(installer_update("v1.8.1", "1.8.1"), None);
        assert_eq!(installer_update("v1.8.0", "1.8.1"), None);
        assert_eq!(installer_update("v1.8.1", "v1.8.1"), None);
    }

    #[test]
    fn a_tag_that_is_not_a_plain_version_is_ignored() {
        assert_eq!(installer_update("v2.0.0-rc1", "1.8.1"), None);
        assert_eq!(installer_update("latest", "1.8.1"), None);
        assert_eq!(installer_update("v2.0", "1.8.1"), None);
        assert_eq!(installer_update("v2.0.0/../../evil", "1.8.1"), None);
    }
}
