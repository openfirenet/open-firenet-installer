# 🔥 Open-Firenet Installer

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-green.svg)](#)
[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)

**Open-Firenet Installer** est l'assistant universel et multiplateforme pour installer, flasher et mettre à jour votre clé Wi-Fi **Open-Firenet** (remplacement open-source du module RIKA Firenet).

Contrairement aux solutions WebSerial (ESP Web Tools) qui ne fonctionnent **ni sur Mozilla Firefox ni sur Apple Safari**, cet outil fonctionne nativement sur **Windows**, **macOS** (Intel & Apple Silicon) et **Linux**, sous la forme d'un exécutable unique et autonome.

---

## ✨ Fonctionnalités principales

1. **🔍 Détection automatique sur le réseau local** :
   - Scan mDNS (`openfirenet.local`) et scan rapide du sous-réseau.
   - Découverte instantanée de l'adresse IP, du modèle de poêle connecté (ex: *DOMO*, *INDUO*), de l'état actuel et de la force du signal Wi-Fi.
2. **⚡ Flashage USB Série (Premier flashage / Réinstallation)** :
   - Détection automatique du port série USB de l'ESP32-S3.
   - Récupération automatique du firmware complet (`factory.bin`) depuis les releases officielles GitHub.
   - Flashage rapide et sécurisé de la mémoire flash.
3. **📡 Mise à jour à distance via Wi-Fi (OTA)** :
   - Mettez à jour le dongle branché sur votre poêle dans le salon sans jamais le débrancher !
   - Envoi du firmware avec barre de progression et suivi du redémarrage en temps réel.
4. **📶 Assistant de configuration Wi-Fi** :
   - Saisie guidée du SSID et du mot de passe pour connecter facilement votre clé neuve à votre box.
5. **📦 Gestion des versions GitHub** :
   - Téléchargement des versions stables, pré-releases ou sélection d'un fichier binaire local.
6. **📟 Moniteur Série intégré** :
   - Visualisation des journaux de démarrage et des trames de dialogue poêle/clé en direct.

---

## 🚀 Utilisation

### Mode Interactif (Recommandé)

Lancez simplement l'application sans argument (ou double-cliquez sur l'exécutable sous Windows/Mac) :

```bash
./open-firenet-installer
```

Un menu interactif s'affiche dans votre terminal :

```text
╔═══════════════════════════════════════════════════════════════╗
║                 🔥  OPEN-FIRENET INSTALLER  🔥                ║
║      Assistant multiplateforme de flash USB et mise à jour    ║
╚═══════════════════════════════════════════════════════════════╝

? Que souhaitez-vous faire ?
❯ 🔍 1. Scanner le réseau local (détecter la clé & état du poêle)
  ⚡ 2. Flasher la clé en USB (premier flash / réinstallation)
  📡 3. Mettre à jour la clé à distance via Wi-Fi (OTA)
  📶 4. Configurer le Wi-Fi de la clé (via USB Série)
  📦 5. Consulter les versions GitHub (releases & pré-releases)
  📟 6. Moniteur Série (voir les logs du poêle en direct)
  🚪 7. Quitter
```

---

### Mode Ligne de Commande (CLI)

Pour les scripts, l'automatisation ou les utilisateurs avancés :

```bash
# Scanner le réseau local
open-firenet-installer scan

# Scanner un sous-réseau spécifique
open-firenet-installer scan --subnet 192.168.1

# Flasher en USB sur un port spécifique
open-firenet-installer flash --port /dev/ttyACM0

# Flasher un fichier binaire local spécifique
open-firenet-installer flash --file ./open-firenet-factory.bin

# Mettre à jour via Wi-Fi (OTA) une clé à une IP donnée
open-firenet-installer ota --ip 192.168.1.93

# Mettre à jour avec une version GitHub spécifique
open-firenet-installer ota --ip 192.168.1.93 --release v2.0.0

# Configurer le Wi-Fi par liaison série
open-firenet-installer wifi-setup --port /dev/ttyACM0

# Écouter les logs série (baudrate 115200)
open-firenet-installer monitor --port /dev/ttyACM0
```

---

## 🛠 Compilation depuis les sources

Prérequis : [Rust et Cargo](https://rustup.rs/) ($\ge$ 1.85).

```bash
# Cloner le dépôt
git clone https://github.com/openfirenet/open-firenet-installer.git
cd open-firenet-installer

# Compiler en mode release optimisé
cargo build --release

# L'exécutable se trouve dans target/release/open-firenet-installer
./target/release/open-firenet-installer
```

---

## 📄 Licence

Distribué sous licence **Apache-2.0**. Voir [LICENSE](LICENSE) pour plus de détails.
