import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { t, setLanguage, getLang } from "./i18n.js";
import { renderMarkdown } from "./markdown.js";

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
const emptyBtnRetry = document.getElementById("empty-btn-retry");
const emptyBtnWifi = document.getElementById("empty-btn-wifi");
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

const confirmModal = document.getElementById("confirm-modal");
const modalTargetPort = document.getElementById("modal-target-port");
const modalTargetFirmware = document.getElementById("modal-target-firmware");
const modalBtnCancel = document.getElementById("modal-btn-cancel");
const modalBtnConfirm = document.getElementById("modal-btn-confirm");

const wifiPortSelect = document.getElementById("wifi-port-select");
const wifiSsid = document.getElementById("wifi-ssid");
const wifiPass = document.getElementById("wifi-pass");
const btnSendWifi = document.getElementById("btn-send-wifi");
const wifiStatusText = document.getElementById("wifi-status-text");

const releasesList = document.getElementById("releases-list");
const btnRefreshReleases = document.getElementById("btn-refresh-releases");

// Language selector dropdown
const langSelect = document.getElementById("lang-select");
if (langSelect) {
  langSelect.addEventListener("change", (e) => {
    setLanguage(e.target.value, () => {
      renderDevices(discoveredDevices);
      renderReleasesList(availableReleases);
      populateReleaseDropdowns(availableReleases);
    });
  });
}

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
let isScanning = false;

async function runScan() {
  if (isScanning) return;
  isScanning = true;

  btnScan.disabled = true;
  if (emptyBtnRetry) emptyBtnRetry.disabled = true;
  btnScan.innerHTML = `<span class="btn-icon">🔄</span> ${t("btnScan")}`;
  scanLoading.classList.remove("hidden");
  emptyDevices.classList.add("hidden");
  devicesContainer.innerHTML = "";

  try {
    discoveredDevices = await invoke("scan_network");
    renderDevices(discoveredDevices);
  } catch (err) {
    console.error("Erreur lors du scan réseau :", err);
    emptyDevices.classList.remove("hidden");
  } finally {
    scanLoading.classList.add("hidden");
    btnScan.disabled = false;
    if (emptyBtnRetry) emptyBtnRetry.disabled = false;
    btnScan.innerHTML = `<span class="btn-icon">🔄</span> ${t("btnScan")}`;
    isScanning = false;
  }
}

