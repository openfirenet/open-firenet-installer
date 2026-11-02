#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CliLang {
    #[default]
    Fr,
    En,
    De,
    It,
    Es,
}

impl CliLang {
    pub fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "en" | "english" => CliLang::En,
            "de" | "deutsch" | "german" => CliLang::De,
            "it" | "italiano" | "italian" => CliLang::It,
            "es" | "español" | "spanish" => CliLang::Es,
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
            } else if l.starts_with("es") {
                return CliLang::Es;
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
            CliLang::Es => "Asistente multiplataforma de actualización y flasheo USB",
        }
    }

    pub fn menu_title(&self) -> &'static str {
        match self {
            CliLang::Fr => "Que souhaitez-vous faire ?",
            CliLang::En => "What would you like to do?",
            CliLang::De => "Was möchten Sie tun?",
            CliLang::It => "Cosa vuoi fare?",
            CliLang::Es => "¿Qué te gustaría hacer?",
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
            CliLang::Es => [
                "🔍 1. Escanear la red local (descubrir el pincho y estado de la estufa)",
                "⚡ 2. Flashear el pincho vía USB (primer flash / reinstalación)",
                "📡 3. Actualizar el pincho remotamente vía Wi-Fi (OTA)",
                "📶 4. Configurar el Wi-Fi del pincho (vía USB Serie)",
                "📦 5. Consultar versiones de GitHub (releases & pre-releases)",
                "📟 6. Monitor Serie (ver logs de la estufa en vivo)",
                "🚪 7. Salir",
            ],
        }
    }

    pub fn menu_return_prompt(&self) -> &'static str {
        match self {
            CliLang::Fr => "Revenir au menu principal ?",
            CliLang::En => "Return to main menu?",
            CliLang::De => "Zurück zum Hauptmenü?",
            CliLang::It => "Tornare al menu principale?",
            CliLang::Es => "¿Volver al menú principal?",
        }
    }

    pub fn goodbye(&self) -> &'static str {
        match self {
            CliLang::Fr => "Au revoir !",
            CliLang::En => "Goodbye!",
            CliLang::De => "Auf Wiedersehen!",
            CliLang::It => "Arrivederci!",
            CliLang::Es => "¡Adiós!",
        }
    }

    pub fn scan_searching(&self) -> &'static str {
        match self {
            CliLang::Fr => "🔍 Recherche de la clé Open Firenet sur votre réseau...",
            CliLang::En => "🔍 Searching for Open Firenet dongle on your network...",
            CliLang::De => "🔍 Open Firenet-Dongle wird im Netzwerk gesucht...",
            CliLang::It => "🔍 Ricerca del dongle Open Firenet sulla tua rete...",
            CliLang::Es => "🔍 Buscando el pincho Open Firenet en tu red...",
        }
    }

    pub fn scan_not_found(&self) -> &'static str {
        match self {
            CliLang::Fr => "❌ Aucune clé Open Firenet détectée sur le réseau.",
            CliLang::En => "❌ No Open Firenet dongle detected on the network.",
            CliLang::De => "❌ Kein Open Firenet-Dongle im Netzwerk gefunden.",
            CliLang::It => "❌ Nessun dongle Open Firenet rilevato sulla rete.",
            CliLang::Es => "❌ No se detectó el pincho Open Firenet en la red.",
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
            CliLang::Es => [
                "Consejos:",
                "  - Verifica que el pincho esté encendido y conectado al Wi-Fi.",
                "  - Si es el primer arranque, conéctate al punto de acceso Wi-Fi 'Open-Firenet-Setup'.",
                "  - O conéctalo vía USB para realizar el primer flasheo.",
            ],
        }
    }

    pub fn scan_found(&self, count: usize) -> String {
        match self {
            CliLang::Fr => format!("✔ {} clé(s) Open Firenet trouvée(s) :", count),
            CliLang::En => format!("✔ {} Open Firenet dongle(s) found:", count),
            CliLang::De => format!("✔ {} Open Firenet-Dongle(s) gefunden:", count),
            CliLang::It => format!("✔ {} dongle Open Firenet trovato/i:", count),
            CliLang::Es => format!("✔ {} pincho Open Firenet encontrado(s):", count),
        }
    }

    pub fn label_ip(&self) -> &'static str {
        match self {
            CliLang::Fr => "Adresse IP",
            CliLang::En => "IP Address",
            CliLang::De => "IP-Adresse",
            CliLang::It => "Indirizzo IP",
            CliLang::Es => "Dirección IP",
        }
    }

    pub fn label_hostname(&self) -> &'static str {
        match self {
            CliLang::Fr => "Nom d'hôte",
            CliLang::En => "Hostname",
            CliLang::De => "Hostname",
            CliLang::It => "Nome host",
            CliLang::Es => "Nombre de host",
        }
    }

    pub fn label_stove_model(&self) -> &'static str {
        match self {
            CliLang::Fr => "Modèle poêle",
            CliLang::En => "Stove model",
            CliLang::De => "Ofenmodell",
            CliLang::It => "Modello stufa",
            CliLang::Es => "Modelo de estufa",
        }
    }

    pub fn label_stove_state(&self) -> &'static str {
        match self {
            CliLang::Fr => "État actuel",
            CliLang::En => "Current state",
            CliLang::De => "Aktueller Status",
            CliLang::It => "Stato attuale",
            CliLang::Es => "Estado actual",
        }
    }

    pub fn label_wifi_signal(&self) -> &'static str {
        match self {
            CliLang::Fr => "Signal Wi-Fi",
            CliLang::En => "Wi-Fi Signal",
            CliLang::De => "WLAN-Signal",
            CliLang::It => "Segnale Wi-Fi",
            CliLang::Es => "Señal Wi-Fi",
        }
    }

    pub fn label_firmware_version(&self) -> &'static str {
        match self {
            CliLang::Fr => "Version firmw.",
            CliLang::En => "Firmware ver.",
            CliLang::De => "Firmware-Vers.",
            CliLang::It => "Vers. firmware",
            CliLang::Es => "Vers. firmware",
        }
    }

    pub fn label_web_access(&self) -> &'static str {
        match self {
            CliLang::Fr => "Accès Web",
            CliLang::En => "Web Access",
            CliLang::De => "Web-Zugriff",
            CliLang::It => "Accesso web",
            CliLang::Es => "Acceso web",
        }
    }

    pub fn flash_usb_title(&self) -> &'static str {
        match self {
            CliLang::Fr => "⚡ Flashage USB série de la clé Open Firenet",
            CliLang::En => "⚡ USB Serial Flashing of Open Firenet dongle",
            CliLang::De => "⚡ USB-Serieller Flash des Open Firenet-Dongles",
            CliLang::It => "⚡ Flash USB seriale del dongle Open Firenet",
            CliLang::Es => "⚡ Flasheo USB serie del dongle Open Firenet",
        }
    }

    pub fn select_serial_port(&self) -> &'static str {
        match self {
            CliLang::Fr => "Sélectionnez le port série USB de votre ESP32-S3 :",
            CliLang::En => "Select the USB serial port of your ESP32-S3:",
            CliLang::De => "Wählen Sie den USB-Seriell-Port Ihres ESP32-S3:",
            CliLang::It => "Seleziona la porta seriale USB del tuo ESP32-S3:",
            CliLang::Es => "Selecciona el puerto serie USB de tu ESP32-S3:",
        }
    }

    pub fn no_serial_port_found(&self) -> &'static str {
        match self {
            CliLang::Fr => "❌ Aucun port série USB détecté. Branchez votre ESP32-S3 en USB.",
            CliLang::En => "❌ No USB serial port detected. Plug in your ESP32-S3 via USB.",
            CliLang::De => "❌ Kein USB-Seriell-Port erkannt. Bitte ESP32-S3 per USB anschließen.",
            CliLang::It => "❌ Nessuna porta seriale USB rilevata. Collega il tuo ESP32-S3 tramite USB.",
            CliLang::Es => "❌ No se detectó ningún puerto serie USB. Conecta tu ESP32-S3 vía USB.",
        }
    }

    pub fn usb_native_hint(&self) -> &'static str {
        match self {
            CliLang::Fr => "💡 Cartes à USB natif (M5Stamp S3, XIAO ESP32-S3) : si la connexion échoue, maintenez le bouton BOOT enfoncé lors du branchement USB.",
            CliLang::En => "💡 Native USB boards (M5Stamp S3, XIAO ESP32-S3): if connection fails, hold down the BOOT button while plugging in the USB cable.",
            CliLang::De => "💡 Boards mit nativem USB (M5Stamp S3, XIAO ESP32-S3): falls Verbindung fehlschlägt, halten Sie beim Einstecken die BOOT-Taste gedrückt.",
            CliLang::It => "💡 Schede con USB nativo (M5Stamp S3, XIAO ESP32-S3): se la connessione non riesce, tieni premuto il pulsante BOOT mentre colleghi il cavo USB.",
            CliLang::Es => "💡 Placas con USB nativo (M5Stamp S3, XIAO ESP32-S3): si la conexión falla, mantén presionado el botón BOOT mientras conectas el cable USB.",
        }
    }

    pub fn usb_post_flash_hint(&self) -> &'static str {
        match self {
            CliLang::Fr => "💡 Note : Sur les cartes à USB natif, appuyez sur le bouton RESET pour lancer le nouveau firmware.",
            CliLang::En => "💡 Note: On native USB boards, press the RESET button to start the new firmware.",
            CliLang::De => "💡 Hinweis: Drücken Sie bei Boards mit nativem USB die RESET-Taste, um die neue Firmware zu starten.",
            CliLang::It => "💡 Nota: sulle schede con USB nativo, premi il pulsante RESET per avviare il nuovo firmware.",
            CliLang::Es => "💡 Nota: en las placas con USB nativo, presiona el botón RESET para iniciar el nuevo firmware.",
        }
    }

    pub fn ota_updating_to(&self, ip: &str) -> String {
        match self {
            CliLang::Fr => format!("📡 Mise à jour Wi-Fi (OTA) vers {}", ip),
            CliLang::En => format!("📡 Wi-Fi Update (OTA) to {}", ip),
            CliLang::De => format!("📡 WLAN-Update (OTA) an {}", ip),
            CliLang::It => format!("📡 Aggiornamento Wi-Fi (OTA) verso {}", ip),
            CliLang::Es => format!("📡 Actualización Wi-Fi (OTA) a {}", ip),
        }
    }

    pub fn select_dongle_to_update(&self) -> &'static str {
        match self {
            CliLang::Fr => "Sélectionnez la clé à mettre à jour :",
            CliLang::En => "Select the dongle to update:",
            CliLang::De => "Wählen Sie den zu aktualisierenden Dongle:",
            CliLang::It => "Seleziona il dongle da aggiornare:",
            CliLang::Es => "Selecciona el pincho a actualizar:",
        }
    }

    pub fn enter_ip_prompt(&self) -> &'static str {
        match self {
            CliLang::Fr => "Entrez l'adresse IP de votre clé Open Firenet",
            CliLang::En => "Enter the IP address of your Open Firenet dongle",
            CliLang::De => "Geben Sie die IP-Adresse des Open Firenet-Dongles ein",
            CliLang::It => "Inserisci l'indirizzo IP del tuo dongle Open Firenet",
            CliLang::Es => "Introduce la dirección IP de tu pincho Open Firenet",
        }
    }

    pub fn fetching_releases(&self) -> &'static str {
        match self {
            CliLang::Fr => "Récupération des versions disponibles sur GitHub...",
            CliLang::En => "Fetching available releases from GitHub...",
            CliLang::De => "Verfügbare Releases von GitHub werden abgerufen...",
            CliLang::It => "Recupero delle versioni disponibili su GitHub...",
            CliLang::Es => "Recuperando las versiones disponibles de GitHub...",
        }
    }

    pub fn no_releases_found(&self) -> &'static str {
        match self {
            CliLang::Fr => "Aucune release publiée sur GitHub pour le moment.",
            CliLang::En => "No releases published on GitHub yet.",
            CliLang::De => "Bisher keine offiziellen Releases auf GitHub veröffentlicht.",
            CliLang::It => "Nessuna release pubblicata su GitHub per ora.",
            CliLang::Es => "No hay releases publicadas en GitHub todavía.",
        }
    }

    pub fn enter_bin_path_prompt(&self) -> &'static str {
        match self {
            CliLang::Fr => "Chemin vers votre fichier binaire .bin local",
            CliLang::En => "Path to your local .bin binary file",
            CliLang::De => "Pfad zu Ihrer lokalen .bin-Binärdatei",
            CliLang::It => "Percorso del tuo file binario .bin locale",
            CliLang::Es => "Ruta a tu archivo binario .bin local",
        }
    }

    pub fn choose_version_prompt(&self) -> &'static str {
        match self {
            CliLang::Fr => "Choisissez la version à installer :",
            CliLang::En => "Choose the version to install:",
            CliLang::De => "Wählen Sie die zu installierende Version:",
            CliLang::It => "Scegli la versione da installare:",
            CliLang::Es => "Elige la versión a instalar:",
        }
    }

    pub fn badge_prerelease(&self) -> &'static str {
        match self {
            CliLang::Fr => " (pré-release/test)",
            CliLang::En => " (pre-release/test)",
            CliLang::De => " (Pre-Release/Test)",
            CliLang::It => " (pre-release/test)",
            CliLang::Es => " (pre-release/test)",
        }
    }

    pub fn badge_stable(&self) -> &'static str {
        match self {
            CliLang::Fr => " (stable)",
            CliLang::En => " (stable)",
            CliLang::De => " (stabil)",
            CliLang::It => " (stabile)",
            CliLang::Es => " (estable)",
        }
    }

    pub fn no_binary_found(&self) -> &'static str {
        match self {
            CliLang::Fr => "Aucun binaire précompilé approprié trouvé dans cette release.",
            CliLang::En => "No suitable precompiled binary found in this release.",
            CliLang::De => "Keine passende vorkompilierte Binärdatei in diesem Release gefunden.",
            CliLang::It => "Nessun binario precompilato adatto trovato in questa release.",
            CliLang::Es => "No se encontró ningún binario precompilado adecuado en esta release.",
        }
    }

    pub fn use_cached_prompt(&self, filename: &str) -> String {
        match self {
            CliLang::Fr => format!("Utiliser la version en cache ({}) ?", filename),
            CliLang::En => format!("Use cached version ({})?", filename),
            CliLang::De => format!("Version aus dem Cache verwenden ({})?", filename),
            CliLang::It => format!("Usare la versione in cache ({})?", filename),
            CliLang::Es => format!("¿Usar la versión en caché ({})?", filename),
        }
    }

    pub fn downloading_asset(&self, name: &str) -> String {
        match self {
            CliLang::Fr => format!("Téléchargement et vérification cryptographique de {}...", name),
            CliLang::En => format!("Downloading and cryptographically verifying {}...", name),
            CliLang::De => format!("Herunterladen und kryptografische Überprüfung von {}...", name),
            CliLang::It => format!("Download e verifica crittografica di {}...", name),
            CliLang::Es => format!("Descargando y verificando criptográficamente {}...", name),
        }
    }

    pub fn verification_ok(&self) -> &'static str {
        match self {
            CliLang::Fr => "✔ Signature Minisign et intégrité SHA256 validées avec succès !",
            CliLang::En => "✔ Minisign signature and SHA256 integrity successfully verified!",
            CliLang::De => "✔ Minisign-Signatur und SHA256-Integrität erfolgreich überprüft!",
            CliLang::It => "✔ Firma Minisign e integrità SHA256 verificate con successo!",
            CliLang::Es => "✔ ¡Firma Minisign e integridad SHA256 verificadas con éxito!",
        }
    }

    pub fn releases_title(&self) -> &'static str {
        match self {
            CliLang::Fr => "📦 Versions d'Open Firenet publiées sur GitHub :",
            CliLang::En => "📦 Open Firenet releases published on GitHub:",
            CliLang::De => "📦 Auf GitHub veröffentlichte Open Firenet-Releases:",
            CliLang::It => "📦 Versioni di Open Firenet pubblicate su GitHub:",
            CliLang::Es => "📦 Versiones de Open Firenet publicadas en GitHub:",
        }
    }

    pub fn wifi_setup_title(&self) -> &'static str {
        match self {
            CliLang::Fr => "📶 Configuration Wi-Fi du dongle Open Firenet",
            CliLang::En => "📶 Open Firenet Dongle Wi-Fi Configuration",
            CliLang::De => "📶 WLAN-Konfiguration des Open Firenet-Dongles",
            CliLang::It => "📶 Configurazione Wi-Fi del dongle Open Firenet",
            CliLang::Es => "📶 Configuración Wi-Fi del dongle Open Firenet",
        }
    }

    pub fn wifi_setup_desc(&self) -> &'static str {
        match self {
            CliLang::Fr => "Ces informations permettront à la clé de se connecter à votre réseau local.",
            CliLang::En => "This information will allow the dongle to connect to your local network.",
            CliLang::De => "Diese Daten ermöglichen dem Dongle die Verbindung mit Ihrem lokalen Netzwerk.",
            CliLang::It => "Queste informazioni permetteranno al dongle di connettersi alla tua rete locale.",
            CliLang::Es => "Esta información permitirá que el dongle se conecte a tu red local.",
        }
    }

    pub fn wifi_native_hint(&self) -> &'static str {
        match self {
            CliLang::Fr => "💡 Astuce : Sur les cartes sans puce UART dédiée, vous pouvez aussi vous connecter au point d'accès Wi-Fi « Open-Firenet-Setup » (192.168.4.1) pour configurer le réseau.",
            CliLang::En => "💡 Tip: On boards without a dedicated UART chip, you can also connect to the \"Open-Firenet-Setup\" Wi-Fi access point (192.168.4.1) to configure the network.",
            CliLang::De => "💡 Tipp: Auf Boards ohne dedizierten UART-Chip können Sie das Netzwerk auch über den WLAN-Access-Point „Open-Firenet-Setup“ (192.168.4.1) einrichten.",
            CliLang::It => "💡 Suggerimento: sulle schede senza chip UART dedicato, puoi anche connetterti al punto di accesso Wi-Fi «Open-Firenet-Setup» (192.168.4.1) per configurare la rete.",
            CliLang::Es => "💡 Consejo: en las placas sin un chip UART dedicado, también puedes conectarte al punto de acceso Wi-Fi «Open-Firenet-Setup» (192.168.4.1) para configurar la red.",
        }
    }

    pub fn wifi_ssid_prompt(&self) -> &'static str {
        match self {
            CliLang::Fr => "Nom de votre réseau Wi-Fi (SSID)",
            CliLang::En => "Your Wi-Fi Network Name (SSID)",
            CliLang::De => "WLAN-Netzwerkname (SSID)",
            CliLang::It => "Nome della tua rete Wi-Fi (SSID)",
            CliLang::Es => "Nombre de tu red Wi-Fi (SSID)",
        }
    }

    pub fn wifi_pass_prompt(&self) -> &'static str {
        match self {
            CliLang::Fr => "Mot de passe Wi-Fi (WPA2/WPA3)",
            CliLang::En => "Wi-Fi Password (WPA2/WPA3)",
            CliLang::De => "WLAN-Passwort (WPA2/WPA3)",
            CliLang::It => "Password Wi-Fi (WPA2/WPA3)",
            CliLang::Es => "Contraseña Wi-Fi (WPA2/WPA3)",
        }
    }

    pub fn wifi_sending(&self, port: &str) -> String {
        match self {
            CliLang::Fr => format!("Envoi de la configuration sur {}...", port),
            CliLang::En => format!("Sending configuration to {}...", port),
            CliLang::De => format!("Konfiguration wird an {} gesendet...", port),
            CliLang::It => format!("Invio della configurazione su {}...", port),
            CliLang::Es => format!("Enviando configuración a {}...", port),
        }
    }

    pub fn wifi_sent_success(&self) -> &'static str {
        match self {
            CliLang::Fr => "Identifiants Wi-Fi enregistrés dans la clé ! Vous pouvez la débrancher et la brancher sur votre poêle.",
            CliLang::En => "Wi-Fi credentials saved to the dongle! You can now disconnect it and plug it into your stove.",
            CliLang::De => "WLAN-Zugangsdaten auf dem Dongle gespeichert! Sie können ihn jetzt abziehen und an Ihren Ofen anschließen.",
            CliLang::It => "Credenziali Wi-Fi salvate nel dongle! Ora puoi scollegarlo e collegarlo alla tua stufa.",
            CliLang::Es => "¡Credenciales Wi-Fi guardadas en el dongle! Ahora puedes desconectarlo y conectarlo a tu estufa.",
        }
    }

    pub fn record_unreachable(&self, ip: &str, err: &str) -> String {
        match self {
            CliLang::Fr => format!("La clé à l'adresse {} ne répond pas ({}). Vérifiez l'adresse et que la clé est allumée.", ip, err),
            CliLang::En => format!("The dongle at {} does not answer ({}). Check the address and that the dongle is powered.", ip, err),
            CliLang::De => format!("Der Dongle unter {} antwortet nicht ({}). Prüfen Sie die Adresse und ob der Dongle eingeschaltet ist.", ip, err),
            CliLang::It => format!("Il dongle all'indirizzo {} non risponde ({}). Verifica l'indirizzo e che il dongle sia acceso.", ip, err),
            CliLang::Es => format!("El dongle en {} no responde ({}). Verifica la dirección y que el dongle esté encendido.", ip, err),
        }
    }
    pub fn record_cannot_write(&self, path: &str) -> String {
        match self {
            CliLang::Fr => format!("Impossible d'écrire le fichier {}", path),
            CliLang::En => format!("Cannot write the file {}", path),
            CliLang::De => format!("Die Datei {} kann nicht geschrieben werden", path),
            CliLang::It => format!("Impossibile scrivere il file {}", path),
            CliLang::Es => format!("No se puede escribir el archivo {}", path),
        }
    }
    pub fn record_started(&self, path: &str, minutes: u64) -> String {
        match self {
            CliLang::Fr => format!("⏺  Enregistrement du cycle de chauffe dans {} (pendant {} min au plus, Ctrl+C pour arrêter).", path, minutes),
            CliLang::En => format!("⏺  Recording the heating cycle to {} (for {} min at most, Ctrl+C to stop).", path, minutes),
            CliLang::De => format!("⏺  Der Heizzyklus wird in {} aufgezeichnet (höchstens {} Min., Strg+C zum Beenden).", path, minutes),
            CliLang::It => format!("⏺  Registrazione del ciclo di riscaldamento in {} (al massimo {} min, Ctrl+C per fermare).", path, minutes),
            CliLang::Es => format!("⏺  Grabando el ciclo de calentamiento en {} (por lo menos {} min, Ctrl+C para detener).", path, minutes),
        }
    }
    pub fn record_already_running(&self) -> &'static str {
        match self {
            CliLang::Fr => "Un enregistrement est déjà en cours.",
            CliLang::En => "A recording is already running.",
            CliLang::De => "Es läuft bereits eine Aufzeichnung.",
            CliLang::It => "Una registrazione è già in corso.",
            CliLang::Es => "Ya hay una grabación en curso.",
        }
    }
    pub fn record_hint(&self) -> &'static str {
        match self {
            CliLang::Fr => "Le poêle est seulement lu, rien ne lui est envoyé. Gardez cet ordinateur allumé. Pour noter ce que vous voyez (par exemple ce qu'affiche l'écran du poêle), tapez une ligne puis Entrée.",
            CliLang::En => "The stove is only read, nothing is sent to it. Keep this computer awake. To note what you see (for example what the stove's screen shows), type a line and press Enter.",
            CliLang::De => "Der Ofen wird nur gelesen, es wird nichts an ihn gesendet. Lassen Sie diesen Computer eingeschaltet. Um zu notieren, was Sie sehen (zum Beispiel die Anzeige des Ofens), tippen Sie eine Zeile und drücken Sie Enter.",
            CliLang::It => "La stufa viene solo letta, non le viene inviato nulla. Tieni acceso questo computer. Per annotare ciò che vedi (per esempio cosa mostra lo schermo della stufa), scrivi una riga e premi Invio.",
            CliLang::Es => "La estufa solo se lee, no se le envía nada. Mantén este ordenador encendido. Para anotar lo que ves (por ejemplo lo que muestra la pantalla de la estufa), escribe una línea y pulsa Enter.",
        }
    }
    pub fn record_done(&self, changes: u32, path: &str) -> String {
        match self {
            CliLang::Fr => format!("Enregistrement terminé : {} changement(s) notés. Fichier : {}\nVous pouvez nous l'envoyer : https://github.com/openfirenet/open-firenet/issues/new?template=heating_cycle.yml", changes, path),
            CliLang::En => format!("Recording finished: {} change(s) noted. File: {}\nYou can send it to us: https://github.com/openfirenet/open-firenet/issues/new?template=heating_cycle.yml", changes, path),
            CliLang::De => format!("Aufzeichnung beendet: {} Änderung(en) notiert. Datei: {}\nSie können sie uns senden: https://github.com/openfirenet/open-firenet/issues/new?template=heating_cycle.yml", changes, path),
            CliLang::It => format!("Registrazione terminata: {} cambiamento/i annotati. File: {}\nPuoi inviarcelo: https://github.com/openfirenet/open-firenet/issues/new?template=heating_cycle.yml", changes, path),
            CliLang::Es => format!("Grabación terminada: {} cambio(s) anotado(s). Archivo: {}\nPuedes enviárnoslo: https://github.com/openfirenet/open-firenet/issues/new?template=heating_cycle.yml", changes, path),
        }
    }

    pub fn monitor_opening(&self, port: &str, baud: u32) -> String {
        match self {
            CliLang::Fr => format!("📟 Ouverture du moniteur série sur {} à {} bauds (Ctrl+C pour quitter)...", port, baud),
            CliLang::En => format!("📟 Opening serial monitor on {} at {} baud (Ctrl+C to quit)...", port, baud),
            CliLang::De => format!("📟 Serieller Monitor wird auf {} mit {} Baud geöffnet (Strg+C zum Beenden)...", port, baud),
            CliLang::It => format!("📟 Apertura del monitor seriale su {} a {} baud (Ctrl+C per uscire)...", port, baud),
            CliLang::Es => format!("📟 Abriendo el monitor serie en {} a {} baudios (Ctrl+C para salir)...", port, baud),
        }
    }

    pub fn monitor_session_end(&self) -> &'static str {
        match self {
            CliLang::Fr => "Fin de session série :",
            CliLang::En => "Serial session ended:",
            CliLang::De => "Serielle Sitzung beendet:",
            CliLang::It => "Sessione seriale terminata:",
            CliLang::Es => "Sesión serie terminada:",
        }
    }
}

