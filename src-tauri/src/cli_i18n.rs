#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CliLang {
    #[default]
    Fr,
    En,
    De,
    It,
}

impl CliLang {
    pub fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "en" | "english" => CliLang::En,
            "de" | "deutsch" | "german" => CliLang::De,
            "it" | "italiano" | "italian" => CliLang::It,
            _ => CliLang::Fr,
        }
    }

    pub fn detect() -> Self {
        if let Ok(lang_var) = std::env::var("LANG").or_else(|_| std::env::var("LC_ALL")) {
            let l = lang_var.to_lowercase();
            if l.starts_with("de") {
                return CliLang::De;
            } else if l.starts_with("it") {
                return CliLang::It;
            } else if l.starts_with("en") {
                return CliLang::En;
            }
        }
        CliLang::Fr
    }

    pub fn banner_subtitle(&self) -> &'static str {
        match self {
            CliLang::Fr => "Assistant multiplateforme de flash USB et mise à jour",
            CliLang::En => "Cross-platform USB flashing and update assistant",
            CliLang::De => "Plattformübergreifender USB-Flash- und Update-Assistent",
            CliLang::It => "Assistente multipiattaforma per flash USB e aggiornamento",
        }
    }

    pub fn menu_title(&self) -> &'static str {
        match self {
            CliLang::Fr => "Que souhaitez-vous faire ?",
            CliLang::En => "What would you like to do?",
            CliLang::De => "Was möchten Sie tun?",
            CliLang::It => "Cosa vuoi fare?",
        }
    }

    pub fn menu_choices(&self) -> [&'static str; 7] {
        match self {
            CliLang::Fr => [
                "🔍 1. Scanner le réseau local (détecter la clé & état du poêle)",
                "⚡ 2. Flasher la clé en USB (premier flash / réinstallation)",
                "📡 3. Mettre à jour la clé à distance via Wi-Fi (OTA)",
                "📶 4. Configurer le Wi-Fi de la clé (via USB Série)",
                "📦 5. Consulter les versions GitHub (releases & pré-releases)",
                "📟 6. Moniteur Série (voir les logs du poêle en direct)",
                "🚪 7. Quitter",
            ],
            CliLang::En => [
                "🔍 1. Scan local network (discover dongle & stove status)",
                "⚡ 2. Flash dongle via USB (first flash / reinstall)",
                "📡 3. Update dongle remotely via Wi-Fi (OTA)",
                "📶 4. Configure dongle Wi-Fi (via USB Serial)",
                "📦 5. Browse GitHub versions (releases & pre-releases)",
                "📟 6. Serial Monitor (view stove logs live)",
                "🚪 7. Quit",
            ],
            CliLang::De => [
                "🔍 1. Lokales Netzwerk scannen (Dongle & Ofenstatus finden)",
                "⚡ 2. Dongle per USB flashen (Erstflash / Neuinstallation)",
                "📡 3. Dongle drahtlos per WLAN aktualisieren (OTA)",
                "📶 4. WLAN des Dongles einrichten (über USB-Seriell)",
                "📦 5. GitHub-Versionen ansehen (Releases & Pre-Releases)",
                "📟 6. Serieller Monitor (Live-Logs des Ofens ansehen)",
                "🚪 7. Beenden",
            ],
            CliLang::It => [
                "🔍 1. Scansiona la rete locale (rileva il dongle e lo stato della stufa)",
                "⚡ 2. Flash del dongle tramite USB (primo flash / reinstallazione)",
                "📡 3. Aggiorna il dongle a distanza tramite Wi-Fi (OTA)",
                "📶 4. Configura il Wi-Fi del dongle (tramite USB seriale)",
                "📦 5. Consulta le versioni GitHub (release e pre-release)",
                "📟 6. Monitor seriale (vedi i log della stufa in diretta)",
                "🚪 7. Esci",
            ],
        }
    }

    pub fn menu_return_prompt(&self) -> &'static str {
        match self {
            CliLang::Fr => "Revenir au menu principal ?",
            CliLang::En => "Return to main menu?",
            CliLang::De => "Zurück zum Hauptmenü?",
            CliLang::It => "Tornare al menu principale?",
        }
    }

    pub fn goodbye(&self) -> &'static str {
        match self {
            CliLang::Fr => "Au revoir !",
            CliLang::En => "Goodbye!",
            CliLang::De => "Auf Wiedersehen!",
            CliLang::It => "Arrivederci!",
        }
    }

    pub fn scan_searching(&self) -> &'static str {
        match self {
            CliLang::Fr => "🔍 Recherche de la clé Open Firenet sur votre réseau...",
            CliLang::En => "🔍 Searching for Open Firenet dongle on your network...",
            CliLang::De => "🔍 Open Firenet-Dongle wird im Netzwerk gesucht...",
            CliLang::It => "🔍 Ricerca del dongle Open Firenet sulla tua rete...",
        }
    }

    pub fn scan_not_found(&self) -> &'static str {
        match self {
            CliLang::Fr => "❌ Aucune clé Open Firenet détectée sur le réseau.",
            CliLang::En => "❌ No Open Firenet dongle detected on the network.",
            CliLang::De => "❌ Kein Open Firenet-Dongle im Netzwerk gefunden.",
            CliLang::It => "❌ Nessun dongle Open Firenet rilevato sulla rete.",
        }
    }

    pub fn scan_tips(&self) -> [&'static str; 4] {
        match self {
            CliLang::Fr => [
                "Conseils :",
                "  - Vérifiez que la clé est bien allumée et connectée au Wi-Fi.",
                "  - Si c'est un premier démarrage, connectez-vous au point d'accès Wi-Fi 'Open-Firenet-Setup'.",
                "  - Ou branchez-la en USB pour effectuer le premier flashage.",
            ],
            CliLang::En => [
                "Tips:",
                "  - Verify that the dongle is powered on and connected to Wi-Fi.",
                "  - If first boot, connect to Wi-Fi access point 'Open-Firenet-Setup'.",
                "  - Or plug it in via USB to perform the first flash.",
            ],
            CliLang::De => [
                "Tipps:",
                "  - Stellen Sie sicher, dass der Dongle eingeschaltet und im WLAN ist.",
                "  - Beim Erststart mit dem WLAN-Zugangspunkt 'Open-Firenet-Setup' verbinden.",
                "  - Oder per USB anschließen, um den ersten Flash durchzuführen.",
            ],
            CliLang::It => [
                "Suggerimenti:",
                "  - Verifica che il dongle sia acceso e connesso al Wi-Fi.",
                "  - Al primo avvio, connettiti al punto di accesso Wi-Fi 'Open-Firenet-Setup'.",
                "  - Oppure collegalo tramite USB per eseguire il primo flash.",
            ],
        }
    }

    pub fn scan_found(&self, count: usize) -> String {
        match self {
            CliLang::Fr => format!("✔ {} clé(s) Open Firenet trouvée(s) :", count),
            CliLang::En => format!("✔ {} Open Firenet dongle(s) found:", count),
            CliLang::De => format!("✔ {} Open Firenet-Dongle(s) gefunden:", count),
            CliLang::It => format!("✔ {} dongle Open Firenet trovato/i:", count),
        }
    }

    pub fn label_ip(&self) -> &'static str {
        match self {
            CliLang::Fr => "Adresse IP",
            CliLang::En => "IP Address",
            CliLang::De => "IP-Adresse",
            CliLang::It => "Indirizzo IP",
        }
    }

    pub fn label_hostname(&self) -> &'static str {
        match self {
            CliLang::Fr => "Nom d'hôte",
            CliLang::En => "Hostname",
            CliLang::De => "Hostname",
            CliLang::It => "Nome host",
        }
    }

    pub fn label_stove_model(&self) -> &'static str {
        match self {
            CliLang::Fr => "Modèle poêle",
            CliLang::En => "Stove model",
            CliLang::De => "Ofenmodell",
            CliLang::It => "Modello stufa",
        }
    }

    pub fn label_stove_state(&self) -> &'static str {
        match self {
            CliLang::Fr => "État actuel",
            CliLang::En => "Current state",
            CliLang::De => "Aktueller Status",
            CliLang::It => "Stato attuale",
        }
    }

    pub fn label_wifi_signal(&self) -> &'static str {
        match self {
            CliLang::Fr => "Signal Wi-Fi",
            CliLang::En => "Wi-Fi Signal",
            CliLang::De => "WLAN-Signal",
            CliLang::It => "Segnale Wi-Fi",
        }
    }

    pub fn label_firmware_version(&self) -> &'static str {
        match self {
            CliLang::Fr => "Version firmw.",
            CliLang::En => "Firmware ver.",
            CliLang::De => "Firmware-Vers.",
            CliLang::It => "Vers. firmware",
        }
    }

    pub fn label_web_access(&self) -> &'static str {
        match self {
            CliLang::Fr => "Accès Web",
            CliLang::En => "Web Access",
            CliLang::De => "Web-Zugriff",
            CliLang::It => "Accesso web",
        }
    }

    pub fn flash_usb_title(&self) -> &'static str {
        match self {
            CliLang::Fr => "⚡ Flashage USB série de la clé Open Firenet",
            CliLang::En => "⚡ USB Serial Flashing of Open Firenet dongle",
            CliLang::De => "⚡ USB-Serieller Flash des Open Firenet-Dongles",
            CliLang::It => "⚡ Flash USB seriale del dongle Open Firenet",
        }
    }

    pub fn select_serial_port(&self) -> &'static str {
        match self {
            CliLang::Fr => "Sélectionnez le port série USB de votre ESP32-S3 :",
            CliLang::En => "Select the USB serial port of your ESP32-S3:",
            CliLang::De => "Wählen Sie den USB-Seriell-Port Ihres ESP32-S3:",
            CliLang::It => "Seleziona la porta seriale USB del tuo ESP32-S3:",
        }
    }

    pub fn no_serial_port_found(&self) -> &'static str {
        match self {
            CliLang::Fr => "❌ Aucun port série USB détecté. Branchez votre ESP32-S3 en USB.",
            CliLang::En => "❌ No USB serial port detected. Plug in your ESP32-S3 via USB.",
            CliLang::De => "❌ Kein USB-Seriell-Port erkannt. Bitte ESP32-S3 per USB anschließen.",
            CliLang::It => "❌ Nessuna porta seriale USB rilevata. Collega il tuo ESP32-S3 tramite USB.",
        }
    }

    pub fn usb_native_hint(&self) -> &'static str {
        match self {
            CliLang::Fr => "💡 Cartes à USB natif (M5Stamp S3, XIAO ESP32-S3) : si la connexion échoue, maintenez le bouton BOOT enfoncé lors du branchement USB.",
            CliLang::En => "💡 Native USB boards (M5Stamp S3, XIAO ESP32-S3): if connection fails, hold down the BOOT button while plugging in the USB cable.",
            CliLang::De => "💡 Boards mit nativem USB (M5Stamp S3, XIAO ESP32-S3): falls Verbindung fehlschlägt, halten Sie beim Einstecken die BOOT-Taste gedrückt.",
            CliLang::It => "💡 Schede con USB nativo (M5Stamp S3, XIAO ESP32-S3): se la connessione non riesce, tieni premuto il pulsante BOOT mentre colleghi il cavo USB.",
        }
    }

    pub fn usb_post_flash_hint(&self) -> &'static str {
        match self {
            CliLang::Fr => "💡 Note : Sur les cartes à USB natif, appuyez sur le bouton RESET pour lancer le nouveau firmware.",
            CliLang::En => "💡 Note: On native USB boards, press the RESET button to start the new firmware.",
            CliLang::De => "💡 Hinweis: Drücken Sie bei Boards mit nativem USB die RESET-Taste, um die neue Firmware zu starten.",
            CliLang::It => "💡 Nota: sulle schede con USB nativo, premi il pulsante RESET per avviare il nuovo firmware.",
        }
    }

    pub fn ota_updating_to(&self, ip: &str) -> String {
        match self {
            CliLang::Fr => format!("📡 Mise à jour Wi-Fi (OTA) vers {}", ip),
            CliLang::En => format!("📡 Wi-Fi Update (OTA) to {}", ip),
            CliLang::De => format!("📡 WLAN-Update (OTA) an {}", ip),
            CliLang::It => format!("📡 Aggiornamento Wi-Fi (OTA) verso {}", ip),
        }
    }

    pub fn select_dongle_to_update(&self) -> &'static str {
        match self {
            CliLang::Fr => "Sélectionnez la clé à mettre à jour :",
            CliLang::En => "Select the dongle to update:",
            CliLang::De => "Wählen Sie den zu aktualisierenden Dongle:",
            CliLang::It => "Seleziona il dongle da aggiornare:",
        }
    }

    pub fn enter_ip_prompt(&self) -> &'static str {
        match self {
            CliLang::Fr => "Entrez l'adresse IP de votre clé Open Firenet",
            CliLang::En => "Enter the IP address of your Open Firenet dongle",
            CliLang::De => "Geben Sie die IP-Adresse des Open Firenet-Dongles ein",
            CliLang::It => "Inserisci l'indirizzo IP del tuo dongle Open Firenet",
        }
    }

    pub fn fetching_releases(&self) -> &'static str {
        match self {
            CliLang::Fr => "Récupération des versions disponibles sur GitHub...",
            CliLang::En => "Fetching available releases from GitHub...",
            CliLang::De => "Verfügbare Releases von GitHub werden abgerufen...",
            CliLang::It => "Recupero delle versioni disponibili su GitHub...",
        }
    }

    pub fn no_releases_found(&self) -> &'static str {
        match self {
            CliLang::Fr => "Aucune release publiée sur GitHub pour le moment.",
            CliLang::En => "No releases published on GitHub yet.",
            CliLang::De => "Bisher keine offiziellen Releases auf GitHub veröffentlicht.",
            CliLang::It => "Nessuna release pubblicata su GitHub per ora.",
        }
    }

    pub fn enter_bin_path_prompt(&self) -> &'static str {
        match self {
            CliLang::Fr => "Chemin vers votre fichier binaire .bin local",
            CliLang::En => "Path to your local .bin binary file",
            CliLang::De => "Pfad zu Ihrer lokalen .bin-Binärdatei",
            CliLang::It => "Percorso del tuo file binario .bin locale",
        }
    }

    pub fn choose_version_prompt(&self) -> &'static str {
        match self {
            CliLang::Fr => "Choisissez la version à installer :",
            CliLang::En => "Choose the version to install:",
            CliLang::De => "Wählen Sie die zu installierende Version:",
            CliLang::It => "Scegli la versione da installare:",
        }
    }

    pub fn badge_prerelease(&self) -> &'static str {
        match self {
            CliLang::Fr => " (pré-release/test)",
            CliLang::En => " (pre-release/test)",
            CliLang::De => " (Pre-Release/Test)",
            CliLang::It => " (pre-release/test)",
        }
    }

    pub fn badge_stable(&self) -> &'static str {
        match self {
            CliLang::Fr => " (stable)",
            CliLang::En => " (stable)",
            CliLang::De => " (stabil)",
            CliLang::It => " (stabile)",
        }
    }

    pub fn no_binary_found(&self) -> &'static str {
        match self {
            CliLang::Fr => "Aucun binaire précompilé approprié trouvé dans cette release.",
            CliLang::En => "No suitable precompiled binary found in this release.",
            CliLang::De => "Keine passende vorkompilierte Binärdatei in diesem Release gefunden.",
            CliLang::It => "Nessun binario precompilato adatto trovato in questa release.",
        }
    }

    pub fn use_cached_prompt(&self, filename: &str) -> String {
        match self {
            CliLang::Fr => format!("Utiliser la version en cache ({}) ?", filename),
            CliLang::En => format!("Use cached version ({})?", filename),
            CliLang::De => format!("Version aus dem Cache verwenden ({})?", filename),
            CliLang::It => format!("Usare la versione in cache ({})?", filename),
        }
    }

    pub fn downloading_asset(&self, name: &str) -> String {
        match self {
            CliLang::Fr => format!("Téléchargement et vérification cryptographique de {}...", name),
            CliLang::En => format!("Downloading and cryptographically verifying {}...", name),
            CliLang::De => format!("Herunterladen und kryptografische Überprüfung von {}...", name),
            CliLang::It => format!("Download e verifica crittografica di {}...", name),
        }
    }

    pub fn verification_ok(&self) -> &'static str {
        match self {
            CliLang::Fr => "✔ Signature Minisign et intégrité SHA256 validées avec succès !",
            CliLang::En => "✔ Minisign signature and SHA256 integrity successfully verified!",
            CliLang::De => "✔ Minisign-Signatur und SHA256-Integrität erfolgreich überprüft!",
            CliLang::It => "✔ Firma Minisign e integrità SHA256 verificate con successo!",
        }
    }

    pub fn releases_title(&self) -> &'static str {
        match self {
            CliLang::Fr => "📦 Versions d'Open Firenet publiées sur GitHub :",
            CliLang::En => "📦 Open Firenet releases published on GitHub:",
            CliLang::De => "📦 Auf GitHub veröffentlichte Open Firenet-Releases:",
            CliLang::It => "📦 Versioni di Open Firenet pubblicate su GitHub:",
        }
    }

    pub fn wifi_setup_title(&self) -> &'static str {
        match self {
            CliLang::Fr => "📶 Configuration Wi-Fi du dongle Open Firenet",
            CliLang::En => "📶 Open Firenet Dongle Wi-Fi Configuration",
            CliLang::De => "📶 WLAN-Konfiguration des Open Firenet-Dongles",
            CliLang::It => "📶 Configurazione Wi-Fi del dongle Open Firenet",
        }
    }

    pub fn wifi_setup_desc(&self) -> &'static str {
        match self {
            CliLang::Fr => "Ces informations permettront à la clé de se connecter à votre réseau local.",
            CliLang::En => "This information will allow the dongle to connect to your local network.",
            CliLang::De => "Diese Daten ermöglichen dem Dongle die Verbindung mit Ihrem lokalen Netzwerk.",
            CliLang::It => "Queste informazioni permetteranno al dongle di connettersi alla tua rete locale.",
        }
    }

    pub fn wifi_native_hint(&self) -> &'static str {
        match self {
            CliLang::Fr => "💡 Astuce : Sur les cartes sans puce UART dédiée, vous pouvez aussi vous connecter au point d'accès Wi-Fi « Open-Firenet-Setup » (192.168.4.1) pour configurer le réseau.",
            CliLang::En => "💡 Tip: On boards without a dedicated UART chip, you can also connect to the \"Open-Firenet-Setup\" Wi-Fi access point (192.168.4.1) to configure the network.",
            CliLang::De => "💡 Tipp: Auf Boards ohne dedizierten UART-Chip können Sie das Netzwerk auch über den WLAN-Access-Point „Open-Firenet-Setup“ (192.168.4.1) einrichten.",
            CliLang::It => "💡 Suggerimento: sulle schede senza chip UART dedicato, puoi anche connetterti al punto di accesso Wi-Fi «Open-Firenet-Setup» (192.168.4.1) per configurare la rete.",
        }
    }

    pub fn wifi_ssid_prompt(&self) -> &'static str {
        match self {
            CliLang::Fr => "Nom de votre réseau Wi-Fi (SSID)",
            CliLang::En => "Your Wi-Fi Network Name (SSID)",
            CliLang::De => "WLAN-Netzwerkname (SSID)",
            CliLang::It => "Nome della tua rete Wi-Fi (SSID)",
        }
    }

    pub fn wifi_pass_prompt(&self) -> &'static str {
        match self {
            CliLang::Fr => "Mot de passe Wi-Fi (WPA2/WPA3)",
            CliLang::En => "Wi-Fi Password (WPA2/WPA3)",
            CliLang::De => "WLAN-Passwort (WPA2/WPA3)",
            CliLang::It => "Password Wi-Fi (WPA2/WPA3)",
        }
    }

    pub fn wifi_sending(&self, port: &str) -> String {
        match self {
            CliLang::Fr => format!("Envoi de la configuration sur {}...", port),
            CliLang::En => format!("Sending configuration to {}...", port),
            CliLang::De => format!("Konfiguration wird an {} gesendet...", port),
            CliLang::It => format!("Invio della configurazione su {}...", port),
        }
    }

    pub fn wifi_sent_success(&self) -> &'static str {
        match self {
            CliLang::Fr => "✔ Commande transmise. Le dongle va tenter de se connecter.",
            CliLang::En => "✔ Command sent. The dongle will now attempt to connect.",
            CliLang::De => "✔ Befehl gesendet. Der Dongle versucht nun, sich zu verbinden.",
            CliLang::It => "✔ Comando inviato. Il dongle proverà ora a connettersi.",
        }
    }

    pub fn monitor_opening(&self, port: &str, baud: u32) -> String {
        match self {
            CliLang::Fr => format!("📟 Ouverture du moniteur série sur {} à {} bauds (Ctrl+C pour quitter)...", port, baud),
            CliLang::En => format!("📟 Opening serial monitor on {} at {} baud (Ctrl+C to quit)...", port, baud),
            CliLang::De => format!("📟 Serieller Monitor wird auf {} mit {} Baud geöffnet (Strg+C zum Beenden)...", port, baud),
            CliLang::It => format!("📟 Apertura del monitor seriale su {} a {} baud (Ctrl+C per uscire)...", port, baud),
        }
    }

    pub fn monitor_session_end(&self) -> &'static str {
        match self {
            CliLang::Fr => "Fin de session série :",
            CliLang::En => "Serial session ended:",
            CliLang::De => "Serielle Sitzung beendet:",
            CliLang::It => "Sessione seriale terminata:",
        }
    }
}