function renderDevices(devices) {
  devicesContainer.innerHTML = "";
  if (!devices || devices.length === 0) {
    emptyDevices.classList.remove("hidden");
    footerDongleSummary.textContent = t("footerNoStove");
    return;
  }

  emptyDevices.classList.add("hidden");

  const d = devices[0]; // Exactement 1 poêle ciblé
  const modelName = d.stove_model || t("stoveModelDefault");
  footerDongleSummary.textContent = `${t("footerStoveActive")} ${modelName} (${d.ip})`;

  // Formatage du statut
  let displayState = d.stove_state;
  if (!displayState || displayState === "--" || displayState === "En veille / Connecté") {
    displayState = t("stoveStandby");
  }

  let displayVersion = d.firmware_version;
  if (!displayVersion || displayVersion.trim() === "" || displayVersion.toLowerCase() === "inconnue" || displayVersion.toLowerCase() === "unknown") {
    displayVersion = t("versionUnknown");
  } else if (!displayVersion.startsWith("v") && !displayVersion.startsWith("V")) {
    displayVersion = `v${displayVersion}`;
  }

  const card = document.createElement("div");
  card.className = "single-stove-card";

  card.innerHTML = `
    <div class="stove-card-header">
      <div class="stove-title-row">
        <span class="stove-flame-icon">🔥</span>
        <div>
          <h2 class="stove-model-title">${modelName}</h2>
          <span class="stove-network-name">${d.hostname || "openfirenet.local"}</span>
        </div>
      </div>
      <span class="stove-status-badge">
        <span class="badge-dot"></span>
        ${displayState}
      </span>
    </div>

    <div class="stove-info-grid">
      <div class="info-tile">
        <span class="info-label">${t("labelIp")}</span>
        <a href="http://${d.ip}/" target="_blank" class="info-value info-link" title="http://${d.ip}/">
          ${d.ip} <span class="external-icon">↗</span>
        </a>
      </div>

      <div class="info-tile">
        <span class="info-label">${t("labelHostname")}</span>
        <a href="http://${d.hostname || "openfirenet.local"}/" target="_blank" class="info-value info-link" title="http://${d.hostname || "openfirenet.local"}/">
          ${d.hostname || "openfirenet.local"} <span class="external-icon">↗</span>
        </a>
      </div>

      <div class="info-tile">
        <span class="info-label">${t("labelFirmware")}</span>
        <span class="info-value version-tag">${displayVersion}</span>
      </div>

      <div class="info-tile">
        <span class="info-label">${t("labelWifiSignal")}</span>
        <span class="info-value signal-value">
          <span class="wifi-icon">📶</span> ${d.wifi_rssi || "-62 dBm"}
        </span>
      </div>
    </div>

    <div class="stove-actions-row">
      <button class="btn btn-primary btn-lg btn-open-web" data-ip="${d.ip}">
        <span class="btn-icon">🌐</span> ${t("btnOpenWeb")}
      </button>
      <button class="btn btn-secondary btn-lg btn-update-this" data-ip="${d.ip}">
        <span class="btn-icon">📡</span> ${t("btnUpdateOta")}
      </button>
    </div>
  `;

  devicesContainer.appendChild(card);

  // Événements boutons de la carte
  card.querySelector(".btn-update-this").addEventListener("click", () => {
    otaIpInput.value = d.ip;
    document.querySelector('[data-tab="tab-ota"]').click();
  });

  card.querySelector(".btn-open-web").addEventListener("click", async () => {
    const url = `http://${d.ip}/`;
    try {
      await invoke("open_browser_url", { url });
    } catch (err) {
      window.open(url, "_blank");
    }
  });
}

// Handlers empty-state
if (emptyBtnRetry) {
  emptyBtnRetry.addEventListener("click", runScan);
}
if (emptyBtnWifi) {
  emptyBtnWifi.addEventListener("click", () => {
    document.querySelector('[data-tab="tab-wifi"]').click();
  });
}