/// One text per language, in the order French, English, German, Italian, Spanish.
macro_rules! tr {
    ($lang:expr, $fr:expr, $en:expr, $de:expr, $it:expr, $es:expr $(,)?) => {
        match $lang {
            CliLang::Fr => $fr,
            CliLang::En => $en,
            CliLang::De => $de,
            CliLang::It => $it,
            CliLang::Es => $es,
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
            Some("es") => CliLang::Es,
            _ => CliLang::En,
        }
    }

    // --- files -------------------------------------------------------------------------------------------------
    pub fn file_not_found(&self, path: &std::path::Path) -> String {
        tr!(self,
            format!("Le fichier binaire n'existe pas : {:?}", path),
            format!("The binary file does not exist: {:?}", path),
            format!("Die Binärdatei existiert nicht: {:?}", path),
            format!("Il file binario non esiste: {:?}", path),
            format!("El archivo binario no existe: {:?}", path))
    }
    pub fn cannot_read_firmware(&self, path: &std::path::Path) -> String {
        tr!(self,
            format!("Impossible de lire le fichier firmware : {:?}", path),
            format!("Cannot read the firmware file: {:?}", path),
            format!("Die Firmware-Datei kann nicht gelesen werden: {:?}", path),
            format!("Impossibile leggere il file del firmware: {:?}", path),
            format!("No se puede leer el archivo del firmware: {:?}", path))
    }

    // --- wireless update ---------------------------------------------------------------------------------------
    pub fn ota_preparing(&self, ip: &str) -> String {
        tr!(self,
            format!("Préparation de la mise à jour sans fil vers {}...", ip),
            format!("Preparing the wireless update to {}...", ip),
            format!("Drahtloses Update an {} wird vorbereitet...", ip),
            format!("Preparazione dell'aggiornamento tramite Wi-Fi verso {}...", ip),
            format!("Preparando la actualización inalámbrica a {}...", ip))
    }
    pub fn reading_firmware(&self) -> &'static str {
        tr!(self,
            "Lecture et vérification du firmware...",
            "Reading and checking the firmware...",
            "Firmware wird gelesen und geprüft...",
            "Lettura e verifica del firmware...",
            "Leyendo y verificando el firmware...")
    }
    pub fn firmware_info(&self, bytes: usize, md5: &str) -> String {
        tr!(self,
            format!("Firmware : {} octets, MD5 : {}", bytes, md5),
            format!("Firmware: {} bytes, MD5: {}", bytes, md5),
            format!("Firmware: {} Bytes, MD5: {}", bytes, md5),
            format!("Firmware: {} byte, MD5: {}", bytes, md5),
            format!("Firmware: {} bytes, MD5: {}", bytes, md5))
    }
    pub fn ota_slot_too_small(&self, firmware_bytes: usize, slot_bytes: usize) -> String {
        tr!(self,
            format!("L'emplacement de mise à jour de cette clé fait {} octets et ce firmware en fait {}. Une mise à jour sans fil ne peut pas agrandir cet emplacement : flashez la clé une fois par USB en mode « Première installation », le seul qui réécrit la table de partitions (le mode « Mise à jour » la conserve). Le Wi-Fi sera à ressaisir, puis les mises à jour sans fil fonctionneront de nouveau.", slot_bytes, firmware_bytes),
            format!("This dongle's update slot is {} bytes and this firmware is {} bytes. A wireless update cannot enlarge the slot: flash the dongle once over USB in \"First installation\" mode, the only one that rewrites the partition table (the \"Update\" mode keeps it). You will have to enter the Wi-Fi again; wireless updates will work afterwards.", slot_bytes, firmware_bytes),
            format!("Der Update-Speicherplatz dieses Dongles hat {} Bytes, diese Firmware {} Bytes. Ein drahtloses Update kann den Speicherplatz nicht vergrößern: Flashen Sie den Dongle einmal per USB im Modus „Erstinstallation“, nur dieser schreibt die Partitionstabelle neu (der Modus „Update“ behält sie). Das WLAN muss danach neu eingegeben werden; drahtlose Updates funktionieren dann wieder.", slot_bytes, firmware_bytes),
            format!("Lo spazio di aggiornamento di questo dongle è di {} byte e questo firmware ne occupa {}. Un aggiornamento tramite Wi-Fi non può ampliare questo spazio: esegui una volta il flash del dongle tramite USB in modalità «Prima installazione», l'unica che riscrive la tabella delle partizioni (la modalità «Aggiornamento» la conserva). Dovrai reinserire il Wi-Fi; dopo, gli aggiornamenti tramite Wi-Fi funzioneranno di nuovo.", slot_bytes, firmware_bytes),
            format!("El espacio de actualización de este pincho es de {} bytes y este firmware ocupa {} bytes. Una actualización inalámbrica no puede ampliar el espacio: flashea el dongle una vez por USB en modo «Primera instalación», el único que reescribe la tabla de particiones (el modo «Actualización» la conserva). Tendrás que volver a introducir el Wi-Fi; después, las actualizaciones inalámbricas funcionarán de nuevo.", slot_bytes, firmware_bytes))
    }
    pub fn ota_tcp_bind_failed(&self) -> &'static str {
        tr!(self,
            "Impossible d'ouvrir le port d'écoute TCP local pour la mise à jour",
            "Cannot open the local TCP listening port for the update",
            "Der lokale TCP-Port für das Update kann nicht geöffnet werden",
            "Impossibile aprire la porta TCP locale per l'aggiornamento",
            "No se puede abrir el puerto TCP local para la actualización")
    }
    pub fn ota_tcp_listening(&self, port: u16) -> String {
        tr!(self,
            format!("Serveur TCP local en écoute sur le port {}", port),
            format!("Local TCP server listening on port {}", port),
            format!("Lokaler TCP-Server lauscht auf Port {}", port),
            format!("Server TCP locale in ascolto sulla porta {}", port),
            format!("Servidor TCP local escuchando en el puerto {}", port))
    }
    pub fn ota_udp_bind_failed(&self) -> &'static str {
        tr!(self,
            "Impossible d'ouvrir la socket UDP locale pour la mise à jour",
            "Cannot open the local UDP socket for the update",
            "Der lokale UDP-Socket für das Update kann nicht geöffnet werden",
            "Impossibile aprire il socket UDP locale per l'aggiornamento",
            "No se puede abrir el socket UDP local para la actualización")
    }
    pub fn invalid_ip(&self, ip: &str) -> String {
        tr!(self,
            format!("Adresse IP de la clé invalide : {}", ip),
            format!("Invalid dongle IP address: {}", ip),
            format!("Ungültige IP-Adresse des Dongles: {}", ip),
            format!("Indirizzo IP del dongle non valido: {}", ip),
            format!("Dirección IP del pincho no válida: {}", ip))
    }
    pub fn ota_negotiating(&self, ip: &str) -> String {
        tr!(self,
            format!("Négociation avec la clé ({}:3232)...", ip),
            format!("Negotiating with the dongle ({}:3232)...", ip),
            format!("Aushandlung mit dem Dongle ({}:3232)...", ip),
            format!("Negoziazione con il dongle ({}:3232)...", ip),
            format!("Negociando con el pincho ({}:3232)...", ip))
    }
    pub fn ota_connecting(&self) -> &'static str {
        tr!(self,
            "Connexion à la clé via Wi-Fi...",
            "Connecting to the dongle over Wi-Fi...",
            "Verbindung zum Dongle über WLAN...",
            "Connessione al dongle tramite Wi-Fi...",
            "Conectando al pincho a través de Wi-Fi...")
    }
    pub fn ota_password_required(&self) -> &'static str {
        tr!(self,
            "Cette clé demande son mot de passe de mise à jour.",
            "This dongle asks for its update password.",
            "Dieser Dongle verlangt sein Update-Passwort.",
            "Questo dongle richiede la sua password di aggiornamento.",
            "Este pincho solicita su contraseña de actualización.")
    }
    pub fn ota_password_wrong(&self) -> &'static str {
        tr!(self,
            "Mot de passe de mise à jour refusé par la clé. Si vous l'avez oublié, un flash USB en mode « Première installation » le supprime.",
            "The dongle refused the update password. If you forgot it, a USB flash in \"First installation\" mode removes it.",
            "Der Dongle hat das Update-Passwort abgelehnt. Wenn Sie es vergessen haben, entfernt ein USB-Flash im Modus „Erstinstallation“ es.",
            "Il dongle ha rifiutato la password di aggiornamento. Se l'hai dimenticata, un flash USB in modalità «Prima installazione» la rimuove.",
            "El pincho rechazó la contraseña de actualización. Si la olvidaste, un flash USB en modo «Primera instalación» la elimina.")
    }
    pub fn ota_auth_unsupported(&self) -> &'static str {
        tr!(self,
            "Cette clé utilise une ancienne forme d'authentification que l'installeur ne gère pas. Mettez-la à jour par USB.",
            "This dongle uses an older form of authentication the installer does not handle. Update it over USB.",
            "Dieser Dongle verwendet eine ältere Authentifizierung, die der Installer nicht unterstützt. Aktualisieren Sie ihn per USB.",
            "Questo dongle usa una vecchia forma di autenticazione che l'installer non gestisce. Aggiornalo tramite USB.",
            "Este pincho utiliza una forma antigua de autenticación que el instalador no maneja. Actualízalo por USB.")
    }
    pub fn ota_checking_password(&self) -> &'static str {
        tr!(self,
            "Vérification du mot de passe par la clé...",
            "The dongle is checking the password...",
            "Der Dongle prüft das Passwort...",
            "Il dongle sta verificando la password...",
            "El pincho está verificando la contraseña...")
    }
    pub fn ota_password_prompt(&self) -> &'static str {
        tr!(self,
            "Mot de passe de mise à jour de la clé",
            "Update password of the dongle",
            "Update-Passwort des Dongles",
            "Password di aggiornamento del dongle",
            "Contraseña de actualización del pincho")
    }
    pub fn ota_password_set_prompt(&self) -> &'static str {
        tr!(self,
            "Nouveau mot de passe de mise à jour (vide pour le retirer)",
            "New update password (empty to remove it)",
            "Neues Update-Passwort (leer, um es zu entfernen)",
            "Nuova password di aggiornamento (vuota per rimuoverla)",
            "Nueva contraseña de actualización (vacía para eliminarla)")
    }
    pub fn wifi_not_confirmed(&self) -> &'static str {
        tr!(self,
            "La clé n'a pas confirmé la réception du Wi-Fi : rien n'a été changé. Vérifiez que le câble est sur le bon port de la clé et qu'elle a bien son firmware, puis réessayez.",
            "The dongle did not confirm it received the Wi-Fi settings: nothing was changed. Check that the cable is on its right port and that it has its firmware, then try again.",
            "Der Dongle hat den Empfang der WLAN-Daten nicht bestätigt: Es wurde nichts geändert. Prüfen Sie, ob das Kabel an seinem richtigen Anschluss steckt und er seine Firmware hat, und versuchen Sie es erneut.",
            "Il dongle non ha confermato la ricezione del Wi-Fi: non è stato modificato nulla. Verifica che il cavo sia sulla sua porta giusta e che abbia il suo firmware, poi riprova.",
            "El pincho no confirmó la recepción del Wi-Fi: no se cambió nada. Verifica que el cable esté en su puerto correcto y que tenga su firmware, luego inténtalo de nuevo.")
    }
    pub fn wifi_probably_saved(&self) -> &'static str {
        tr!(self,
            "La clé s'est débranchée sans répondre : les identifiants Wi-Fi sont probablement enregistrés, mais rien ne le confirme. Rebranchez la clé sur le poêle et vérifiez qu'elle se connecte ; sinon, recommencez.",
            "The dongle disconnected without answering: the Wi-Fi settings are probably saved, but nothing confirms it. Plug the dongle into the stove and check that it connects; if it does not, try again.",
            "Der Dongle hat sich ohne Antwort getrennt: Die WLAN-Daten sind wahrscheinlich gespeichert, es gibt aber keine Bestätigung. Stecken Sie den Dongle an den Ofen und prüfen Sie, ob er sich verbindet; falls nicht, versuchen Sie es erneut.",
            "Il dongle si è scollegato senza rispondere: le impostazioni Wi-Fi sono probabilmente salvate, ma nulla lo conferma. Collega il dongle alla stufa e verifica che si connetta; altrimenti riprova.",
            "El pincho se desconectó sin responder: probablemente se guardaron los ajustes de Wi-Fi, pero nada lo confirma. Conecta el pincho a la estufa y verifica que se conecte; de lo contrario, inténtalo de nuevo.")
    }
    pub fn ota_password_probably_saved(&self) -> &'static str {
        tr!(self,
            "La clé s'est débranchée sans répondre : le mot de passe de mise à jour est probablement enregistré, mais rien ne le confirme. Rebranchez la clé et vérifiez, sinon recommencez.",
            "The dongle disconnected without answering: the update password is probably saved, but nothing confirms it. Plug the dongle in again and check; if not, try again.",
            "Der Dongle hat sich ohne Antwort getrennt: Das Update-Passwort ist wahrscheinlich gespeichert, es gibt aber keine Bestätigung. Stecken Sie den Dongle erneut an und prüfen Sie es; falls nicht, versuchen Sie es erneut.",
            "Il dongle si è scollegato senza rispondere: la password di aggiornamento è probabilmente salvata, ma nulla lo conferma. Ricollega il dongle e verifica; altrimenti riprova.",
            "El pincho se desconectó sin responder: probablemente se guardó la contraseña de actualización, pero nada lo confirma. Vuelve a conectar el pincho y verifica; de lo contrario, inténtalo de nuevo.")
    }
    pub fn ota_password_not_confirmed(&self) -> &'static str {
        tr!(self,
            "La clé n'a pas confirmé le mot de passe de mise à jour : rien n'a été changé. Vérifiez que le câble est sur le port UART/COM de la clé et que son firmware est en version 4.0 ou plus, puis réessayez.",
            "The dongle did not confirm the update password: nothing was changed. Check that the cable is on the dongle's UART/COM port and that its firmware is version 4.0 or later, then try again.",
            "Der Dongle hat das Update-Passwort nicht bestätigt: Es wurde nichts geändert. Prüfen Sie, ob das Kabel am UART/COM-Anschluss des Dongles steckt und die Firmware Version 4.0 oder neuer ist, und versuchen Sie es erneut.",
            "Il dongle non ha confermato la password di aggiornamento: non è stato modificato nulla. Verifica che il cavo sia sulla porta UART/COM del dongle e che il firmware sia in versione 4.0 o successiva, poi riprova.",
            "El pincho no confirmó la contraseña de actualización: no se cambió nada. Verifica que el cable esté en el puerto UART/COM del pincho y que el firmware sea la versión 4.0 o posterior, luego inténtalo de nuevo.")
    }
    pub fn ota_password_sent(&self, removed: bool) -> &'static str {
        if removed {
            tr!(self,
                "Mot de passe de mise à jour retiré de la clé. Il reste demandé jusqu'à ce que la clé soit débranchée puis rebranchée.",
                "Update password removed from the dongle. It is still asked until the dongle is unplugged and plugged in again.",
                "Update-Passwort vom Dongle entfernt. Es wird weiter abgefragt, bis der Dongle aus- und wieder eingesteckt wird.",
                "Password di aggiornamento rimossa dal dongle. Viene ancora richiesta finché il dongle non viene scollegato e ricollegato.",
                "Contraseña de actualización eliminada del pincho. Todavía se solicita hasta que el pincho se desconecte y se vuelva a conectar.")
        } else {
            tr!(self,
                "Mot de passe de mise à jour enregistré sur la clé. Il sera demandé à chaque mise à jour sans fil.",
                "Update password saved on the dongle. It will be asked at each wireless update.",
                "Update-Passwort auf dem Dongle gespeichert. Es wird bei jedem drahtlosen Update abgefragt.",
                "Password di aggiornamento salvata sul dongle. Verrà richiesta a ogni aggiornamento tramite Wi-Fi.",
                "Contraseña de actualización guardada en el pincho. Se solicitará en cada actualización inalámbrica.")
        }
    }
    pub fn ota_waiting_reply(&self, attempt: u32, of: u32) -> String {
        tr!(self,
            format!("Attente de la réponse de la clé ({}/{})...", attempt, of),
            format!("Waiting for the dongle to answer ({}/{})...", attempt, of),
            format!("Warten auf Antwort des Dongles ({}/{})...", attempt, of),
            format!("In attesa della risposta del dongle ({}/{})...", attempt, of),
            format!("En espera de la respuesta del pincho ({}/{})...", attempt, of))
    }
    pub fn ota_no_reply(&self, ip: &str) -> String {
        tr!(self,
            format!("La clé sur {} (port 3232) n'a pas répondu à l'invitation de mise à jour. Vérifiez qu'elle est allumée et sur le même réseau.", ip),
            format!("The dongle at {} (port 3232) did not answer the update invitation. Check that it is powered and on the same network.", ip),
            format!("Der Dongle unter {} (Port 3232) hat nicht auf die Update-Einladung geantwortet. Prüfen Sie, ob er eingeschaltet und im selben Netzwerk ist.", ip),
            format!("Il dongle su {} (porta 3232) non ha risposto all'invito di aggiornamento. Verifica che sia acceso e sulla stessa rete.", ip),
            format!("El pincho en {} (puerto 3232) no respondió a la invitación de actualización. Verifica que esté encendido y en la misma red.", ip))
    }
    pub fn ota_waiting_tcp(&self) -> &'static str {
        tr!(self,
            "Attente de la connexion TCP de la clé...",
            "Waiting for the dongle's TCP connection...",
            "Warten auf die TCP-Verbindung des Dongles...",
            "In attesa della connessione TCP del dongle...",
            "En espera de la conexión TCP del pincho...")
    }
    pub fn ota_connected_init(&self) -> &'static str {
        tr!(self,
            "Connexion établie, initialisation du transfert...",
            "Connected, starting the transfer...",
            "Verbindung hergestellt, Übertragung wird gestartet...",
            "Connessione stabilita, avvio del trasferimento...",
            "Conexión establecida, iniciando la transferencia...")
    }
    pub fn ota_tcp_received(&self, peer: &str) -> String {
        tr!(self,
            format!("Connexion TCP reçue de la clé depuis {}", peer),
            format!("TCP connection received from the dongle at {}", peer),
            format!("TCP-Verbindung des Dongles von {} erhalten", peer),
            format!("Connessione TCP ricevuta dal dongle da {}", peer),
            format!("Conexión TCP recibida del pincho desde {}", peer))
    }
    pub fn ota_tcp_accept_error(&self, error: &str) -> String {
        tr!(self,
            format!("Erreur d'acceptation TCP : {}", error),
            format!("TCP accept error: {}", error),
            format!("Fehler beim Annehmen der TCP-Verbindung: {}", error),
            format!("Errore di accettazione TCP: {}", error),
            format!("Error al aceptar la conexión TCP: {}", error))
    }
    /// The dongle accepted the invitation but never connected back. `default_slot_bytes` is given when the
    /// firmware is larger than the update slot of Arduino's default partition scheme: that cause is then named too.
    pub fn ota_no_tcp_connection(&self, port: u16, firmware_bytes: usize, default_slot_bytes: Option<usize>) -> String {
        let mut text = tr!(self,
            format!("Délai d'attente dépassé : la clé n'a pas pu se connecter au port TCP {} de cet ordinateur. Vérifiez que votre pare-feu ou le routage de votre réseau autorise les connexions entrantes sur le réseau local.", port),
            format!("Timed out: the dongle could not connect to TCP port {} of this computer. Check that your firewall or network routing allows incoming connections on the local network.", port),
            format!("Zeitüberschreitung: Der Dongle konnte sich nicht mit TCP-Port {} dieses Computers verbinden. Prüfen Sie, ob Ihre Firewall oder das Netzwerk-Routing eingehende Verbindungen im lokalen Netzwerk zulässt.", port),
            format!("Tempo scaduto: il dongle non è riuscito a connettersi alla porta TCP {} di questo computer. Verifica che il firewall o l'instradamento di rete consenta le connessioni in entrata sulla rete locale.", port),
            format!("Tiempo de espera agotado: el pincho no pudo conectarse al puerto TCP {} de este ordenador. Comprueba que tu firewall o el enrutamiento de red permita las conexiones entrantes en la red local.", port));
        if let Some(slot) = default_slot_bytes {
            text.push_str("\n\n");
            text.push_str(&tr!(self,
                format!("Autre cause possible : l'emplacement de mise à jour de la clé est trop petit pour ce firmware ({} octets). Il fait {} octets quand la clé a été flashée depuis l'IDE Arduino avec le schéma de partition par défaut. Dans ce cas, flashez la clé une fois par USB en mode « Première installation », le seul qui réécrit la table de partitions (le mode « Mise à jour » la conserve). Le Wi-Fi sera à ressaisir, puis les mises à jour sans fil fonctionneront de nouveau.", firmware_bytes, slot),
                format!("Other possible cause: the dongle's update slot is too small for this firmware ({} bytes). It is {} bytes when the dongle was flashed from the Arduino IDE with the default partition scheme. In that case, flash the dongle once over USB in \"First installation\" mode, the only one that rewrites the partition table (the \"Update\" mode keeps it). You will have to enter the Wi-Fi again; wireless updates will work afterwards.", firmware_bytes, slot),
                format!("Andere mögliche Ursache: Der Update-Speicherplatz des Dongles ist für diese Firmware ({} Bytes) zu klein. Er hat {} Bytes, wenn der Dongle aus der Arduino IDE mit dem Standard-Partitionsschema geflasht wurde. Flashen Sie den Dongle in diesem Fall einmal per USB im Modus „Erstinstallation“, nur dieser schreibt die Partitionstabelle neu (der Modus „Update“ behält sie). Das WLAN muss danach neu eingegeben werden; drahtlose Updates funktionieren dann wieder.", firmware_bytes, slot),
                format!("Altra causa possibile: lo spazio di aggiornamento del dongle è troppo piccolo per questo firmware ({} byte). È di {} byte quando il dongle è stato programmato dall'IDE Arduino con lo schema di partizioni predefinito. In tal caso, esegui una volta il flash del dongle tramite USB in modalità «Prima installazione», l'unica che riscrive la tabella delle partizioni (la modalità «Aggiornamento» la conserva). Dovrai reinserire il Wi-Fi; dopo, gli aggiornamenti tramite Wi-Fi funzioneranno di nuovo.", firmware_bytes, slot),
                format!("Otra posible causa: el espacio de actualización del pincho es demasiado pequeño para este firmware ({} bytes). Es de {} bytes cuando el pincho se flashea desde el IDE de Arduino con el esquema de partición por defecto. En ese caso, flashea el pincho una vez por USB en modo «Primera instalación», el único que reescribe la tabla de particiones (el modo «Actualización» la conserva). Tendrás que volver a introducir el Wi-Fi; las actualizaciones inalámbricas funcionarán después.", firmware_bytes, slot)));
        }
        text
    }
    pub fn ota_sending(&self) -> &'static str {
        tr!(self, "Envoi du firmware...", "Sending the firmware...", "Firmware wird gesendet...", "Invio del firmware...", "Enviando el firmware...")
    }
    pub fn ota_send_error(&self) -> &'static str {
        tr!(self,
            "Erreur lors de l'envoi des données vers la clé",
            "Error while sending data to the dongle",
            "Fehler beim Senden der Daten an den Dongle",
            "Errore durante l'invio dei dati al dongle",
            "Error al enviar los datos al pincho")
    }
    pub fn ota_progress(&self, percent: u32) -> String {
        tr!(self,
            format!("Envoi Wi-Fi : {}%", percent),
            format!("Wi-Fi transfer: {}%", percent),
            format!("WLAN-Übertragung: {}%", percent),
            format!("Invio Wi-Fi: {}%", percent),
            format!("Enviando Wi-Fi: {}%", percent))
    }
    pub fn ota_verifying(&self) -> &'static str {
        tr!(self,
            "Vérification de l'intégrité et écriture en mémoire par la clé...",
            "The dongle is checking integrity and writing to flash...",
            "Der Dongle prüft die Integrität und schreibt in den Flash...",
            "Il dongle verifica l'integrità e scrive in memoria...",
            "El pincho está comprobando la integridad y escribiendo en la memoria...")
    }
    pub fn ota_done(&self) -> &'static str {
        tr!(self,
            "Mise à jour sans fil terminée avec succès !",
            "Wireless update completed successfully!",
            "Drahtloses Update erfolgreich abgeschlossen!",
            "Aggiornamento tramite Wi-Fi completato con successo!",
            "¡Actualización inalámbrica completada con éxito!")
    }
    pub fn rebooted_online(&self) -> &'static str {
        tr!(self,
            "✔ La clé a redémarré et est de nouveau en ligne !",
            "✔ The dongle restarted and is back online!",
            "✔ Der Dongle wurde neu gestartet und ist wieder online!",
            "✔ Il dongle si è riavviato ed è di nuovo online!",
            "✔ El pincho se ha reiniciado y está de nuevo en línea!")
    }
    pub fn rebooting_waiting(&self) -> &'static str {
        tr!(self,
            "Redémarrage de la clé en cours... Attente de la reconnexion au Wi-Fi...",
            "The dongle is restarting... Waiting for it to rejoin the Wi-Fi...",
            "Der Dongle startet neu... Warten auf die erneute WLAN-Verbindung...",
            "Riavvio del dongle in corso... In attesa della riconnessione al Wi-Fi...",
            "Reinicio del pincho en curso... Esperando a que se vuelva a conectar al Wi-Fi...")
    }
    pub fn waiting_reconnect(&self, attempt: u32, of: u32) -> String {
        tr!(self,
            format!("Attente de la reconnexion au réseau ({}/{})...", attempt, of),
            format!("Waiting for it to rejoin the network ({}/{})...", attempt, of),
            format!("Warten auf die erneute Netzwerkverbindung ({}/{})...", attempt, of),
            format!("In attesa della riconnessione alla rete ({}/{})...", attempt, of),
            format!("Esperando a que se vuelva a conectar a la red ({}/{})...", attempt, of))
    }
    pub fn reconnect_slow(&self) -> &'static str {
        tr!(self,
            "La clé met plus de temps à se reconnecter. Vérifiez son adresse IP sur votre box.",
            "The dongle is taking longer to reconnect. Check its IP address on your router.",
            "Der Dongle braucht länger für die Wiederverbindung. Prüfen Sie seine IP-Adresse im Router.",
            "Il dongle impiega più tempo a riconnettersi. Verifica il suo indirizzo IP sul router.",
            "El pincho está tardando más en reconectarse. Comprueba su dirección IP en el router.")
    }

    // --- USB flashing ------------------------------------------------------------------------------------------
    pub fn cannot_list_ports(&self) -> &'static str {
        tr!(self,
            "Impossible d'énumérer les ports série",
            "Cannot list the serial ports",
            "Die seriellen Ports können nicht aufgelistet werden",
            "Impossibile elencare le porte seriali",
            "No se pueden enumerar los puertos serie")
    }
    pub fn port_generic(&self) -> &'static str {
        tr!(self, "Port série générique", "Generic serial port", "Generischer serieller Port", "Porta seriale generica", "Puerto serie genérico")
    }
    pub fn unknown(&self) -> &'static str {
        tr!(self, "Inconnu", "Unknown", "Unbekannt", "Sconosciuto", "Desconocido")
    }
    pub fn port_open_firenet(&self) -> &'static str {
        match self {
            CliLang::Fr => "🔥 Clé Open Firenet (déjà installée)",
            CliLang::En => "🔥 Open Firenet dongle (already installed)",
            CliLang::De => "🔥 Open-Firenet-Dongle (bereits installiert)",
            CliLang::It => "🔥 Dongle Open Firenet (già installato)",
            CliLang::Es => "🔥 Pincho Open Firenet (ya instalado)",
        }
    }
    pub fn port_serial_adapter(&self, product: &str) -> String {
        tr!(self,
            format!("⚡ Adaptateur Série/USB ({})", product),
            format!("⚡ Serial/USB adapter ({})", product),
            format!("⚡ Seriell/USB-Adapter ({})", product),
            format!("⚡ Adattatore seriale/USB ({})", product),
            format!("⚡ Adaptador serie/USB ({})", product))
    }
    pub fn invalid_offset(&self, offset: &str) -> String {
        tr!(self,
            format!("Adresse de départ invalide : {}", offset),
            format!("Invalid start address: {}", offset),
            format!("Ungültige Startadresse: {}", offset),
            format!("Indirizzo di partenza non valido: {}", offset),
            format!("Dirección de inicio no válida: {}", offset))
    }
    pub fn usb_flash_start(&self, port: &str, baud: u32, offset: &str) -> String {
        tr!(self,
            format!("Début du flashage USB sur {} à {} bauds (adresse {})...", port, baud, offset),
            format!("Starting USB flashing on {} at {} baud (address {})...", port, baud, offset),
            format!("USB-Flash auf {} mit {} Baud wird gestartet (Adresse {})...", port, baud, offset),
            format!("Avvio del flash USB su {} a {} baud (indirizzo {})...", port, baud, offset),
            format!("Inicio del flash USB en {} a {} baudios (dirección {})...", port, baud, offset))
    }
    pub fn usb_connecting_chip(&self) -> &'static str {
        tr!(self,
            "Connexion à la puce ESP32-S3...",
            "Connecting to the ESP32-S3 chip...",
            "Verbindung zum ESP32-S3-Chip...",
            "Connessione al chip ESP32-S3...",
            "Conectando con el chip ESP32-S3...")
    }
    pub fn usb_done(&self) -> &'static str {
        tr!(self,
            "Flashage terminé avec succès !",
            "Flashing completed successfully!",
            "Flash erfolgreich abgeschlossen!",
            "Flash completato con successo!",
            "¡Flash completado con éxito!")
    }
    pub fn usb_native_failed(&self, error: &str) -> String {
        tr!(self,
            format!("Échec du flashage intégré : {}", error),
            format!("Built-in flashing failed: {}", error),
            format!("Integrierter Flash fehlgeschlagen: {}", error),
            format!("Flash integrato non riuscito: {}", error),
            format!("Flash integrado fallido: {}", error))
    }
    pub fn usb_fallback(&self, tool: &str) -> String {
        tr!(self,
            format!("Tentative de secours via {}...", tool),
            format!("Trying again with {}...", tool),
            format!("Erneuter Versuch mit {}...", tool),
            format!("Nuovo tentativo con {}...", tool),
            format!("Nuevo intento con {}...", tool))
    }
    pub fn usb_flash_failed(&self, error: &str) -> String {
        tr!(self,
            format!("Échec du flashage USB : {}\n\nAstuce : sur les cartes à USB natif (M5Stamp S3, XIAO ESP32-S3), maintenez le bouton BOOT enfoncé lors du branchement USB pour forcer le mode de démarrage (bootloader).", error),
            format!("USB flashing failed: {}\n\nTip: on native USB boards (M5Stamp S3, XIAO ESP32-S3), hold down the BOOT button while plugging in the USB cable to force bootloader mode.", error),
            format!("USB-Flash fehlgeschlagen: {}\n\nTipp: Halten Sie bei Boards mit nativem USB (M5Stamp S3, XIAO ESP32-S3) beim Einstecken des USB-Kabels die BOOT-Taste gedrückt, um den Bootloader-Modus zu erzwingen.", error),
            format!("Flash USB non riuscito: {}\n\nSuggerimento: sulle schede con USB nativo (M5Stamp S3, XIAO ESP32-S3), tieni premuto il pulsante BOOT mentre colleghi il cavo USB per forzare la modalità bootloader.", error),
            format!("Flash USB fallido: {}\n\nConsejo: en las placas con USB nativo (M5Stamp S3, XIAO ESP32-S3), mantén presionado el botón BOOT mientras conectas el cable USB para forzar el modo bootloader.", error))
    }
    pub fn cannot_open_port(&self, port: &str, error: &str) -> String {
        tr!(self,
            format!("Impossible d'ouvrir le port série {} : {}", port, error),
            format!("Cannot open serial port {}: {}", port, error),
            format!("Der serielle Port {} kann nicht geöffnet werden: {}", port, error),
            format!("Impossibile aprire la porta seriale {}: {}", port, error),
            format!("No se puede abrir el puerto serie {}: {}", port, error))
    }
    pub fn cannot_open_serial(&self) -> &'static str {
        tr!(self,
            "Impossible d'ouvrir le port série",
            "Cannot open the serial port",
            "Der serielle Port kann nicht geöffnet werden",
            "Impossibile aprire la porta seriale",
            "No se puede abrir el puerto serie")
    }
    pub fn usb_syncing(&self) -> &'static str {
        tr!(self,
            "Connexion et synchronisation avec l'ESP32-S3...",
            "Connecting and synchronising with the ESP32-S3...",
            "Verbindung und Synchronisierung mit dem ESP32-S3...",
            "Connessione e sincronizzazione con l'ESP32-S3...",
            "Conectando y sincronizando con el ESP32-S3...")
    }
    pub fn bootloader_connect_failed(&self, error: &str) -> String {
        tr!(self,
            format!("Connexion au bootloader impossible ({})", error),
            format!("Cannot connect to the bootloader ({})", error),
            format!("Verbindung zum Bootloader nicht möglich ({})", error),
            format!("Impossibile connettersi al bootloader ({})", error),
            format!("No se puede conectar con el bootloader ({})", error))
    }
    pub fn usb_synced_writing(&self) -> &'static str {
        tr!(self,
            "ESP32-S3 synchronisé. Écriture en mémoire flash...",
            "ESP32-S3 synchronised. Writing to flash...",
            "ESP32-S3 synchronisiert. Schreiben in den Flash...",
            "ESP32-S3 sincronizzato. Scrittura nella memoria flash...",
            "ESP32-S3 sincronizado. Escribiendo en la memoria flash...")
    }
    pub fn usb_preparing_flash(&self) -> &'static str {
        tr!(self,
            "Préparation de la mémoire flash...",
            "Preparing the flash memory...",
            "Flash-Speicher wird vorbereitet...",
            "Preparazione della memoria flash...",
            "Preparando la memoria flash...")
    }
    pub fn usb_write_progress(&self, percent: u32) -> String {
        tr!(self,
            format!("Écriture flash USB : {}%", percent),
            format!("USB flash write: {}%", percent),
            format!("USB-Flash-Schreiben: {}%", percent),
            format!("Scrittura flash USB: {}%", percent),
            format!("Escritura flash USB: {}%", percent))
    }
    pub fn integrity_ok(&self) -> &'static str {
        tr!(self,
            "Vérification de l'intégrité... OK",
            "Integrity check... OK",
            "Integritätsprüfung... OK",
            "Verifica dell'integrità... OK",
            "Comprobación de integridad... OK")
    }
    pub fn usb_write_error(&self, error: &str) -> String {
        tr!(self,
            format!("Erreur lors de l'écriture flash : {}", error),
            format!("Error while writing to flash: {}", error),
            format!("Fehler beim Schreiben in den Flash: {}", error),
            format!("Errore durante la scrittura flash: {}", error),
            format!("Error al escribir en la memoria flash: {}", error))
    }
    pub fn esptool_launch_failed(&self) -> &'static str {
        tr!(self,
            "Échec du lancement d'esptool",
            "Failed to start esptool",
            "esptool konnte nicht gestartet werden",
            "Avvio di esptool non riuscito",
            "Error al iniciar esptool")
    }
    pub fn esptool_failed(&self, status: &str) -> String {
        tr!(self,
            format!("Le flashage par esptool a échoué ({})", status),
            format!("Flashing with esptool failed ({})", status),
            format!("Flash mit esptool fehlgeschlagen ({})", status),
            format!("Flash con esptool non riuscito ({})", status),
            format!("Flash con esptool fallido ({})", status))
    }

    // --- GitHub downloads --------------------------------------------------------------------------------------
    pub fn github_request_failed(&self) -> &'static str {
        tr!(self,
            "Échec de la requête vers GitHub",
            "The request to GitHub failed",
            "Die Anfrage an GitHub ist fehlgeschlagen",
            "Richiesta a GitHub non riuscita",
            "Error al solicitar a GitHub")
    }
    pub fn github_decode_failed(&self) -> &'static str {
        tr!(self,
            "Impossible de lire la réponse de GitHub",
            "Cannot read GitHub's answer",
            "Die Antwort von GitHub kann nicht gelesen werden",
            "Impossibile leggere la risposta di GitHub",
            "No se puede leer la respuesta de GitHub")
    }
    pub fn no_stable_release(&self) -> &'static str {
        tr!(self,
            "Aucune version stable trouvée",
            "No stable release found",
            "Kein stabiles Release gefunden",
            "Nessuna versione stabile trovata",
            "No se encontró ninguna versión estable")
    }
    pub fn download_error(&self, url: &str, error: &str) -> String {
        tr!(self,
            format!("Erreur lors du téléchargement de {} : {}", url, error),
            format!("Error while downloading {}: {}", url, error),
            format!("Fehler beim Herunterladen von {}: {}", url, error),
            format!("Errore durante il download di {}: {}", url, error),
            format!("Error al descargar {}: {}", url, error))
    }
    pub fn cannot_create_file(&self) -> &'static str {
        tr!(self,
            "Impossible de créer le fichier de destination",
            "Cannot create the destination file",
            "Die Zieldatei kann nicht erstellt werden",
            "Impossibile creare il file di destinazione",
            "No se puede crear el archivo de destino")
    }
    pub fn download_done(&self) -> &'static str {
        tr!(self, "Téléchargement terminé", "Download complete", "Download abgeschlossen", "Download completato", "Descarga completada")
    }
    pub fn checksums_download_failed(&self) -> &'static str {
        tr!(self,
            "Impossible de télécharger le fichier SHA256SUMS",
            "Cannot download the SHA256SUMS file",
            "Die Datei SHA256SUMS kann nicht heruntergeladen werden",
            "Impossibile scaricare il file SHA256SUMS",
            "No se puede descargar el archivo SHA256SUMS")
    }
    pub fn signature_download_failed(&self) -> &'static str {
        tr!(self,
            "Impossible de télécharger la signature SHA256SUMS.minisig",
            "Cannot download the SHA256SUMS.minisig signature",
            "Die Signatur SHA256SUMS.minisig kann nicht heruntergeladen werden",
            "Impossibile scaricare la firma SHA256SUMS.minisig",
            "No se puede descargar la firma SHA256SUMS.minisig")
    }
    pub fn invalid_public_key(&self, error: &str) -> String {
        tr!(self,
            format!("Clé publique Minisign invalide : {}", error),
            format!("Invalid Minisign public key: {}", error),
            format!("Ungültiger öffentlicher Minisign-Schlüssel: {}", error),
            format!("Chiave pubblica Minisign non valida: {}", error),
            format!("Clave pública de Minisign no válida: {}", error))
    }
    pub fn invalid_signature(&self, error: &str) -> String {
        tr!(self,
            format!("Format de signature Minisign invalide : {}", error),
            format!("Invalid Minisign signature format: {}", error),
            format!("Ungültiges Minisign-Signaturformat: {}", error),
            format!("Formato della firma Minisign non valido: {}", error),
            format!("Formato de firma Minisign no válido: {}", error))
    }
    pub fn signature_check_failed(&self, tag: &str, error: &str) -> String {
        tr!(self,
            format!("Échec de la vérification de la signature Minisign pour la version {} : {}", tag, error),
            format!("Minisign signature verification failed for release {}: {}", tag, error),
            format!("Überprüfung der Minisign-Signatur für Release {} fehlgeschlagen: {}", tag, error),
            format!("Verifica della firma Minisign non riuscita per la versione {}: {}", tag, error),
            format!("La verificación de la firma Minisign falló para la versión {}: {}", tag, error))
    }
    pub fn checksum_mismatch(&self, hash: &str) -> String {
        tr!(self,
            format!("Échec de la validation SHA256 ! Le fichier téléchargé ne correspond pas à la somme de contrôle officielle ({})", hash),
            format!("SHA256 validation failed! The downloaded file does not match the official checksum ({})", hash),
            format!("SHA256-Prüfung fehlgeschlagen! Die heruntergeladene Datei stimmt nicht mit der offiziellen Prüfsumme überein ({})", hash),
            format!("Convalida SHA256 non riuscita! Il file scaricato non corrisponde al checksum ufficiale ({})", hash),
            format!("¡La validación SHA256 falló! El archivo descargado no coincide con el checksum oficial ({})", hash))
    }
    pub fn release_not_found(&self, tag: &str) -> String {
        tr!(self,
            format!("Version {} introuvable sur GitHub", tag),
            format!("Release {} not found on GitHub", tag),
            format!("Release {} auf GitHub nicht gefunden", tag),
            format!("Versione {} non trovata su GitHub", tag),
            format!("Versión {} no encontrada en GitHub", tag))
    }
    pub fn no_update_binary(&self) -> &'static str {
        tr!(self,
            "Aucun fichier de mise à jour trouvé dans cette version",
            "No update file found in this release",
            "Keine Update-Datei in diesem Release gefunden",
            "Nessun file di aggiornamento trovato in questa versione",
            "No se encontró ningún archivo de actualización en esta versión")
    }
    pub fn no_factory_binary(&self) -> &'static str {
        tr!(self,
            "Aucune image complète (factory) trouvée dans cette version",
            "No full image (factory) found in this release",
            "Kein vollständiges Image (factory) in diesem Release gefunden",
            "Nessuna immagine completa (factory) trovata in questa versione",
            "No se encontró ninguna imagen completa (factory) en esta versión")
    }
    pub fn downloading_verifying(&self) -> &'static str {
        tr!(self,
            "Téléchargement et vérification de la signature...",
            "Downloading and verifying the signature...",
            "Herunterladen und Überprüfen der Signatur...",
            "Download e verifica della firma...",
            "Descargando y verificando la firma...")
    }
    pub fn choose_version_or_file(&self) -> &'static str {
        tr!(self,
            "Veuillez choisir une version ou un fichier local",
            "Please choose a release or a local file",
            "Bitte wählen Sie ein Release oder eine lokale Datei",
            "Scegli una versione o un file locale",
            "Por favor, elija una versión o un archivo local")
    }
    pub fn github_unreachable(&self, error: &str) -> String {
        tr!(self,
            format!("Impossible de contacter GitHub : {}", error),
            format!("Cannot reach GitHub: {}", error),
            format!("GitHub ist nicht erreichbar: {}", error),
            format!("Impossibile contattare GitHub: {}", error),
            format!("No se pudo contactar a GitHub: {}", error))
    }
    pub fn badge_beta(&self) -> &'static str {
        tr!(self, "[BÊTA/TEST]", "[BETA/TEST]", "[BETA/TEST]", "[BETA/TEST]", "[BETA/TEST]")
    }
    pub fn badge_stable_tag(&self) -> &'static str {
        tr!(self, "[STABLE]", "[STABLE]", "[STABIL]", "[STABILE]", "[ESTABLE]")
    }

    // --- window results ----------------------------------------------------------------------------------------
    pub fn usb_flashing_in_progress(&self) -> &'static str {
        tr!(self,
            "Flashage en cours sur le port USB...",
            "Flashing over the USB port...",
            "Flash über den USB-Port läuft...",
            "Flash in corso sulla porta USB...",
            "Flasheando en el puerto USB...")
    }
    pub fn usb_done_restarting(&self) -> &'static str {
        tr!(self,
            "Flashage terminé avec succès ! La clé redémarre.",
            "Flashing completed successfully! The dongle is restarting.",
            "Flash erfolgreich abgeschlossen! Der Dongle startet neu.",
            "Flash completato con successo! Il dongle si sta riavviando...",
            "¡Flasheo completado con éxito! El dongle se está reiniciando.")
    }
    pub fn ota_sending_wifi(&self) -> &'static str {
        tr!(self,
            "Envoi du firmware via Wi-Fi...",
            "Sending the firmware over Wi-Fi...",
            "Firmware wird über WLAN gesendet...",
            "Invio del firmware tramite Wi-Fi...",
            "Enviando el firmware por Wi-Fi...")
    }
    pub fn ota_success(&self) -> &'static str {
        tr!(self,
            "Mise à jour réussie ! La clé a redémarré et est de nouveau en ligne.",
            "Update successful! The dongle restarted and is back online.",
            "Update erfolgreich! Der Dongle wurde neu gestartet und ist wieder online.",
            "Aggiornamento riuscito! Il dongle si è riavviato ed è di nuovo online...",
            "¡Actualización exitosa! El dongle se ha reiniciado y está de nuevo en línea.")
    }
    pub fn wifi_config_sent(&self) -> &'static str {
        tr!(self,
            "Les identifiants Wi-Fi sont enregistrés sur la clé, vous pouvez la débrancher pour l'utiliser sur votre poêle.",
            "The Wi-Fi credentials are saved on the dongle, you can unplug it to use it on your stove.",
            "Die WLAN-Zugangsdaten wurden auf dem Dongle gespeichert. Sie können ihn ausstecken, um ihn an Ihrem Ofen zu verwenden.",
            "Le credenziali Wi-Fi sono state salvate sul dongle, puoi scollegarlo per usarlo sulla stufa...",
            "Las credenciales de Wi-Fi se han guardado en el dongle, puedes desconectarlo para usarlo en tu estufa.")
    }
    pub fn cannot_open_browser(&self, error: &str) -> String {
        tr!(self,
            format!("Impossible d'ouvrir le navigateur : {}", error),
            format!("Cannot open the browser: {}", error),
            format!("Der Browser kann nicht geöffnet werden: {}", error),
            format!("Impossibile aprire il browser: {}", error),
            format!("No se pudo abrir el navegador: {}", error))
    }

    // --- discovery ---------------------------------------------------------------------------------------------
    pub fn version_unknown(&self) -> &'static str {
        tr!(self, "Inconnue", "Unknown", "Unbekannt", "Sconosciuta", "Desconocida")
    }
    /// Label of the stove's main state number, as the bridge reports it.
    pub fn stove_state(&self, code: i64) -> &'static str {
        match code {
            0 => "Standby",
            1 => tr!(self, "Allumage", "Ignition", "Zündung", "Accensione", "Encendido"),
            2 => tr!(self, "Démarrage", "Start-up", "Anlauf", "Avvio", "Inicio"),
            3 => tr!(self, "Régulation", "Regulation", "Regelung", "Regolazione", "Regulación"),
            4 => tr!(self, "Nettoyage", "Cleaning", "Reinigung", "Pulizia", "Limpieza"),
            5 => tr!(self, "Arrêt", "Shutdown", "Ausbrand", "Spegnimento", "Apagado"),
            _ => self.unknown(),
        }
    }
    pub fn scanning_subnet(&self) -> &'static str {
        tr!(self,
            "🔍 Scan du sous-réseau :",
            "🔍 Scanning subnet:",
            "🔍 Subnetz wird gescannt:",
            "🔍 Scansione della sottorete:",
            "🔍 Escaneando la subred:")
    }
    pub fn dongle_summary(&self, ip: &str, model: &str, version: &str) -> String {
        tr!(self,
            format!("{} (Poêle {}, version {})", ip, model, version),
            format!("{} (Stove {}, version {})", ip, model, version),
            format!("{} (Ofen {}, Version {})", ip, model, version),
            format!("{} (Stufa {}, versione {})", ip, model, version),
            format!("{} (Estufa {}, versión {})", ip, model, version))
    }

    pub fn megabyte_unit(&self) -> &'static str {
        tr!(self, "Mo", "MB", "MB", "MB", "MB")
    }

    // --- language menu -----------------------------------------------------------------------------------------
    pub fn switch_language_choice(&self) -> &'static str {
        tr!(self,
            "🌐 8. Changer de langue / Switch Language",
            "🌐 8. Switch Language / Changer de langue",
            "🌐 8. Sprache wechseln / Switch Language",
            "🌐 8. Cambia lingua / Switch Language",
            "🌐 8. Cambiar idioma / Switch Language")
    }
}

#[cfg(test)]
mod tests {
    use super::CliLang;

    const ALL: [CliLang; 5] = [CliLang::Fr, CliLang::En, CliLang::De, CliLang::It, CliLang::Es];

    #[test]
    fn language_names_are_recognised() {
        assert_eq!(CliLang::from_str("it"), CliLang::It);
        assert_eq!(CliLang::from_str("Italiano"), CliLang::It);
        assert_eq!(CliLang::from_str("de"), CliLang::De);
        assert_eq!(CliLang::from_str("en"), CliLang::En);
        assert_eq!(CliLang::from_str("xx"), CliLang::Fr);
        assert_eq!(CliLang::from_window(Some("it")), CliLang::It);
        assert_eq!(CliLang::from_window(Some("fr")), CliLang::Fr);
        assert_eq!(CliLang::from_window(Some("es")), CliLang::Es);
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
