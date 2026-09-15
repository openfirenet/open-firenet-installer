export const translations = {
  fr: {
    // Header
    appTitle: "Open-Firenet",
    appTitleSuffix: "Installer",
    appSubtitle: "Assistant universel d'installation & mise à jour",
    statusReady: "Prêt",
    statusSearching: "Recherche sur le réseau...",
    statusNotFound: "Aucun poêle trouvé",
    statusFound: "Poêle détecté",
    statusError: "Erreur de scan",
    statusFlashingUsb: "Flashage USB en cours...",
    statusFlashingSuccess: "Flashage terminé avec succès !",
    statusOtaWaiting: "En attente...",
    statusOtaSending: "Envoi du firmware via Wi-Fi...",
    statusOtaSuccess: "Mise à jour réussie ! Le poêle redémarre.",

    // Tabs
    tabDiscovery: "Détection Réseau",
    tabOta: "Mise à jour Wi-Fi",
    tabUsb: "Flash USB",
    tabWifi: "Config Wi-Fi",
    tabReleases: "Releases",

    // Discovery Tab
    discoveryTitle: "Détection automatique de votre poêle",
    discoveryDesc: "Recherche votre poêle Open-Firenet via mDNS (openfirenet.local) et sur votre réseau local.",
    btnScanInProgress: "Recherche en cours...",
    btnScan: "Rechercher",
    scanLoadingTitle: "Recherche du poêle sur le réseau local...",
    scanLoadingDesc: "Détection mDNS et sonde du sous-réseau en cours...",
    emptyDevicesTitle: "Aucun poêle Open-Firenet détecté",
    emptyDevicesDesc: "Vérifiez que votre clé est allumée et connectée au même réseau Wi-Fi que cet ordinateur.",
    emptyBtnWifiSetup: "Configurer le Wi-Fi via USB",
    emptyBtnRetryScan: "Relancer la recherche",

    // Single Device Card
    stoveOnline: "En ligne / Connecté",
    stoveStandby: "En veille / Connecté",
    stoveModelDefault: "Poêle RIKA",
    labelIp: "Adresse IP",
    labelHostname: "Nom d'hôte",
    labelFirmware: "Version Firmware",
    versionUnknown: "Inconnue",
    labelWifiSignal: "Signal Wi-Fi",
    btnOpenWeb: "Ouvrir l'interface Web",
    btnUpdateOta: "Mettre à jour via Wi-Fi",

    // OTA Tab
    otaTitle: "Mise à jour sans fil (OTA)",
    otaDesc: "Mettez à jour le firmware de votre poêle à distance via votre réseau Wi-Fi.",
    labelOtaIp: "Adresse IP du poêle :",
    btnUseDetectedIp: "Utiliser l'IP détectée",
    placeholderOtaIp: "Ex: 192.168.1.93 ou openfirenet.local",
    labelOtaRelease: "Version du firmware à installer :",
    loadingReleases: "Chargement des versions officielles...",
    labelLocalFile: "Ou sélectionner un fichier binaire local (.bin) :",
    btnStartOta: "Lancer la mise à jour OTA",
    otaSuccessMsg: "Mise à jour réussie ! Le poêle redémarre.",

    // USB Tab
    usbTitle: "Flashage USB Série",
    usbDesc: "Idéal pour le premier flash d'un ESP32-S3 neuf ou une réinstallation complète.",
    btnRefreshPorts: "Actualiser les ports",
    labelUsbPort: "Port USB Série connecté :",
    detectingPorts: "Détection des ports en cours...",
    noPortDetected: "Aucun port série détecté (branchez votre clé en USB)",
    labelUsbRelease: "Version du firmware (Image Factory complète) :",
    labelUsbLocalFile: "Ou binaire factory local (.bin) :",
    btnStartUsbFlash: "Flasher la clé USB",
    flashingInProgress: "Flashage en cours...",

    // Wi-Fi Config Tab
    wifiTitle: "Configuration Wi-Fi de la clé",
    wifiDesc: "Transmettez le nom de votre box Wi-Fi (SSID) et sa clé de sécurité via la liaison USB.",
    labelWifiPort: "Port USB de la clé :",
    selectPort: "Sélectionnez un port...",
    labelWifiSsid: "Nom du réseau Wi-Fi (SSID) :",
    labelWifiPass: "Mot de passe Wi-Fi :",
    btnSendWifi: "Enregistrer le Wi-Fi dans la clé",
    wifiConfigSuccess: "Identifiants Wi-Fi envoyés ! La clé tente la connexion...",

    // Releases Tab
    releasesTitle: "Versions officielles Open-Firenet",
    releasesDesc: "Historique des publications officielles sur GitHub vérifiées par Minisign.",
    btnRefreshReleases: "Recharger",
    checkingReleases: "Interrogation de GitHub...",
    checkingReleasesDesc: "Vérification des versions officielles sur openfirenet/open-firenet...",
    badgeStable: "Stable",
    badgePrerelease: "Pré-release",
    availableFiles: "Fichiers & signatures :",
    viewOnGithub: "Voir sur GitHub",
    useForOta: "Installer via Wi-Fi (OTA)",
    useForUsb: "Installer via Flash USB",
    noReleaseTitle: "Aucune version officielle trouvée",
    noReleaseDesc: "Vous pouvez flasher ou mettre à jour directement votre clé à l'aide d'un fichier .bin local.",

    // Modal confirmation flash USB
    modalConfirmTitle: "Confirmation du flashage USB",
    modalConfirmDesc: "Vous êtes sur le point d'écrire le firmware sur la clé Open-Firenet. Veuillez vérifier les paramètres :",
    modalTargetPort: "Port USB cible :",
    modalTargetVersion: "Firmware :",
    modalConfirmWarning: "Attention : cette opération va écraser la mémoire flash de l'ESP32. Ne débranchez pas la clé pendant l'opération !",
    modalBtnCancel: "Annuler",
    modalBtnConfirm: "Confirmer et flasher",

    // Footer & Alerts
    footerTagline: "Open-Firenet Installer • Open-Source & Sans Télémétrie",
    footerNoStove: "Aucun poêle connecté",
    footerStoveActive: "Poêle actif :",
    alertFillIp: "Veuillez renseigner l'adresse IP du poêle.",
    alertSelectVersionOrFile: "Veuillez sélectionner une version ou un fichier binaire local.",
    alertSelectPort: "Veuillez sélectionner un port USB.",
    alertSelectWifiPort: "Veuillez sélectionner le port USB de la clé.",
    alertFillSsid: "Veuillez saisir le nom de votre réseau Wi-Fi (SSID).",
    alertNoStoveDetected: "Aucun poêle détecté sur le réseau pour le moment. Lancez une recherche d'abord.",
    alertErrorPrefix: "Erreur :"
  },

  en: {
    // Header
    appTitle: "Open-Firenet",
    appTitleSuffix: "Installer",
    appSubtitle: "Universal installation & update assistant",
    statusReady: "Ready",
    statusSearching: "Scanning network...",
    statusNotFound: "No stove found",
    statusFound: "Stove detected",
    statusError: "Scan error",
    statusFlashingUsb: "Flashing USB device...",
    statusFlashingSuccess: "Flashing completed successfully!",
    statusOtaWaiting: "Waiting...",
    statusOtaSending: "Sending firmware over Wi-Fi...",
    statusOtaSuccess: "Update successful! The stove is rebooting.",

    // Tabs
    tabDiscovery: "Network Scan",
    tabOta: "Wi-Fi Update",
    tabUsb: "USB Flash",
    tabWifi: "Wi-Fi Setup",
    tabReleases: "Releases",

    // Discovery Tab
    discoveryTitle: "Automatic Stove Discovery",
    discoveryDesc: "Discovers your Open-Firenet stove via mDNS (openfirenet.local) and your local network subnet.",
    btnScanInProgress: "Scanning...",
    btnScan: "Search",
    scanLoadingTitle: "Searching for stove on local network...",
    scanLoadingDesc: "Probing mDNS and local subnet in progress...",
    emptyDevicesTitle: "No Open-Firenet stove detected",
    emptyDevicesDesc: "Ensure your dongle is powered on and connected to the same Wi-Fi network as this computer.",
    emptyBtnWifiSetup: "Configure Wi-Fi via USB",
    emptyBtnRetryScan: "Scan Again",

    // Single Device Card
    stoveOnline: "Online / Connected",
    stoveStandby: "Standby / Connected",
    stoveModelDefault: "RIKA Stove",
    labelIp: "IP Address",
    labelHostname: "Hostname",
    labelFirmware: "Firmware Version",
    versionUnknown: "Unknown",
    labelWifiSignal: "Wi-Fi Signal",
    btnOpenWeb: "Open Web Dashboard",
    btnUpdateOta: "Update via Wi-Fi",

    // OTA Tab
    otaTitle: "Wireless Update (OTA)",
    otaDesc: "Remotely update your stove firmware over your local Wi-Fi network.",
    labelOtaIp: "Stove IP Address:",
    btnUseDetectedIp: "Use Detected IP",
    placeholderOtaIp: "E.g., 192.168.1.93 or openfirenet.local",
    labelOtaRelease: "Firmware version to install:",
    loadingReleases: "Loading official releases...",
    labelLocalFile: "Or select a local binary file (.bin):",
    btnStartOta: "Start OTA Update",
    otaSuccessMsg: "Update successful! The stove is restarting.",

    // USB Tab
    usbTitle: "USB Serial Flasher",
    usbDesc: "Ideal for first-time flashing of a fresh ESP32-S3 or complete reinstallation.",
    btnRefreshPorts: "Refresh Ports",
    labelUsbPort: "Connected USB Serial Port:",
    detectingPorts: "Detecting serial ports...",
    noPortDetected: "No serial port detected (plug your dongle into USB)",
    labelUsbRelease: "Firmware version (Full Factory Image):",
    labelUsbLocalFile: "Or local factory binary (.bin):",
    btnStartUsbFlash: "Flash USB Dongle",
    flashingInProgress: "Flashing in progress...",

    // Wi-Fi Config Tab
    wifiTitle: "Dongle Wi-Fi Configuration",
    wifiDesc: "Send your home Wi-Fi network name (SSID) and security password through the USB serial link.",
    labelWifiPort: "Dongle USB Port:",
    selectPort: "Select a port...",
    labelWifiSsid: "Wi-Fi Network Name (SSID):",
    labelWifiPass: "Wi-Fi Password:",
    btnSendWifi: "Save Wi-Fi to Dongle",
    wifiConfigSuccess: "Wi-Fi credentials sent! The dongle is attempting to connect...",

    // Releases Tab
    releasesTitle: "Official Open-Firenet Releases",
    releasesDesc: "History of official releases on GitHub verified cryptographically with Minisign.",
    btnRefreshReleases: "Reload",
    checkingReleases: "Querying GitHub...",
    checkingReleasesDesc: "Checking official releases on openfirenet/open-firenet...",
    badgeStable: "Stable",
    badgePrerelease: "Pre-release",
    availableFiles: "Files & signatures:",
    viewOnGithub: "View on GitHub",
    useForOta: "Install via Wi-Fi (OTA)",
    useForUsb: "Install via USB Flash",
    noReleaseTitle: "No official releases found",
    noReleaseDesc: "You can flash or update your dongle directly using a local .bin file.",

    // Modal confirmation flash USB
    modalConfirmTitle: "USB Flashing Confirmation",
    modalConfirmDesc: "You are about to flash your Open-Firenet dongle. Please verify the settings below:",
    modalTargetPort: "Target USB Port:",
    modalTargetVersion: "Firmware:",
    modalConfirmWarning: "Warning: This operation will overwrite the ESP32 flash memory. Do not unplug the dongle during the process!",
    modalBtnCancel: "Cancel",
    modalBtnConfirm: "Confirm & Flash",

    // Footer & Alerts
    footerTagline: "Open-Firenet Installer • Open-Source & Privacy First",
    footerNoStove: "No stove connected",
    footerStoveActive: "Active stove:",
    alertFillIp: "Please enter the stove IP address.",
    alertSelectVersionOrFile: "Please select a version or a local binary file.",
    alertSelectPort: "Please select a USB port.",
    alertSelectWifiPort: "Please select the dongle USB port.",
    alertFillSsid: "Please enter your Wi-Fi network name (SSID).",
    alertNoStoveDetected: "No stove detected on the network yet. Run a search first.",
    alertErrorPrefix: "Error:"
  }
};