// 3. GitHub Releases
async function loadReleases() {
  btnRefreshReleases.disabled = true;
  btnRefreshReleases.innerHTML = `<span class="spinner-btn"></span> ${t("checkingReleases")}`;
  releasesList.innerHTML = `
    <div class="card loading-card">
      <h3>${t("checkingReleases")}</h3>
      <p>${t("checkingReleasesDesc")}</p>
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
    btnRefreshReleases.innerHTML = `<span class="btn-icon">🔄</span> ${t("btnRefreshReleases")}`;
  }
}

function populateReleaseDropdowns(releases) {
  otaReleaseSelect.innerHTML = "";
  usbReleaseSelect.innerHTML = "";

  if (!releases || releases.length === 0) {
    const emptyOpt = document.createElement("option");
    emptyOpt.value = "";
    emptyOpt.textContent = t("noReleaseDesc");
    otaReleaseSelect.appendChild(emptyOpt);

    const emptyOptUsb = document.createElement("option");
    emptyOptUsb.value = "";
    emptyOptUsb.textContent = t("noReleaseDesc");
    usbReleaseSelect.appendChild(emptyOptUsb);
    return;
  }

  releases.forEach((r) => {
    const badgeText = r.prerelease ? t("badgePrerelease") : t("badgeStable");
    const optOta = document.createElement("option");
    optOta.value = r.tag_name;
    optOta.textContent = `${r.tag_name} (${badgeText}) - ${r.name || ""}`;
    otaReleaseSelect.appendChild(optOta);

    const optUsb = document.createElement("option");
    optUsb.value = r.tag_name;
    optUsb.textContent = `${r.tag_name} (${badgeText}) - ${r.name || ""}`;
    usbReleaseSelect.appendChild(optUsb);
  });
}

function formatBytes(bytes) {
  if (!bytes || bytes === 0) return "";
  const k = 1024;
  const sizes = ["o", "Ko", "Mo", "Go"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + " " + sizes[i];
}

function getAssetIcon(name) {
  if (name.endsWith(".minisig")) return "🔐";
  if (name.includes("factory") || name.includes("merged")) return "⚡";
  if (name.includes("ota")) return "📡";
  if (name.includes("SHA256") || name.includes("sha256")) return "📄";
  if (name.endsWith(".bin")) return "💾";
  return "📦";
}

function renderReleasesList(releases) {
  releasesList.innerHTML = "";
  if (!releases || releases.length === 0) {
    releasesList.innerHTML = `
      <div class="empty-state">
        <div class="empty-icon">📦</div>
        <h3>${t("noReleaseTitle")}</h3>
        <p>${t("noReleaseDesc")}</p>
      </div>
    `;
    return;
  }

  releases.forEach((r) => {
    const card = document.createElement("div");
    card.className = "release-card";

    const isPrerelease = r.prerelease;
    const badgeClass = isPrerelease ? "badge-prerelease" : "badge-stable";
    const badgeText = isPrerelease ? t("badgePrerelease") : t("badgeStable");
    const releaseTitle = r.name && r.name.trim() ? r.name : r.tag_name;
    const githubReleaseUrl = `https://github.com/openfirenet/open-firenet/releases/tag/${r.tag_name}`;

    const assetsHtml = (r.assets && r.assets.length > 0)
      ? `
        <div class="release-assets-section">
          <div class="release-assets-title">
            <span>📁</span> <span>${t("availableFiles")}</span>
          </div>
          <div class="release-assets-grid">
            ${r.assets.map((a) => `
              <a href="${a.browser_download_url}" class="asset-chip external-link" title="${a.name}">
                <span class="asset-icon">${getAssetIcon(a.name)}</span>
                <span class="asset-name">${a.name}</span>
                ${a.size ? `<span class="asset-size">(${formatBytes(a.size)})</span>` : ""}
                <span class="ext-icon">↗</span>
              </a>
            `).join("")}
          </div>
        </div>
      `
      : "";

    const hasFactory = r.assets && r.assets.some(a => a.name.includes("factory") || a.name.includes("merged") || (a.name.endsWith(".bin") && !a.name.includes("ota")));
    const hasOta = r.assets && r.assets.some(a => a.name.includes("ota") || a.name.endsWith(".bin"));

    const actionsHtml = `
      <div class="release-actions-row">
        ${hasOta ? `
          <button class="btn btn-secondary btn-sm btn-quick-ota" data-tag="${r.tag_name}">
            <span class="btn-icon">📡</span> ${t("useForOta")}
          </button>
        ` : ""}
        ${hasFactory ? `
          <button class="btn btn-secondary btn-sm btn-quick-usb" data-tag="${r.tag_name}">
            <span class="btn-icon">⚡</span> ${t("useForUsb")}
          </button>
        ` : ""}
      </div>
    `;

    card.innerHTML = `
      <div class="release-card-header">
        <div class="release-title-row">
          <span class="release-tag-badge">${r.tag_name}</span>
          <h3 class="release-title">${releaseTitle}</h3>
        </div>
        <div class="release-badges-row">
          <span class="release-type-badge ${badgeClass}">${badgeText}</span>
          <a href="${githubReleaseUrl}" class="github-link external-link" title="${t("viewOnGithub")}">
            GitHub <span class="ext-icon">↗</span>
          </a>
        </div>
      </div>

      <div class="release-body">
        ${renderMarkdown(r.body)}
      </div>

      ${assetsHtml}
      ${actionsHtml}
    `;

    releasesList.appendChild(card);

    const btnOta = card.querySelector(".btn-quick-ota");
    if (btnOta) {
      btnOta.addEventListener("click", () => {
        otaReleaseSelect.value = r.tag_name;
        const otaTabBtn = document.querySelector('[data-tab="tab-ota"]');
        if (otaTabBtn) otaTabBtn.click();
      });
    }

    const btnUsb = card.querySelector(".btn-quick-usb");
    if (btnUsb) {
      btnUsb.addEventListener("click", () => {
        usbReleaseSelect.value = r.tag_name;
        const usbTabBtn = document.querySelector('[data-tab="tab-usb"]');
        if (usbTabBtn) usbTabBtn.click();
      });
    }
  });
}