/// One text per language, in the order French, English, German, Italian.
macro_rules! tr {
    ($lang:expr, $fr:expr, $en:expr, $de:expr, $it:expr $(,)?) => {
        match $lang {
            CliLang::Fr => $fr,
            CliLang::En => $en,
            CliLang::De => $de,
            CliLang::It => $it,
        }
    };
}

/// Texts of the flashing, download and discovery code. They are shown by the command line and, through the
/// progress and error messages, by the window: both pass the user's language.
impl CliLang {
    /// Language chosen in the window ("fr", "en", "de", "it"). A language the window has but this file does not
    /// have yet gets English.
    pub fn from_window(lang: Option<&str>) -> Self {
        match lang.map(|l| l.trim().to_lowercase()).as_deref() {
            Some("fr") => CliLang::Fr,
            Some("de") => CliLang::De,
            Some("it") => CliLang::It,
            _ => CliLang::En,
        }
    }

    // --- files -------------------------------------------------------------------------------------------------
    pub fn file_not_found(&self, path: &std::path::Path) -> String {
        tr!(self,
            format!("Le fichier binaire n'existe pas : {:?}", path),
            format!("The binary file does not exist: {:?}", path),
            format!("Die Binärdatei existiert nicht: {:?}", path),
            format!("Il file binario non esiste: {:?}", path))
    }
    pub fn cannot_read_firmware(&self, path: &std::path::Path) -> String {
        tr!(self,
            format!("Impossible de lire le fichier firmware : {:?}", path),
            format!("Cannot read the firmware file: {:?}", path),
            format!("Die Firmware-Datei kann nicht gelesen werden: {:?}", path),
            format!("Impossibile leggere il file del firmware: {:?}", path))
    }

