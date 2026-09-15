import { invoke } from "@tauri-apps/api/core";

// State
let discoveredDevices = [];
let availableReleases = [];
let detectedPorts = [];

// DOM Elements
const tabs = document.querySelectorAll(".tab-btn");
const tabPanes = document.querySelectorAll(".tab-pane");
const btnScan = document.getElementById("btn-scan");
const scanLoading = document.getElementById("scan-loading");
const devicesContainer = document.getElementById("devices-container");
const emptyDevices = document.getElementById("empty-devices");
const headerStatusText = document.getElementById("header-status-text");
const footerDongleSummary = document.getElementById("footer-dongle-summary");

const otaIpInput = document.getElementById("ota-ip");
const btnUseDetectedIp = document.getElementById("btn-use-detected-ip");
const otaReleaseSelect = document.getElementById("ota-release-select");
const btnStartOta = document.getElementById("btn-start-ota");
const otaProgressBox = document.getElementById("ota-progress-box");
const otaProgressBar = document.getElementById("ota-progress-bar");
const otaStatusText = document.getElementById("ota-status-text");

const usbPortSelect = document.getElementById("usb-port-select");
const usbReleaseSelect = document.getElementById("usb-release-select");
const btnRefreshPorts = document.getElementById("btn-refresh-ports");
const btnStartUsbFlash = document.getElementById("btn-start-usb-flash");
const usbProgressBox = document.getElementById("usb-progress-box");
const usbProgressBar = document.getElementById("usb-progress-bar");
const usbStatusText = document.getElementById("usb-status-text");

const wifiPortSelect = document.getElementById("wifi-port-select");
const wifiSsid = document.getElementById("wifi-ssid");
const wifiPass = document.getElementById("wifi-pass");
const btnSendWifi = document.getElementById("btn-send-wifi");
const wifiStatusText = document.getElementById("wifi-status-text");

const releasesList = document.getElementById("releases-list");
const btnRefreshReleases = document.getElementById("btn-refresh-releases");

// 1. Tab Switching
tabs.forEach((tab) => {
  tab.addEventListener("click", () => {
    tabs.forEach((t) => t.classList.remove("active"));
    tabPanes.forEach((p) => p.classList.remove("active"));
    tab.classList.add("active");
    const targetId = tab.getAttribute("data-tab");
    const targetPane = document.getElementById(targetId);
    if (targetPane) targetPane.classList.add("active");
  });
});

// 2. Scan Network
async function runScan() {
  btnScan.disabled = true;
  btnScan.innerHTML = '<span class="spinner-btn"></span> Scan en cours...';
  scanLoading.classList.remove("hidden");
  emptyDevices.classList.add("hidden");
  devicesContainer.innerHTML = "";
  headerStatusText.textContent = "Recherche sur le réseau...";

  try {
    discoveredDevices = await invoke("scan_network");
    renderDevices(discoveredDevices);
  } catch (err) {
    console.error("Erreur lors du scan réseau :", err);
    emptyDevices.classList.remove("hidden");
    headerStatusText.textContent = "Erreur de scan";
  } finally {
    scanLoading.classList.add("hidden");
    btnScan.disabled = false;
    btnScan.innerHTML = '<span class="btn-icon">🔄</span> Scanner le réseau';
  }
}

function renderDevices(devices) {
  devicesContainer.innerHTML = "";
  if (!devices || devices.length === 0) {
    emptyDevices.classList.remove("hidden");
    headerStatusText.textContent = "Aucun poêle trouvé";
    footerDongleSummary.textContent = "Aucun poêle connecté";
    return;
  }

  emptyDevices.classList.add("hidden");
  headerStatusText.textContent = `${devices.length} poêle(s) détecté(s)`;
  footerDongleSummary.textContent = `Poêle actif : ${devices[0].stove_model} (${devices[0].ip})`;

  devices.forEach((d) => {
    const card = document.createElement("div");
    card.className = "device-card";
    
    // Format human-friendly state
    let displayState = d.stove_state;
    if (!displayState || displayState === "--") {
      displayState = "En veille / Connecté";
    }

    let displayVersion = d.firmware_version;
    if (!displayVersion || displayVersion.trim() === "" || displayVersion === "Inconnue") {
      displayVersion = "Inconnue";
    }

    card.innerHTML = `
      <div class="device-header">
        <div class="device-model">🔥 ${d.stove_model || "Poêle RIKA"}</div>
        <span class="device-badge">${displayState}</span>
      </div>
      <div class="device-details">
        <div>Adresse IP : <strong>${d.ip}</strong></div>
        <div>Signal Wi-Fi : <strong>${d.wifi_rssi}</strong></div>
        <div>Version : <strong>${displayVersion}</strong></div>
        <div>Nom réseau : <strong>${d.hostname}</strong></div>
      </div>
      <div class="device-actions">
        <button class="btn btn-primary btn-sm btn-update-this" data-ip="${d.ip}">
          📡 Mettre à jour en Wi-Fi (OTA)
        </button>
        <button class="btn btn-secondary btn-sm btn-open-web" data-ip="${d.ip}">
          🌐 Ouvrir l'interface Web
        </button>
      </div>
    `;
    devicesContainer.appendChild(card);
  });

  // Attach click handlers
  document.querySelectorAll(".btn-update-this").forEach((btn) => {
    btn.addEventListener("click", (e) => {
      const ip = e.currentTarget.getAttribute("data-ip");
      otaIpInput.value = ip;
      document.querySelector('[data-tab="tab-ota"]').click();
    });
  });

  document.querySelectorAll(".btn-open-web").forEach((btn) => {
    btn.addEventListener("click", async (e) => {
      const ip = e.currentTarget.getAttribute("data-ip");
      const url = `http://${ip}/`;
      try {
        await invoke("open_browser_url", { url });
      } catch (err) {
        console.error("Erreur ouverture navigateur :", err);
        window.open(url, "_blank");
      }
    });
  });
}