// 4. USB Ports
async function refreshPorts() {
  btnRefreshPorts.disabled = true;
  usbPortSelect.innerHTML = `<option>${t("detectingPorts")}</option>`;
  wifiPortSelect.innerHTML = `<option>${t("detectingPorts")}</option>`;

  try {
    detectedPorts = await invoke("list_serial_ports");
    usbPortSelect.innerHTML = "";
    wifiPortSelect.innerHTML = "";

    if (!detectedPorts || detectedPorts.length === 0) {
      usbPortSelect.innerHTML = `<option value=''>${t("noPortDetected")}</option>`;
      wifiPortSelect.innerHTML = `<option value=''>${t("noPortDetected")}</option>`;
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
    usbPortSelect.innerHTML = `<option value=''>${t("alertErrorPrefix")} USB</option>`;
    wifiPortSelect.innerHTML = `<option value=''>${t("alertErrorPrefix")} USB</option>`;
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
    alert(t("alertNoStoveDetected"));
  }
});

btnStartOta.addEventListener("click", async () => {
  const ip = otaIpInput.value.trim();
  if (!ip) {
    alert(t("alertFillIp"));
    return;
  }
  const tag = otaReleaseSelect.value;
  const localFileInput = document.getElementById("ota-file-input");
  const localFile = localFileInput && localFileInput.files && localFileInput.files[0] ? localFileInput.files[0].name : null;

  if (!tag && !localFile) {
    alert(t("alertSelectVersionOrFile"));
    return;
  }

  btnStartOta.disabled = true;
  otaProgressBox.classList.remove("hidden");
  otaProgressBar.style.width = "5%";
  otaStatusText.textContent = t("statusOtaSending");
  otaProgressBox.scrollIntoView({ behavior: "smooth", block: "nearest" });

  try {
    await invoke("update_ota_device", {
      ip,
      releaseTag: tag || null,
      customFile: localFile,
    });
    otaProgressBar.style.width = "100%";
    otaStatusText.textContent = `✔ ${t("statusOtaSuccess")}`;
    setTimeout(() => {
      alert(t("otaSuccessMsg"));
    }, 400);
  } catch (err) {
    console.error("Erreur OTA :", err);
    otaStatusText.textContent = `${t("alertErrorPrefix")} ${err}`;
    alert(`${t("alertErrorPrefix")} ${err}`);
  } finally {
    btnStartOta.disabled = false;
  }
});

// Flash USB avec boîte de dialogue de confirmation préalable
async function doUsbFlash() {
  const port = usbPortSelect.value;
  const tag = usbReleaseSelect.value;
  const localFileInput = document.getElementById("usb-file-input");
  const localFile = localFileInput && localFileInput.files && localFileInput.files[0] ? localFileInput.files[0].name : null;

  btnStartUsbFlash.disabled = true;
  usbProgressBox.classList.remove("hidden");
  usbProgressBar.style.width = "5%";
  usbStatusText.textContent = t("statusFlashingUsb");
  usbProgressBox.scrollIntoView({ behavior: "smooth", block: "nearest" });

  try {
    await invoke("flash_usb_device", {
      port,
      releaseTag: tag || null,
      customFile: localFile,
    });
    usbProgressBar.style.width = "100%";
    usbStatusText.textContent = `✔ ${t("statusFlashingSuccess")}`;
    setTimeout(() => {
      alert(t("statusFlashingSuccess"));
    }, 400);
  } catch (err) {
    console.error("Erreur Flash USB :", err);
    usbStatusText.textContent = `${t("alertErrorPrefix")} ${err}`;
    alert(`${t("alertErrorPrefix")} ${err}`);
  } finally {
    btnStartUsbFlash.disabled = false;
  }
}

