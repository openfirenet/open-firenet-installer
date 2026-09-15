// Open-Firenet Installer Frontend Logic

let invoke = null;
let listen = null;

// Dynamically import Tauri API if running in Tauri
if (window.__TAURI_INTERNALS__) {
  import("@tauri-apps/api/core").then((core) => {
    invoke = core.invoke;
  });
  import("@tauri-apps/api/event").then((event) => {
    listen = event.listen;
  });
}

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
  scanLoading.classList.remove("hidden");
  emptyDevices.classList.add("hidden");
  devicesContainer.innerHTML = "";
  headerStatusText.textContent = "Scan en cours...";

  try {
    if (invoke) {
      discoveredDevices = await invoke("scan_network");
    } else {
      // Mock for browser preview / local dev
      await new Promise((r) => setTimeout(r, 1200));
      discoveredDevices = [
        {
          ip: "192.168.1.93",
          hostname: "openfirenet.local",
          stove_model: "DOMO",
          stove_state: "En fonctionnement (Flamme)",
          wifi_rssi: "-51 dBm (Excellent)",
          firmware_version: "v1.9.0",
        },
      ];
    }
    renderDevices(discoveredDevices);
  } catch (err) {
    console.error("Erreur scan :", err);
    emptyDevices.classList.remove("hidden");
    headerStatusText.textContent = "Erreur de scan";
  } finally {
    scanLoading.classList.add("hidden");
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
    card.innerHTML = `
      <div class="device-header">
        <div class="device-model">🔥 ${d.stove_model || "Poêle RIKA"}</div>
        <span class="device-badge">${d.stove_state || "Connecté"}</span>
      </div>
      <div class="device-details">
        <div>IP : <strong>${d.ip}</strong></div>
        <div>Signal : <strong>${d.wifi_rssi}</strong></div>
        <div>Version : <strong>${d.firmware_version || "Inconnue"}</strong></div>
        <div>Hôte : <strong>${d.hostname}</strong></div>
      </div>
      <div class="device-actions">
        <button class="btn btn-primary btn-sm btn-update-this" data-ip="${d.ip}">
          📡 Mettre à jour en Wi-Fi (OTA)
        </button>
        <a href="http://${d.ip}" target="_blank" class="btn btn-secondary btn-sm">
          🌐 Ouvrir l'interface Web
        </a>
      </div>
    `;
    devicesContainer.appendChild(card);
  });

  // Attach click handler for "Mettre à jour"
  document.querySelectorAll(".btn-update-this").forEach((btn) => {
    btn.addEventListener("click", (e) => {
      const ip = e.currentTarget.getAttribute("data-ip");
      otaIpInput.value = ip;
      // Switch to OTA tab
      document.querySelector('[data-tab="tab-ota"]').click();
    });
  });
}

// 3. GitHub Releases
async function loadReleases() {
  try {
    if (invoke) {
      availableReleases = await invoke("get_releases");
    } else {
      // Mock for browser dev
      availableReleases = [
        {
          tag_name: "v2.0.0",
          name: "Open-Firenet v2.0.0 Stable",
          body: "Release majeure avec support 26 modèles RIKA.",
          prerelease: false,
          assets: [
            { name: "open-firenet-factory.bin", browser_download_url: "#", size: 1450000 },
            { name: "open-firenet-ota.bin", browser_download_url: "#", size: 950000 },
          ],
        },
        {
          tag_name: "v1.9.0",
          name: "Open-Firenet v1.9.0",
          body: "Version stable précédente.",
          prerelease: false,
          assets: [
            { name: "open-firenet-factory.bin", browser_download_url: "#", size: 1400000 },
            { name: "open-firenet-ota.bin", browser_download_url: "#", size: 920000 },
          ],
        },
      ];
    }

    populateReleaseDropdowns(availableReleases);
    renderReleasesList(availableReleases);
  } catch (err) {
    console.error("Erreur récupération releases :", err);
  }
}