    // --- wireless update ---------------------------------------------------------------------------------------
    pub fn ota_preparing(&self, ip: &str) -> String {
        tr!(self,
            format!("Préparation de la mise à jour sans fil vers {}...", ip),
            format!("Preparing the wireless update to {}...", ip),
            format!("Drahtloses Update an {} wird vorbereitet...", ip),
            format!("Preparazione dell'aggiornamento tramite Wi-Fi verso {}...", ip))
    }
    pub fn reading_firmware(&self) -> &'static str {
        tr!(self,
            "Lecture et vérification du firmware...",
            "Reading and checking the firmware...",
            "Firmware wird gelesen und geprüft...",
            "Lettura e verifica del firmware...")
    }
    pub fn firmware_info(&self, bytes: usize, md5: &str) -> String {
        tr!(self,
            format!("Firmware : {} octets, MD5 : {}", bytes, md5),
            format!("Firmware: {} bytes, MD5: {}", bytes, md5),
            format!("Firmware: {} Bytes, MD5: {}", bytes, md5),
            format!("Firmware: {} byte, MD5: {}", bytes, md5))
    }
    pub fn ota_slot_too_small(&self, firmware_bytes: usize, slot_bytes: usize) -> String {
        tr!(self,
            format!("L'emplacement de mise à jour de cette clé fait {} octets et ce firmware en fait {}. Une mise à jour sans fil ne peut pas agrandir cet emplacement : flashez la clé une fois par USB en mode « Reset complet (Image Usine) », le seul qui réécrit la table de partitions (le mode « Mise à jour » la conserve). Le Wi-Fi sera à ressaisir, puis les mises à jour sans fil fonctionneront de nouveau.", slot_bytes, firmware_bytes),
            format!("This dongle's update slot is {} bytes and this firmware is {} bytes. A wireless update cannot enlarge the slot: flash the dongle once over USB in \"Full Reset (Factory Image)\" mode, the only one that rewrites the partition table (the \"Update\" mode keeps it). You will have to enter the Wi-Fi again; wireless updates will work afterwards.", slot_bytes, firmware_bytes),
            format!("Der Update-Speicherplatz dieses Dongles hat {} Bytes, diese Firmware {} Bytes. Ein drahtloses Update kann den Speicherplatz nicht vergrößern: Flashen Sie den Dongle einmal per USB im Modus „Kompletter Reset (Werkseinstellung)“, nur dieser schreibt die Partitionstabelle neu (der Modus „Update“ behält sie). Das WLAN muss danach neu eingegeben werden; drahtlose Updates funktionieren dann wieder.", slot_bytes, firmware_bytes),
            format!("Lo spazio di aggiornamento di questo dongle è di {} byte e questo firmware ne occupa {}. Un aggiornamento tramite Wi-Fi non può ampliare questo spazio: esegui una volta il flash del dongle tramite USB in modalità «Ripristino completo (immagine di fabbrica)», l'unica che riscrive la tabella delle partizioni (la modalità «Aggiornamento» la conserva). Dovrai reinserire il Wi-Fi; dopo, gli aggiornamenti tramite Wi-Fi funzioneranno di nuovo.", slot_bytes, firmware_bytes))
    }
    pub fn ota_tcp_bind_failed(&self) -> &'static str {
        tr!(self,
            "Impossible d'ouvrir le port d'écoute TCP local pour la mise à jour",
            "Cannot open the local TCP listening port for the update",
            "Der lokale TCP-Port für das Update kann nicht geöffnet werden",
            "Impossibile aprire la porta TCP locale per l'aggiornamento")
    }
    pub fn ota_tcp_listening(&self, port: u16) -> String {
        tr!(self,
            format!("Serveur TCP local en écoute sur le port {}", port),
            format!("Local TCP server listening on port {}", port),
            format!("Lokaler TCP-Server lauscht auf Port {}", port),
            format!("Server TCP locale in ascolto sulla porta {}", port))
    }
    pub fn ota_udp_bind_failed(&self) -> &'static str {
        tr!(self,
            "Impossible d'ouvrir la socket UDP locale pour la mise à jour",
            "Cannot open the local UDP socket for the update",
            "Der lokale UDP-Socket für das Update kann nicht geöffnet werden",
            "Impossibile aprire il socket UDP locale per l'aggiornamento")
    }
    pub fn invalid_ip(&self, ip: &str) -> String {
        tr!(self,
            format!("Adresse IP de la clé invalide : {}", ip),
            format!("Invalid dongle IP address: {}", ip),
            format!("Ungültige IP-Adresse des Dongles: {}", ip),
            format!("Indirizzo IP del dongle non valido: {}", ip))
    }
    pub fn ota_negotiating(&self, ip: &str) -> String {
        tr!(self,
            format!("Négociation avec la clé ({}:3232)...", ip),
            format!("Negotiating with the dongle ({}:3232)...", ip),
            format!("Aushandlung mit dem Dongle ({}:3232)...", ip),
            format!("Negoziazione con il dongle ({}:3232)...", ip))
    }
    pub fn ota_connecting(&self) -> &'static str {
        tr!(self,
            "Connexion à la clé via Wi-Fi...",
            "Connecting to the dongle over Wi-Fi...",
            "Verbindung zum Dongle über WLAN...",
            "Connessione al dongle tramite Wi-Fi...")
    }
    pub fn ota_auth_required(&self) -> &'static str {
        tr!(self,
            "La clé a demandé une authentification par mot de passe",
            "The dongle asked for password authentication",
            "Der Dongle verlangt eine Passwort-Authentifizierung",
            "Il dongle ha richiesto un'autenticazione con password")
    }
    pub fn ota_waiting_reply(&self, attempt: u32, of: u32) -> String {
        tr!(self,
            format!("Attente de la réponse de la clé ({}/{})...", attempt, of),
            format!("Waiting for the dongle to answer ({}/{})...", attempt, of),
            format!("Warten auf Antwort des Dongles ({}/{})...", attempt, of),
            format!("In attesa della risposta del dongle ({}/{})...", attempt, of))
    }
    pub fn ota_no_reply(&self, ip: &str) -> String {
        tr!(self,
            format!("La clé sur {} (port 3232) n'a pas répondu à l'invitation de mise à jour. Vérifiez qu'elle est allumée et sur le même réseau.", ip),
            format!("The dongle at {} (port 3232) did not answer the update invitation. Check that it is powered and on the same network.", ip),
            format!("Der Dongle unter {} (Port 3232) hat nicht auf die Update-Einladung geantwortet. Prüfen Sie, ob er eingeschaltet und im selben Netzwerk ist.", ip),
            format!("Il dongle su {} (porta 3232) non ha risposto all'invito di aggiornamento. Verifica che sia acceso e sulla stessa rete.", ip))
    }
    pub fn ota_waiting_tcp(&self) -> &'static str {
        tr!(self,
            "Attente de la connexion TCP de la clé...",
            "Waiting for the dongle's TCP connection...",
            "Warten auf die TCP-Verbindung des Dongles...",
            "In attesa della connessione TCP del dongle...")
    }
    pub fn ota_connected_init(&self) -> &'static str {
        tr!(self,
            "Connexion établie, initialisation du transfert...",
            "Connected, starting the transfer...",
            "Verbindung hergestellt, Übertragung wird gestartet...",
            "Connessione stabilita, avvio del trasferimento...")
    }
    pub fn ota_tcp_received(&self, peer: &str) -> String {
        tr!(self,
            format!("Connexion TCP reçue de la clé depuis {}", peer),
            format!("TCP connection received from the dongle at {}", peer),
            format!("TCP-Verbindung des Dongles von {} erhalten", peer),
            format!("Connessione TCP ricevuta dal dongle da {}", peer))
    }
    pub fn ota_tcp_accept_error(&self, error: &str) -> String {
        tr!(self,
            format!("Erreur d'acceptation TCP : {}", error),
            format!("TCP accept error: {}", error),
            format!("Fehler beim Annehmen der TCP-Verbindung: {}", error),
            format!("Errore di accettazione TCP: {}", error))
    }
    /// The dongle accepted the invitation but never connected back. `default_slot_bytes` is given when the
    /// firmware is larger than the update slot of Arduino's default partition scheme: that cause is then named too.
    pub fn ota_no_tcp_connection(&self, port: u16, firmware_bytes: usize, default_slot_bytes: Option<usize>) -> String {
        let mut text = tr!(self,
            format!("Délai d'attente dépassé : la clé n'a pas pu se connecter au port TCP {} de cet ordinateur. Vérifiez que votre pare-feu autorise les connexions entrantes sur le réseau local.", port),
            format!("Timed out: the dongle could not connect to TCP port {} of this computer. Check that your firewall allows incoming connections on the local network.", port),
            format!("Zeitüberschreitung: Der Dongle konnte sich nicht mit TCP-Port {} dieses Computers verbinden. Prüfen Sie, ob Ihre Firewall eingehende Verbindungen im lokalen Netzwerk zulässt.", port),
            format!("Tempo scaduto: il dongle non è riuscito a connettersi alla porta TCP {} di questo computer. Verifica che il firewall consenta le connessioni in entrata sulla rete locale.", port));
        if let Some(slot) = default_slot_bytes {
            text.push_str("\n\n");
            text.push_str(&tr!(self,
                format!("Autre cause possible : l'emplacement de mise à jour de la clé est trop petit pour ce firmware ({} octets). Il fait {} octets quand la clé a été flashée depuis l'IDE Arduino avec le schéma de partition par défaut. Dans ce cas, flashez la clé une fois par USB en mode « Reset complet (Image Usine) », le seul qui réécrit la table de partitions (le mode « Mise à jour » la conserve). Le Wi-Fi sera à ressaisir, puis les mises à jour sans fil fonctionneront de nouveau.", firmware_bytes, slot),
                format!("Other possible cause: the dongle's update slot is too small for this firmware ({} bytes). It is {} bytes when the dongle was flashed from the Arduino IDE with the default partition scheme. In that case, flash the dongle once over USB in \"Full Reset (Factory Image)\" mode, the only one that rewrites the partition table (the \"Update\" mode keeps it). You will have to enter the Wi-Fi again; wireless updates will work afterwards.", firmware_bytes, slot),
                format!("Andere mögliche Ursache: Der Update-Speicherplatz des Dongles ist für diese Firmware ({} Bytes) zu klein. Er hat {} Bytes, wenn der Dongle aus der Arduino IDE mit dem Standard-Partitionsschema geflasht wurde. Flashen Sie den Dongle in diesem Fall einmal per USB im Modus „Kompletter Reset (Werkseinstellung)“, nur dieser schreibt die Partitionstabelle neu (der Modus „Update“ behält sie). Das WLAN muss danach neu eingegeben werden; drahtlose Updates funktionieren dann wieder.", firmware_bytes, slot),
                format!("Altra causa possibile: lo spazio di aggiornamento del dongle è troppo piccolo per questo firmware ({} byte). È di {} byte quando il dongle è stato programmato dall'IDE Arduino con lo schema di partizioni predefinito. In tal caso, esegui una volta il flash del dongle tramite USB in modalità «Ripristino completo (immagine di fabbrica)», l'unica che riscrive la tabella delle partizioni (la modalità «Aggiornamento» la conserva). Dovrai reinserire il Wi-Fi; dopo, gli aggiornamenti tramite Wi-Fi funzioneranno di nuovo.", firmware_bytes, slot)));
        }
        text
    }
    pub fn ota_sending(&self) -> &'static str {
        tr!(self, "Envoi du firmware...", "Sending the firmware...", "Firmware wird gesendet...", "Invio del firmware...")
    }
    pub fn ota_send_error(&self) -> &'static str {
        tr!(self,
            "Erreur lors de l'envoi des données vers la clé",
            "Error while sending data to the dongle",
            "Fehler beim Senden der Daten an den Dongle",
            "Errore durante l'invio dei dati al dongle")
    }
    pub fn ota_progress(&self, percent: u32) -> String {
        tr!(self,
            format!("Envoi Wi-Fi : {}%", percent),
            format!("Wi-Fi transfer: {}%", percent),
            format!("WLAN-Übertragung: {}%", percent),
            format!("Invio Wi-Fi: {}%", percent))
    }
    pub fn ota_verifying(&self) -> &'static str {
        tr!(self,
            "Vérification de l'intégrité et écriture en mémoire par la clé...",
            "The dongle is checking integrity and writing to flash...",
            "Der Dongle prüft die Integrität und schreibt in den Flash...",
            "Il dongle verifica l'integrità e scrive in memoria...")
    }
    pub fn ota_done(&self) -> &'static str {
        tr!(self,
            "Mise à jour sans fil terminée avec succès !",
            "Wireless update completed successfully!",
            "Drahtloses Update erfolgreich abgeschlossen!",
            "Aggiornamento tramite Wi-Fi completato con successo!")
    }
    pub fn rebooted_online(&self) -> &'static str {
        tr!(self,
            "✔ La clé a redémarré et est de nouveau en ligne !",
            "✔ The dongle restarted and is back online!",
            "✔ Der Dongle wurde neu gestartet und ist wieder online!",
            "✔ Il dongle si è riavviato ed è di nuovo online!")
    }
    pub fn rebooting_waiting(&self) -> &'static str {
        tr!(self,
            "Redémarrage de la clé en cours... Attente de la reconnexion au Wi-Fi...",
            "The dongle is restarting... Waiting for it to rejoin the Wi-Fi...",
            "Der Dongle startet neu... Warten auf die erneute WLAN-Verbindung...",
            "Riavvio del dongle in corso... In attesa della riconnessione al Wi-Fi...")
    }
    pub fn waiting_reconnect(&self, attempt: u32, of: u32) -> String {
        tr!(self,
            format!("Attente de la reconnexion au réseau ({}/{})...", attempt, of),
            format!("Waiting for it to rejoin the network ({}/{})...", attempt, of),
            format!("Warten auf die erneute Netzwerkverbindung ({}/{})...", attempt, of),
            format!("In attesa della riconnessione alla rete ({}/{})...", attempt, of))
    }
    pub fn reconnect_slow(&self) -> &'static str {
        tr!(self,
            "La clé met plus de temps à se reconnecter. Vérifiez son adresse IP sur votre box.",
            "The dongle is taking longer to reconnect. Check its IP address on your router.",
            "Der Dongle braucht länger für die Wiederverbindung. Prüfen Sie seine IP-Adresse im Router.",
            "Il dongle impiega più tempo a riconnettersi. Verifica il suo indirizzo IP sul router.")
    }

    // --- USB flashing ------------------------------------------------------------------------------------------
    pub fn cannot_list_ports(&self) -> &'static str {
        tr!(self,
            "Impossible d'énumérer les ports série",
            "Cannot list the serial ports",
            "Die seriellen Ports können nicht aufgelistet werden",
            "Impossibile elencare le porte seriali")
    }
    pub fn port_generic(&self) -> &'static str {
        tr!(self, "Port série générique", "Generic serial port", "Generischer serieller Port", "Porta seriale generica")
    }
    pub fn unknown(&self) -> &'static str {
        tr!(self, "Inconnu", "Unknown", "Unbekannt", "Sconosciuto")
    }
    pub fn port_serial_adapter(&self, product: &str) -> String {
        tr!(self,
            format!("⚡ Adaptateur Série/USB ({})", product),
            format!("⚡ Serial/USB adapter ({})", product),
            format!("⚡ Seriell/USB-Adapter ({})", product),
            format!("⚡ Adattatore seriale/USB ({})", product))
    }
    pub fn invalid_offset(&self, offset: &str) -> String {
        tr!(self,
            format!("Adresse de départ invalide : {}", offset),
            format!("Invalid start address: {}", offset),
            format!("Ungültige Startadresse: {}", offset),
            format!("Indirizzo di partenza non valido: {}", offset))
    }
    pub fn usb_flash_start(&self, port: &str, baud: u32, offset: &str) -> String {
        tr!(self,
            format!("Début du flashage USB sur {} à {} bauds (adresse {})...", port, baud, offset),
            format!("Starting USB flashing on {} at {} baud (address {})...", port, baud, offset),
            format!("USB-Flash auf {} mit {} Baud wird gestartet (Adresse {})...", port, baud, offset),
            format!("Avvio del flash USB su {} a {} baud (indirizzo {})...", port, baud, offset))
    }
    pub fn usb_connecting_chip(&self) -> &'static str {
        tr!(self,
            "Connexion à la puce ESP32-S3...",
            "Connecting to the ESP32-S3 chip...",
            "Verbindung zum ESP32-S3-Chip...",
            "Connessione al chip ESP32-S3...")
    }
    pub fn usb_done(&self) -> &'static str {
        tr!(self,
            "Flashage terminé avec succès !",
            "Flashing completed successfully!",
            "Flash erfolgreich abgeschlossen!",
            "Flash completato con successo!")
    }
    pub fn usb_native_failed(&self, error: &str) -> String {
        tr!(self,
            format!("Échec du flashage intégré : {}", error),
            format!("Built-in flashing failed: {}", error),
            format!("Integrierter Flash fehlgeschlagen: {}", error),
            format!("Flash integrato non riuscito: {}", error))
    }
    pub fn usb_fallback(&self, tool: &str) -> String {
        tr!(self,
            format!("Tentative de secours via {}...", tool),
            format!("Trying again with {}...", tool),
            format!("Erneuter Versuch mit {}...", tool),
            format!("Nuovo tentativo con {}...", tool))
    }
    pub fn usb_flash_failed(&self, error: &str) -> String {
        tr!(self,
            format!("Échec du flashage USB : {}\n\nAstuce : sur les cartes à USB natif (M5Stamp S3, XIAO ESP32-S3), maintenez le bouton BOOT enfoncé lors du branchement USB pour forcer le mode de démarrage (bootloader).", error),
            format!("USB flashing failed: {}\n\nTip: on native USB boards (M5Stamp S3, XIAO ESP32-S3), hold down the BOOT button while plugging in the USB cable to force bootloader mode.", error),
            format!("USB-Flash fehlgeschlagen: {}\n\nTipp: Halten Sie bei Boards mit nativem USB (M5Stamp S3, XIAO ESP32-S3) beim Einstecken des USB-Kabels die BOOT-Taste gedrückt, um den Bootloader-Modus zu erzwingen.", error),
            format!("Flash USB non riuscito: {}\n\nSuggerimento: sulle schede con USB nativo (M5Stamp S3, XIAO ESP32-S3), tieni premuto il pulsante BOOT mentre colleghi il cavo USB per forzare la modalità bootloader.", error))
    }
    pub fn cannot_open_port(&self, port: &str, error: &str) -> String {
        tr!(self,
            format!("Impossible d'ouvrir le port série {} : {}", port, error),
            format!("Cannot open serial port {}: {}", port, error),
            format!("Der serielle Port {} kann nicht geöffnet werden: {}", port, error),
            format!("Impossibile aprire la porta seriale {}: {}", port, error))
    }
    pub fn cannot_open_serial(&self) -> &'static str {
        tr!(self,
            "Impossible d'ouvrir le port série",
            "Cannot open the serial port",
            "Der serielle Port kann nicht geöffnet werden",
            "Impossibile aprire la porta seriale")
    }
    pub fn usb_syncing(&self) -> &'static str {
        tr!(self,
            "Connexion et synchronisation avec l'ESP32-S3...",
            "Connecting and synchronising with the ESP32-S3...",
            "Verbindung und Synchronisierung mit dem ESP32-S3...",
            "Connessione e sincronizzazione con l'ESP32-S3...")
    }
    pub fn bootloader_connect_failed(&self, error: &str) -> String {
        tr!(self,
            format!("Connexion au bootloader impossible ({})", error),
            format!("Cannot connect to the bootloader ({})", error),
            format!("Verbindung zum Bootloader nicht möglich ({})", error),
            format!("Impossibile connettersi al bootloader ({})", error))
    }
    pub fn usb_synced_writing(&self) -> &'static str {
        tr!(self,
            "ESP32-S3 synchronisé. Écriture en mémoire flash...",
            "ESP32-S3 synchronised. Writing to flash...",
            "ESP32-S3 synchronisiert. Schreiben in den Flash...",
            "ESP32-S3 sincronizzato. Scrittura nella memoria flash...")
    }
    pub fn usb_preparing_flash(&self) -> &'static str {
        tr!(self,
            "Préparation de la mémoire flash...",
            "Preparing the flash memory...",
            "Flash-Speicher wird vorbereitet...",
            "Preparazione della memoria flash...")
    }
    pub fn usb_write_progress(&self, percent: u32) -> String {
        tr!(self,
            format!("Écriture flash USB : {}%", percent),
            format!("USB flash write: {}%", percent),
            format!("USB-Flash-Schreiben: {}%", percent),
            format!("Scrittura flash USB: {}%", percent))
    }
    pub fn integrity_ok(&self) -> &'static str {
        tr!(self,
            "Vérification de l'intégrité... OK",
            "Integrity check... OK",
            "Integritätsprüfung... OK",
            "Verifica dell'integrità... OK")
    }
    pub fn usb_write_error(&self, error: &str) -> String {
        tr!(self,
            format!("Erreur lors de l'écriture flash : {}", error),
            format!("Error while writing to flash: {}", error),
            format!("Fehler beim Schreiben in den Flash: {}", error),
            format!("Errore durante la scrittura flash: {}", error))
    }
    pub fn esptool_launch_failed(&self) -> &'static str {
        tr!(self,
            "Échec du lancement d'esptool",
            "Failed to start esptool",
            "esptool konnte nicht gestartet werden",
            "Avvio di esptool non riuscito")
    }
    pub fn esptool_failed(&self, status: &str) -> String {
        tr!(self,
            format!("Le flashage par esptool a échoué ({})", status),
            format!("Flashing with esptool failed ({})", status),
            format!("Flash mit esptool fehlgeschlagen ({})", status),
            format!("Flash con esptool non riuscito ({})", status))
    }

    // --- GitHub downloads --------------------------------------------------------------------------------------
    pub fn github_request_failed(&self) -> &'static str {
        tr!(self,
            "Échec de la requête vers GitHub",
            "The request to GitHub failed",
            "Die Anfrage an GitHub ist fehlgeschlagen",
            "Richiesta a GitHub non riuscita")
    }
    pub fn github_decode_failed(&self) -> &'static str {
        tr!(self,
            "Impossible de lire la réponse de GitHub",
            "Cannot read GitHub's answer",
            "Die Antwort von GitHub kann nicht gelesen werden",
            "Impossibile leggere la risposta di GitHub")
    }
    pub fn no_stable_release(&self) -> &'static str {
        tr!(self,
            "Aucune version stable trouvée",
            "No stable release found",
            "Kein stabiles Release gefunden",
            "Nessuna versione stabile trovata")
    }
    pub fn download_error(&self, url: &str, error: &str) -> String {
        tr!(self,
            format!("Erreur lors du téléchargement de {} : {}", url, error),
            format!("Error while downloading {}: {}", url, error),
            format!("Fehler beim Herunterladen von {}: {}", url, error),
            format!("Errore durante il download di {}: {}", url, error))
    }
    pub fn cannot_create_file(&self) -> &'static str {
        tr!(self,
            "Impossible de créer le fichier de destination",
            "Cannot create the destination file",
            "Die Zieldatei kann nicht erstellt werden",
            "Impossibile creare il file di destinazione")
    }
    pub fn download_done(&self) -> &'static str {
        tr!(self, "Téléchargement terminé", "Download complete", "Download abgeschlossen", "Download completato")
    }
    pub fn checksums_download_failed(&self) -> &'static str {
        tr!(self,
            "Impossible de télécharger le fichier SHA256SUMS",
            "Cannot download the SHA256SUMS file",
            "Die Datei SHA256SUMS kann nicht heruntergeladen werden",
            "Impossibile scaricare il file SHA256SUMS")
    }
    pub fn signature_download_failed(&self) -> &'static str {
        tr!(self,
            "Impossible de télécharger la signature SHA256SUMS.minisig",
            "Cannot download the SHA256SUMS.minisig signature",
            "Die Signatur SHA256SUMS.minisig kann nicht heruntergeladen werden",
            "Impossibile scaricare la firma SHA256SUMS.minisig")
    }
    pub fn invalid_public_key(&self, error: &str) -> String {
        tr!(self,
            format!("Clé publique Minisign invalide : {}", error),
            format!("Invalid Minisign public key: {}", error),
            format!("Ungültiger öffentlicher Minisign-Schlüssel: {}", error),
            format!("Chiave pubblica Minisign non valida: {}", error))
    }
    pub fn invalid_signature(&self, error: &str) -> String {
        tr!(self,
            format!("Format de signature Minisign invalide : {}", error),
            format!("Invalid Minisign signature format: {}", error),
            format!("Ungültiges Minisign-Signaturformat: {}", error),
            format!("Formato della firma Minisign non valido: {}", error))
    }
    pub fn signature_check_failed(&self, tag: &str, error: &str) -> String {
        tr!(self,
            format!("Échec de la vérification de la signature Minisign pour la version {} : {}", tag, error),
            format!("Minisign signature verification failed for release {}: {}", tag, error),
            format!("Überprüfung der Minisign-Signatur für Release {} fehlgeschlagen: {}", tag, error),
            format!("Verifica della firma Minisign non riuscita per la versione {}: {}", tag, error))
    }
    pub fn checksum_mismatch(&self, hash: &str) -> String {
        tr!(self,
            format!("Échec de la validation SHA256 ! Le fichier téléchargé ne correspond pas à la somme de contrôle officielle ({})", hash),
            format!("SHA256 validation failed! The downloaded file does not match the official checksum ({})", hash),
            format!("SHA256-Prüfung fehlgeschlagen! Die heruntergeladene Datei stimmt nicht mit der offiziellen Prüfsumme überein ({})", hash),
            format!("Convalida SHA256 non riuscita! Il file scaricato non corrisponde al checksum ufficiale ({})", hash))
    }
    pub fn release_not_found(&self, tag: &str) -> String {
        tr!(self,
            format!("Version {} introuvable sur GitHub", tag),
            format!("Release {} not found on GitHub", tag),
            format!("Release {} auf GitHub nicht gefunden", tag),
            format!("Versione {} non trovata su GitHub", tag))
    }
    pub fn no_update_binary(&self) -> &'static str {
        tr!(self,
            "Aucun fichier de mise à jour trouvé dans cette version",
            "No update file found in this release",
            "Keine Update-Datei in diesem Release gefunden",
            "Nessun file di aggiornamento trovato in questa versione")
    }
    pub fn no_factory_binary(&self) -> &'static str {
        tr!(self,
            "Aucune image complète (factory) trouvée dans cette version",
            "No full image (factory) found in this release",
            "Kein vollständiges Image (factory) in diesem Release gefunden",
            "Nessuna immagine completa (factory) trovata in questa versione")
    }
    pub fn downloading_verifying(&self) -> &'static str {
        tr!(self,
            "Téléchargement et vérification de la signature...",
            "Downloading and verifying the signature...",
            "Herunterladen und Überprüfen der Signatur...",
            "Download e verifica della firma...")
    }
    pub fn choose_version_or_file(&self) -> &'static str {
        tr!(self,
            "Veuillez choisir une version ou un fichier local",
            "Please choose a release or a local file",
            "Bitte wählen Sie ein Release oder eine lokale Datei",
            "Scegli una versione o un file locale")
    }
    pub fn github_unreachable(&self, error: &str) -> String {
        tr!(self,
            format!("Impossible de contacter GitHub : {}", error),
            format!("Cannot reach GitHub: {}", error),
            format!("GitHub ist nicht erreichbar: {}", error),
            format!("Impossibile contattare GitHub: {}", error))
    }
    pub fn badge_beta(&self) -> &'static str {
        tr!(self, "[BÊTA/TEST]", "[BETA/TEST]", "[BETA/TEST]", "[BETA/TEST]")
    }
    pub fn badge_stable_tag(&self) -> &'static str {
        tr!(self, "[STABLE]", "[STABLE]", "[STABIL]", "[STABILE]")
    }

    // --- window results ----------------------------------------------------------------------------------------
    pub fn usb_flashing_in_progress(&self) -> &'static str {
        tr!(self,
            "Flashage en cours sur le port USB...",
            "Flashing over the USB port...",
            "Flash über den USB-Port läuft...",
            "Flash in corso sulla porta USB...")
    }
    pub fn usb_done_restarting(&self) -> &'static str {
        tr!(self,
            "Flashage terminé avec succès ! La clé redémarre.",
            "Flashing completed successfully! The dongle is restarting.",
            "Flash erfolgreich abgeschlossen! Der Dongle startet neu.",
            "Flash completato con successo! Il dongle si sta riavviando.")
    }
    pub fn ota_sending_wifi(&self) -> &'static str {
        tr!(self,
            "Envoi du firmware via Wi-Fi...",
            "Sending the firmware over Wi-Fi...",
            "Firmware wird über WLAN gesendet...",
            "Invio del firmware tramite Wi-Fi...")
    }
    pub fn ota_success(&self) -> &'static str {
        tr!(self,
            "Mise à jour réussie ! La clé a redémarré et est de nouveau en ligne.",
            "Update successful! The dongle restarted and is back online.",
            "Update erfolgreich! Der Dongle wurde neu gestartet und ist wieder online.",
            "Aggiornamento riuscito! Il dongle si è riavviato ed è di nuovo online.")
    }
    pub fn wifi_config_sent(&self) -> &'static str {
        tr!(self,
            "Configuration Wi-Fi envoyée avec succès !",
            "Wi-Fi configuration sent successfully!",
            "WLAN-Konfiguration erfolgreich gesendet!",
            "Configurazione Wi-Fi inviata con successo!")
    }
    pub fn cannot_open_browser(&self, error: &str) -> String {
        tr!(self,
            format!("Impossible d'ouvrir le navigateur : {}", error),
            format!("Cannot open the browser: {}", error),
            format!("Der Browser kann nicht geöffnet werden: {}", error),
            format!("Impossibile aprire il browser: {}", error))
    }

    // --- discovery ---------------------------------------------------------------------------------------------
    pub fn version_unknown(&self) -> &'static str {
        tr!(self, "Inconnue", "Unknown", "Unbekannt", "Sconosciuta")
    }
    /// Label of the stove's main state number, as the bridge reports it.
    pub fn stove_state(&self, code: i64) -> &'static str {
        match code {
            0 => "Standby",
            1 => tr!(self, "Allumage", "Ignition", "Zündung", "Accensione"),
            2 => tr!(self, "Démarrage", "Start-up", "Anlauf", "Avvio"),
            3 => tr!(self, "Régulation", "Regulation", "Regelung", "Regolazione"),
            4 => tr!(self, "Nettoyage", "Cleaning", "Reinigung", "Pulizia"),
            5 => tr!(self, "Arrêt", "Shutdown", "Ausbrand", "Spegnimento"),
            _ => self.unknown(),
        }
    }
    pub fn scanning_subnet(&self) -> &'static str {
        tr!(self,
            "🔍 Scan du sous-réseau :",
            "🔍 Scanning subnet:",
            "🔍 Subnetz wird gescannt:",
            "🔍 Scansione della sottorete:")
    }
    pub fn dongle_summary(&self, ip: &str, model: &str, version: &str) -> String {
        tr!(self,
            format!("{} (Poêle {}, version {})", ip, model, version),
            format!("{} (Stove {}, version {})", ip, model, version),
            format!("{} (Ofen {}, Version {})", ip, model, version),
            format!("{} (Stufa {}, versione {})", ip, model, version))
    }

    pub fn megabyte_unit(&self) -> &'static str {
        tr!(self, "Mo", "MB", "MB", "MB")
    }

    // --- language menu -----------------------------------------------------------------------------------------
    pub fn switch_language_choice(&self) -> &'static str {
        tr!(self,
            "🌐 8. Changer de langue / Switch Language",
            "🌐 8. Switch Language / Changer de langue",
            "🌐 8. Sprache wechseln / Switch Language",
            "🌐 8. Cambia lingua / Switch Language")
    }
}