btnStartUsbFlash.addEventListener("click", () => {
  const port = usbPortSelect.value;
  if (!port) {
    alert(t("alertSelectPort"));
    return;
  }

  const tag = usbReleaseSelect.value;
  const localFileInput = document.getElementById("usb-file-input");
  const localFile = localFileInput && localFileInput.files && localFileInput.files[0] ? localFileInput.files[0].name : null;

  if (!tag && !localFile) {
    alert(t("alertSelectVersionOrFile"));
    return;
  }

  modalTargetPort.textContent = port;
  modalTargetFirmware.textContent = tag ? `${tag} (Factory Image)` : localFile;
  confirmModal.classList.remove("hidden");
});

modalBtnCancel.addEventListener("click", () => {
  confirmModal.classList.add("hidden");
});

modalBtnConfirm.addEventListener("click", () => {
  confirmModal.classList.add("hidden");
  doUsbFlash();
});

confirmModal.addEventListener("click", (e) => {
  if (e.target === confirmModal) {
    confirmModal.classList.add("hidden");
  }
});

btnSendWifi.addEventListener("click", async () => {
  const port = wifiPortSelect.value;
  const ssid = wifiSsid.value.trim();
  const pass = wifiPass.value;

  if (!port) {
    alert(t("alertSelectWifiPort"));
    return;
  }
  if (!ssid) {
    alert(t("alertFillSsid"));
    return;
  }

  btnSendWifi.disabled = true;
  wifiStatusText.textContent = t("statusSearching");

  try {
    await invoke("configure_wifi", { port, ssid, pass });
    wifiStatusText.textContent = t("wifiConfigSuccess");
    alert(t("wifiConfigSuccess"));
  } catch (err) {
    console.error("Erreur Wi-Fi :", err);
    wifiStatusText.textContent = `${t("alertErrorPrefix")} ${err}`;
    alert(`${t("alertErrorPrefix")} ${err}`);
  } finally {
    btnSendWifi.disabled = false;
  }
});

// Écoute des événements en direct émis par Tauri
listen("flash-status", (event) => {
  if (usbStatusText) usbStatusText.textContent = event.payload;
});

listen("flash-progress", (event) => {
  const payload = event.payload;
  if (!payload) return;
  const percent = typeof payload === "object" ? payload.percent : payload;
  const message = typeof payload === "object" ? payload.message : null;
  if (typeof percent === "number") {
    usbProgressBox.classList.remove("hidden");
    usbProgressBar.style.width = `${percent}%`;
  }
  if (message && usbStatusText) {
    usbStatusText.textContent = message;
  }
});

listen("ota-status", (event) => {
  if (otaStatusText) otaStatusText.textContent = event.payload;
});

listen("ota-progress", (event) => {
  const payload = event.payload;
  if (!payload) return;
  const percent = typeof payload === "object" ? payload.percent : payload;
  const message = typeof payload === "object" ? payload.message : null;
  if (typeof percent === "number") {
    otaProgressBox.classList.remove("hidden");
    otaProgressBar.style.width = `${percent}%`;
  }
  if (message && otaStatusText) {
    otaStatusText.textContent = message;
  }
});

// Intercepter tous les liens externes pour les ouvrir dans le navigateur par défaut de l'utilisateur
document.addEventListener("click", async (e) => {
  const link = e.target.closest("a");
  if (link && link.href) {
    const url = link.href;
    if (url.startsWith("http://") || url.startsWith("https://")) {
      e.preventDefault();
      try {
        await invoke("open_browser_url", { url });
      } catch (err) {
        console.error("Erreur ouverture navigateur natif :", err);
        window.open(url, "_blank");
      }
    }
  }
});

// Initialisation au démarrage
window.addEventListener("DOMContentLoaded", () => {
  // Appliquer la langue détectée ou sauvegardée
  setLanguage(getLang());

  // Lancement des scans asynchrones
  runScan();
  refreshPorts();
  loadReleases();
});