let currentLang = localStorage.getItem("of_lang") || (navigator.language.startsWith("en") ? "en" : "fr");

export function getLang() {
  return currentLang;
}

export function t(key) {
  const dict = translations[currentLang] || translations.fr;
  return dict[key] || translations.fr[key] || key;
}

export function setLanguage(lang, onLangChangeCallback) {
  if (lang !== "fr" && lang !== "en") lang = "fr";
  currentLang = lang;
  localStorage.setItem("of_lang", lang);
  document.documentElement.lang = lang;

  // Update static DOM elements
  document.querySelectorAll("[data-i18n]").forEach((el) => {
    const key = el.getAttribute("data-i18n");
    if (key) el.textContent = t(key);
  });

  document.querySelectorAll("[data-i18n-placeholder]").forEach((el) => {
    const key = el.getAttribute("data-i18n-placeholder");
    if (key) el.placeholder = t(key);
  });

  document.querySelectorAll("[data-i18n-title]").forEach((el) => {
    const key = el.getAttribute("data-i18n-title");
    if (key) el.title = t(key);
  });

  // Sync dropdown value
  const selectEl = document.getElementById("lang-select");
  if (selectEl && selectEl.value !== lang) {
    selectEl.value = lang;
  }

  if (typeof onLangChangeCallback === "function") {
    onLangChangeCallback(lang);
  }
}