#[cfg(test)]
mod tests {
    use super::CliLang;

    const ALL: [CliLang; 4] = [CliLang::Fr, CliLang::En, CliLang::De, CliLang::It];

    #[test]
    fn language_names_are_recognised() {
        assert_eq!(CliLang::from_str("it"), CliLang::It);
        assert_eq!(CliLang::from_str("Italiano"), CliLang::It);
        assert_eq!(CliLang::from_str("de"), CliLang::De);
        assert_eq!(CliLang::from_str("en"), CliLang::En);
        assert_eq!(CliLang::from_str("xx"), CliLang::Fr);
        assert_eq!(CliLang::from_window(Some("it")), CliLang::It);
        assert_eq!(CliLang::from_window(Some("fr")), CliLang::Fr);
        assert_eq!(CliLang::from_window(Some("es")), CliLang::En);
        assert_eq!(CliLang::from_window(None), CliLang::En);
    }

    #[test]
    fn texts_carry_their_values_in_every_language() {
        for lang in ALL {
            assert!(lang.ota_slot_too_small(1_348_720, 1_310_720).contains("1348720"));
            assert!(lang.ota_slot_too_small(1_348_720, 1_310_720).contains("1310720"));
            assert!(lang.ota_no_tcp_connection(55499, 10, None).contains("55499"));
            assert!(!lang.ota_no_tcp_connection(55499, 10, None).contains("\n\n"));
            assert!(lang.ota_no_tcp_connection(55499, 1_348_720, Some(1_310_720)).contains("1310720"));
            assert!(lang.usb_flash_failed("boom").contains("boom"));
            assert!(lang.dongle_summary("192.168.1.5", "DOMO", "3.6.0").contains("DOMO"));
            assert_eq!(lang.menu_choices().len(), 7);
            assert!(!lang.stove_state(3).is_empty());
            assert_eq!(lang.stove_state(42), lang.unknown());
        }
    }
}