// 3. GitHub Releases
async function loadReleases() {
  btnRefreshReleases.disabled = true;
  btnRefreshReleases.innerHTML = '<span class="spinner-btn"></span> Vérification...';
  releasesList.innerHTML = `
    <div class="card loading-card">
      <div class="big-spinner"></div>
      <h3>Interrogation de GitHub...</h3>
      <p>Vérification des versions officielles sur openfirenet/open-firenet...</p>
      <div class="indeterminate-progress-bar">
        <div class="indeterminate-progress-fill"></div>
      </div>
    </div>
  `;

  try {
    availableReleases = await invoke("get_releases");
    populateReleaseDropdowns(availableReleases);
    renderReleasesList(availableReleases);
  } catch (err) {
    console.error("Erreur récupération releases :", err);
    populateReleaseDropdowns([]);
    renderReleasesList([]);
  } finally {
    btnRefreshReleases.disabled = false;
    btnRefreshReleases.innerHTML = '<span class="btn-icon">🔄</span> Recharger';
  }
}

function populateReleaseDropdowns(releases) {
  otaReleaseSelect.innerHTML = "";
  usbReleaseSelect.innerHTML = "";

  if (!releases || releases.length === 0) {
    const emptyOpt = document.createElement("option");
    emptyOpt.value = "";
    emptyOpt.textContent = "Aucune release GitHub officielle disponible (utilisez un fichier .bin local)";
    otaReleaseSelect.appendChild(emptyOpt);

    const emptyOptUsb = document.createElement("option");
    emptyOptUsb.value = "";
    emptyOptUsb.textContent = "Aucune release GitHub officielle disponible (utilisez un fichier .bin local)";
    usbReleaseSelect.appendChild(emptyOptUsb);
    return;
  }

  releases.forEach((r) => {
    const optOta = document.createElement("option");
    optOta.value = r.tag_name;
    optOta.textContent = `${r.tag_name} ${r.prerelease ? "(Pré-release)" : "(Stable)"} - ${r.name || ""}`;
    otaReleaseSelect.appendChild(optOta);

    const optUsb = document.createElement("option");
    optUsb.value = r.tag_name;
    optUsb.textContent = `${r.tag_name} ${r.prerelease ? "(Pré-release)" : "(Stable)"} - ${r.name || ""}`;
    usbReleaseSelect.appendChild(optUsb);
  });
}

function renderReleasesList(releases) {
  releasesList.innerHTML = "";
  if (!releases || releases.length === 0) {
    releasesList.innerHTML = `
      <div class="empty-state">
        <div class="empty-icon">📦</div>
        <h3>Aucune version officielle publiée pour le moment</h3>
        <p>Le dépôt GitHub <code>openfirenet/open-firenet</code> n'a pas encore de release formalisée.<br>
        Vous pouvez flasher ou mettre à jour directement votre clé à l'aide d'un fichier <code>.bin</code> local.</p>
      </div>
    `;
    return;
  }

  releases.forEach((r) => {
    const item = document.createElement("div");
    item.className = "card mb-3";
    item.style.marginBottom = "14px";
    item.innerHTML = `
      <div style="display:flex; justify-content:space-between; align-items:center;">
        <h3>${r.tag_name} - ${r.name || "Release"}</h3>
        <span class="device-badge">${r.prerelease ? "Pré-release" : "Stable"}</span>
      </div>
      <p style="font-size:13px; color:var(--text-muted); margin: 8px 0;">${r.body || "Aucune note de version."}</p>
      <div style="font-size:12px; color:var(--text-muted);">
        Fichiers disponibles : ${r.assets.map((a) => a.name).join(", ")}
      </div>
    `;
    releasesList.appendChild(item);
  });
}