function populateReleaseDropdowns(releases) {
  otaReleaseSelect.innerHTML = "";
  usbReleaseSelect.innerHTML = "";

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
  usbPortSelect.innerHTML = "<option>Détection...</option>";
  wifiPortSelect.innerHTML = "<option>Détection...</option>";

  try {
    if (invoke) {
      detectedPorts = await invoke("list_serial_ports");
    } else {
      detectedPorts = [
        { port_name: "/dev/ttyACM0", description: "🔥 ESP32-S3 (Open-Firenet)", is_esp: true },
        { port_name: "/dev/ttyUSB0", description: "USB Serial Generic", is_esp: false },
      ];
    }

    usbPortSelect.innerHTML = "";
    wifiPortSelect.innerHTML = "";

    if (detectedPorts.length === 0) {
      usbPortSelect.innerHTML = "<option value=''>Aucun port série détecté</option>";
      wifiPortSelect.innerHTML = "<option value=''>Aucun port série détecté</option>";
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
  }
}

// 5. Actions
btnScan.addEventListener("click", runScan);
btnRefreshPorts.addEventListener("click", refreshPorts);
btnRefreshReleases.addEventListener("click", loadReleases);

btnUseDetectedIp.addEventListener("click", () => {
  if (discoveredDevices.length > 0) {
    otaIpInput.value = discoveredDevices[0].ip;
  }
});

btnStartOta.addEventListener("click", async () => {
  const ip = otaIpInput.value.trim();
  if (!ip) {
    alert("Veuillez saisir ou sélectionner une adresse IP.");
    return;
  }
  const tag = otaReleaseSelect.value;
  otaProgressBox.classList.remove("hidden");
  otaProgressBar.style.width = "20%";
  otaStatusText.textContent = "Téléchargement du firmware et préparation...";

  try {
    if (invoke) {
      otaProgressBar.style.width = "50%";
      const res = await invoke("update_ota_device", {
        ip: ip,
        releaseTag: tag || null,
        customFile: null,
      });
      otaProgressBar.style.width = "100%";
      otaStatusText.textContent = res;
    } else {
      await new Promise((r) => setTimeout(r, 2000));
      otaProgressBar.style.width = "100%";
      otaStatusText.textContent = "Mise à jour réussie ! Le poêle redémarre.";
    }
  } catch (err) {
    otaProgressBar.style.width = "100%";
    otaProgressBar.style.backgroundColor = "var(--danger-red)";
    otaStatusText.textContent = `Erreur : ${err}`;
  }
});

btnStartUsbFlash.addEventListener("click", async () => {
  const port = usbPortSelect.value;
  if (!port) {
    alert("Veuillez brancher et sélectionner votre port USB.");
    return;
  }
  const tag = usbReleaseSelect.value;
  usbProgressBox.classList.remove("hidden");
  usbProgressBar.style.width = "20%";
  usbStatusText.textContent = "Connexion au bootloader USB...";

  try {
    if (invoke) {
      usbProgressBar.style.width = "50%";
      const res = await invoke("flash_usb_device", {
        port: port,
        releaseTag: tag || null,
        customFile: null,
      });
      usbProgressBar.style.width = "100%";
      usbStatusText.textContent = res;
    } else {
      await new Promise((r) => setTimeout(r, 2500));
      usbProgressBar.style.width = "100%";
      usbStatusText.textContent = "Flashage terminé avec succès !";
    }
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
    if (invoke) {
      const res = await invoke("configure_wifi", { port, ssid, password: pass });
      wifiStatusText.textContent = res;
      wifiStatusText.style.color = "var(--success-green)";
    } else {
      await new Promise((r) => setTimeout(r, 1000));
      wifiStatusText.textContent = "Identifiants Wi-Fi envoyés !";
      wifiStatusText.style.color = "var(--success-green)";
    }
  } catch (err) {
    wifiStatusText.textContent = `Erreur : ${err}`;
    wifiStatusText.style.color = "var(--danger-red)";
  }
});

// Auto-run on startup
window.addEventListener("DOMContentLoaded", () => {
  runScan();
  loadReleases();
  refreshPorts();
});