// 4. USB Ports
async function refreshPorts() {
  btnRefreshPorts.disabled = true;
  usbPortSelect.innerHTML = "<option>Recherche des ports USB...</option>";
  wifiPortSelect.innerHTML = "<option>Recherche des ports USB...</option>";

  try {
    detectedPorts = await invoke("list_serial_ports");
    usbPortSelect.innerHTML = "";
    wifiPortSelect.innerHTML = "";

    if (!detectedPorts || detectedPorts.length === 0) {
      usbPortSelect.innerHTML = "<option value=''>Aucun port série détecté (branchez votre clé en USB)</option>";
      wifiPortSelect.innerHTML = "<option value=''>Aucun port série détecté (branchez votre clé en USB)</option>";
      return;
    }

    detectedPorts.forEach((p) => {
      const opt = document.createElement("option");
      opt.value = p.port_name;
      opt.textContent = `${p.port_name} • ${p.description}`;
      usbPortSelect.appendChild(opt);

      const optWifi = document.createElement("option");
      optWifi.value = p.port_name;
      optWifi.textContent = `${p.port_name} • ${p.description}`;
      wifiPortSelect.appendChild(optWifi);
    });
  } catch (err) {
    console.error("Erreur détection ports :", err);
    usbPortSelect.innerHTML = "<option value=''>Erreur d'accès aux ports série</option>";
    wifiPortSelect.innerHTML = "<option value=''>Erreur d'accès aux ports série</option>";
  } finally {
    btnRefreshPorts.disabled = false;
  }
}

// 5. Actions
btnScan.addEventListener("click", runScan);
btnRefreshPorts.addEventListener("click", refreshPorts);
btnRefreshReleases.addEventListener("click", loadReleases);

btnUseDetectedIp.addEventListener("click", () => {
  if (discoveredDevices.length > 0) {
    otaIpInput.value = discoveredDevices[0].ip;
  } else {
    alert("Aucun poêle détecté sur le réseau pour le moment. Lancez un scan d'abord.");
  }
});

btnStartOta.addEventListener("click", async () => {
  const ip = otaIpInput.value.trim();
  if (!ip) {
    alert("Veuillez renseigner l'adresse IP du poêle.");
    return;
  }
  const tag = otaReleaseSelect.value;
  const localFileInput = document.getElementById("ota-file-input");
  const localFile = localFileInput && localFileInput.files && localFileInput.files[0] ? localFileInput.files[0].name : null;

  if (!tag && !localFile) {
    alert("Veuillez sélectionner un fichier .bin local (aucune release GitHub disponible pour l'instant).");
    return;
  }

  otaProgressBox.classList.remove("hidden");
  otaProgressBar.style.width = "20%";
  otaProgressBar.style.backgroundColor = "";
  otaStatusText.textContent = "Téléchargement / préparation du firmware...";

  try {
    otaProgressBar.style.width = "50%";
    const res = await invoke("update_ota_device", {
      ip: ip,
      releaseTag: tag || null,
      customFile: localFile || null,
    });
    otaProgressBar.style.width = "100%";
    otaStatusText.textContent = res;
  } catch (err) {
    otaProgressBar.style.width = "100%";
    otaProgressBar.style.backgroundColor = "var(--danger-red)";
    otaStatusText.textContent = `Erreur : ${err}`;
  }
});

btnStartUsbFlash.addEventListener("click", async () => {
  const port = usbPortSelect.value;
  if (!port) {
    alert("Veuillez brancher la clé en USB et sélectionner son port série.");
    return;
  }
  const tag = usbReleaseSelect.value;
  const localFileInput = document.getElementById("usb-file-input");
  const localFile = localFileInput && localFileInput.files && localFileInput.files[0] ? localFileInput.files[0].name : null;

  if (!tag && !localFile) {
    alert("Veuillez sélectionner un fichier .bin local (aucune release GitHub disponible pour l'instant).");
    return;
  }

  usbProgressBox.classList.remove("hidden");
  usbProgressBar.style.width = "20%";
  usbProgressBar.style.backgroundColor = "";
  usbStatusText.textContent = "Connexion au bootloader ESP32-S3...";

  try {
    usbProgressBar.style.width = "50%";
    const res = await invoke("flash_usb_device", {
      port: port,
      releaseTag: tag || null,
      customFile: localFile || null,
    });
    usbProgressBar.style.width = "100%";
    usbStatusText.textContent = res;
  } catch (err) {
    usbProgressBar.style.width = "100%";
    usbProgressBar.style.backgroundColor = "var(--danger-red)";
    usbStatusText.textContent = `Erreur : ${err}`;
  }
});

btnSendWifi.addEventListener("click", async () => {
  const port = wifiPortSelect.value;
  const ssid = wifiSsid.value.trim();
  const pass = wifiPass.value;

  if (!port || !ssid) {
    alert("Veuillez sélectionner un port USB et renseigner le nom Wi-Fi (SSID).");
    return;
  }

  wifiStatusText.textContent = "Envoi des identifiants au poêle...";

  try {
    const res = await invoke("configure_wifi", { port, ssid, password: pass });
    wifiStatusText.textContent = res;
    wifiStatusText.style.color = "var(--success-green)";
  } catch (err) {
    wifiStatusText.textContent = `Erreur : ${err}`;
    wifiStatusText.style.color = "var(--danger-red)";
  }
});

// Auto-run on startup
function init() {
  requestAnimationFrame(() => {
    setTimeout(() => {
      runScan();
      loadReleases();
      refreshPorts();
    }, 50);
  });
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", init);
} else {
  init();
}
