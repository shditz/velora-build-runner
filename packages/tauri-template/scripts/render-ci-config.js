import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { PNG } from "pngjs";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const templateRoot = path.resolve(__dirname, "..");
const srcTauriDir = path.join(templateRoot, "src-tauri");

function ensureRgbaPng(filePath) {
  try {
    if (!fs.existsSync(filePath)) return;
    const buf = fs.readFileSync(filePath);
    const isPng =
      buf.length > 26 &&
      buf[0] === 0x89 &&
      buf[1] === 0x50 &&
      buf[2] === 0x4e &&
      buf[3] === 0x47;
    if (!isPng) return;
    // Tauri's generate_context! proc macro panics if any icon is not 32-bit RGBA (bitDepth 8, colorType 6)
    if (buf[24] === 8 && buf[25] === 6) {
      return;
    }
    console.log(
      `[RenderConfig] Icon ${path.basename(filePath)} is not 32-bit RGBA (bitDepth: ${buf[24]}, colorType: ${buf[25]}). Normalizing to RGBA...`,
    );
    const parsed = PNG.sync.read(buf);
    const rgbaBuf = PNG.sync.write(parsed, { colorType: 6 });
    fs.writeFileSync(filePath, rgbaBuf);
    console.log(
      `[RenderConfig] Successfully converted ${path.basename(filePath)} to 32-bit RGBA (colorType: 6).`,
    );
  } catch (err) {
    console.warn(
      `[RenderConfig] Warning: Failed to convert ${filePath} to RGBA: ${err.message}`,
    );
  }
}

function renderIndexHtml(splash, websiteUrl, appName) {
  const bgType = splash.bgType || "solid";
  const gradPreset = splash.gradientPreset || "obsidian-violet";
  const solidBg = splash.bgColor || "#0d0e12";
  const gradients = {
    sunset: "linear-gradient(135deg, #1e1b4b 0%, #431407 50%, #18181b 100%)",
    aurora: "linear-gradient(135deg, #022c22 0%, #082f49 50%, #0f172a 100%)",
    cosmic: "linear-gradient(135deg, #09090b 0%, #2e1065 50%, #030712 100%)",
    ocean: "linear-gradient(135deg, #0c4a6e 0%, #1e1b4b 100%)",
    dark: "linear-gradient(180deg, #18181b 0%, #09090b 100%)",
    "obsidian-violet":
      "linear-gradient(135deg, #0f0728 0%, #180b38 40%, #09090b 100%)",
    "cyber-emerald":
      "linear-gradient(135deg, #022c22 0%, #064e3b 50%, #021f18 100%)",
    "midnight-blue":
      "linear-gradient(135deg, #031525 0%, #0c2340 50%, #020b14 100%)",
  };
  const effectiveBg =
    bgType === "gradient" && gradients[gradPreset]
      ? gradients[gradPreset]
      : solidBg;
  const splashMode = splash.mode || "visual";
  const customHtml = splash.customHtml || "";
  const customCss = splash.customCss || "";
  const durationMs = Number(splash.durationMs || 2000);
  const title = splash.title || appName;
  const subtext =
    splash.subtext !== undefined
      ? splash.subtext
      : splash.loadingText !== undefined
        ? splash.loadingText
        : "Loading application...";
  const subtextBadge =
    splash.subtextBadge !== undefined ? splash.subtextBadge : "";
  const spinnerStyle = splash.spinnerStyle || "orbit";
  const imageMode = splash.image || "";
  const iconDataUrl = splash.iconDataUrl || "";
  const logoSize = Math.max(48, Math.min(120, Number(splash.logoSize || 72)));
  const showContainer = splash.showLogoContainer !== false;
  const entrance = splash.entranceAnimation || "fade";
  const footerText = splash.footerText || "";
  const versionBadge = splash.versionBadge || "";
  const initial = (appName.match(/[a-zA-Z0-9]/)?.[0] || "V").toUpperCase();

  let bodyContent = "";
  if (splashMode === "custom" && customHtml.trim().length > 0) {
    bodyContent = `<style>${customCss}</style>${customHtml}`;
  } else {
    const boxStyle = showContainer
      ? `width:${logoSize}px;height:${logoSize}px;border-radius:18px;background:rgba(24,24,27,0.9);border:1px solid rgba(255,255,255,0.1);display:flex;align-items:center;justify-content:center;box-shadow:0 12px 30px rgba(0,0,0,0.5);backdrop-filter:blur(12px);-webkit-backdrop-filter:blur(12px);overflow:hidden;margin:0 auto;`
      : `width:${logoSize}px;height:${logoSize}px;display:flex;align-items:center;justify-content:center;overflow:hidden;margin:0 auto;`;

    let iconHtml = "";
    if (imageMode === "sparkles" || imageMode === "emblem") {
      iconHtml = `<div style="position:relative;margin-bottom:1.25rem"><div style="${boxStyle}"><svg style="width:${Math.round(logoSize * 0.45)}px;height:${Math.round(logoSize * 0.45)}px;color:#e4e4e7" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="5"/><circle cx="12" cy="12" r="4"/></svg></div></div>`;
    } else if (imageMode === "none") {
      iconHtml = "";
    } else if (
      imageMode &&
      (imageMode.startsWith("data:image") || imageMode.startsWith("http"))
    ) {
      iconHtml = `<div style="position:relative;margin-bottom:1.25rem"><div style="${boxStyle}"><img src="${imageMode}" alt="Splash Graphic" style="width:100%;height:100%;object-fit:${showContainer ? "cover" : "contain"};border-radius:${showContainer ? "18px" : "0"};" /></div></div>`;
    } else if (iconDataUrl && iconDataUrl.length > 50) {
      iconHtml = `<div style="position:relative;margin-bottom:1.25rem"><div style="${boxStyle}"><img src="${iconDataUrl}" alt="App Icon" style="width:${showContainer ? "70%" : "100%"};height:${showContainer ? "70%" : "100%"};object-fit:contain;border-radius:${showContainer ? "10px" : "0"};" /></div></div>`;
    } else {
      iconHtml = `<div style="position:relative;margin-bottom:1.25rem"><div style="${showContainer ? boxStyle : `width:${logoSize}px;height:${logoSize}px;border-radius:18px;background:linear-gradient(180deg,#27272a,#18181b);border:1px solid rgba(255,255,255,0.15);display:flex;align-items:center;justify-content:center;box-shadow:0 12px 30px rgba(0,0,0,0.5);color:#fff;font-weight:600;font-size:${Math.round(logoSize * 0.38)}px;margin:0 auto;`}">${initial}</div></div>`;
    }

    let spHtml = "";
    if (spinnerStyle === "bar") {
      spHtml =
        '<div style="position:relative;width:140px;height:3px;border-radius:9999px;background:rgba(255,255,255,0.12);overflow:hidden;margin:0 auto 1.5rem"><div style="position:absolute;top:0;bottom:0;left:0;width:50px;border-radius:9999px;background:#e4e4e7;animation:velora-bar-slide 1.2s cubic-bezier(0.4,0,0.2,1) infinite"></div></div>';
    } else if (spinnerStyle === "pulse") {
      spHtml =
        '<div style="display:flex;align-items:center;justify-content:center;height:20px;margin:0 auto 1.5rem"><span style="width:8px;height:8px;border-radius:50%;background:#d4d4d8;animation:velora-pulse-fade 1.5s ease-in-out infinite"></span></div>';
    } else if (spinnerStyle === "dots") {
      spHtml =
        '<div style="display:flex;align-items:center;justify-content:center;gap:6px;height:20px;margin:0 auto 1.5rem"><span style="width:5px;height:5px;border-radius:50%;background:#e4e4e7;animation:velora-bounce-dot 1s cubic-bezier(0.34,1.56,0.64,1) infinite"></span><span style="width:5px;height:5px;border-radius:50%;background:#e4e4e7;animation:velora-bounce-dot 1s cubic-bezier(0.34,1.56,0.64,1) infinite;animation-delay:0.15s"></span><span style="width:5px;height:5px;border-radius:50%;background:#e4e4e7;animation:velora-bounce-dot 1s cubic-bezier(0.34,1.56,0.64,1) infinite;animation-delay:0.3s"></span></div>';
    } else if (spinnerStyle === "progress") {
      spHtml =
        '<div style="width:170px;margin:0 auto 1.5rem"><div style="height:3px;border-radius:9999px;background:rgba(255,255,255,0.12);overflow:hidden;margin-bottom:6px"><div id="velora-progress-bar" style="height:100%;width:0%;border-radius:9999px;background:#e4e4e7"></div></div><div style="display:flex;justify-content:space-between;font-size:10px;font-family:ui-monospace,monospace;color:rgba(255,255,255,0.5)"><span>Loading</span><span id="velora-progress-val">0%</span></div></div>';
    } else {
      spHtml =
        '<div style="display:flex;align-items:center;justify-content:center;height:24px;margin:0 auto 1.5rem"><svg style="width:22px;height:22px;animation:velora-spin-cw 0.9s linear infinite" viewBox="0 0 24 24" fill="none"><circle cx="12" cy="12" r="10" stroke="rgba(255,255,255,0.15)" stroke-width="2.5"></circle><path fill="currentColor" opacity="0.9" d="M4 12a8 8 0 018-8v4a4 4 0 00-4 4H4z"></path></svg></div>';
    }

    const safeSubtext = (subtext || "").trim();
    const safeBadge = (subtextBadge || "").trim();
    const subtextHtml = safeSubtext
      ? `<div style="font-size:0.75rem;color:rgba(255,255,255,0.45);margin-bottom:${safeBadge ? "0.5rem" : "1.5rem"};letter-spacing:0.01em">${safeSubtext}</div>`
      : "";
    const badgeHtml = safeBadge
      ? `<div style="display:inline-flex;align-items:center;gap:6px;padding:2px 10px;border-radius:99px;font-size:10px;font-weight:500;background:rgba(255,255,255,0.08);border:1px solid rgba(255,255,255,0.12);color:#d4d4d8;margin-bottom:1.5rem;letter-spacing:0.02em"><span style="width:5px;height:5px;border-radius:50%;background:#34d399"></span>${safeBadge}</div>`
      : "";
    const titleHtml = `<h1 style="margin:0 0 .25rem;font-size:1.25rem;font-weight:600;letter-spacing:-0.02em;color:#ffffff">${title}</h1>`;
    const footerHtml =
      versionBadge || footerText
        ? `<div style="position:absolute;bottom:1.5rem;left:0;right:0;text-align:center;font-size:11px;color:rgba(255,255,255,0.4);font-family:system-ui,sans-serif">${versionBadge ? `<span style="font-family:monospace;font-size:10px;color:rgba(255,255,255,0.5);display:block;margin-bottom:2px">v${versionBadge}</span>` : ""}${footerText}</div>`
        : "";

    let entranceStyle = "animation: velora-splash-fade 0.4s ease forwards;";
    if (entrance === "zoom")
      entranceStyle =
        "animation: velora-splash-zoom 0.5s cubic-bezier(0.16, 1, 0.3, 1) forwards;";
    else if (entrance === "shimmer")
      entranceStyle =
        "animation: velora-splash-breathe 2.5s ease-in-out infinite alternate;";
    else if (entrance === "none") entranceStyle = "";

    bodyContent = `
      <div style="text-align:center;padding:2rem;max-width:380px;display:flex;flex-direction:column;align-items:center;${entranceStyle}">
        ${iconHtml}
        ${titleHtml}
        ${subtextHtml}
        ${badgeHtml || (subtextHtml ? "" : '<div style="margin-bottom:1.5rem"></div>')}
        ${spHtml}
      </div>
      ${footerHtml}
    `;
  }

  return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>${title}</title>
  <style>
    *, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }
    html, body {
      width: 100%;
      height: 100%;
      overflow: hidden;
      background: ${effectiveBg};
      color: #fff;
      font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
      user-select: none;
      -webkit-user-select: none;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
    }
    @keyframes velora-splash-fade{from{opacity:0;transform:translateY(6px)}to{opacity:1;transform:translateY(0)}}
    @keyframes velora-splash-zoom{from{opacity:0;transform:scale(0.9)}to{opacity:1;transform:scale(1)}}
    @keyframes velora-splash-breathe{0%{opacity:0.8;transform:scale(0.98)}100%{opacity:1;transform:scale(1.02)}}
    @keyframes velora-spin-cw{0%{transform:rotate(0deg)}100%{transform:rotate(360deg)}}
    @keyframes velora-bar-slide{0%{transform:translateX(-50px)}100%{transform:translateX(140px)}}
    @keyframes velora-pulse-fade{0%,100%{opacity:0.3;transform:scale(0.85)}50%{opacity:1;transform:scale(1.15)}}
    @keyframes velora-bounce-dot{0%,100%{transform:translateY(0);opacity:0.4}50%{transform:translateY(-6px);opacity:1}}
  </style>
</head>
<body>
  ${bodyContent}
  <script>
    (function(){
      var targetUrl = ${JSON.stringify(websiteUrl)};
      var minDuration = ${durationMs};
      var start = Date.now();

      if (${spinnerStyle === "progress"}) {
        var progStart = Date.now();
        var progDuration = minDuration;
        var pBar = document.getElementById('velora-progress-bar');
        var pVal = document.getElementById('velora-progress-val');
        var progTimer = setInterval(function() {
          var pct = Math.min(100, Math.round(((Date.now() - progStart) / progDuration) * 100));
          if (pBar) pBar.style.width = pct + '%';
          if (pVal) pVal.textContent = pct + '%';
          if (pct >= 100) clearInterval(progTimer);
        }, 50);
      }

      function proceed(){
        var elapsed = Date.now() - start;
        var remain = Math.max(0, minDuration - elapsed);
        setTimeout(function(){
          if (navigator.onLine === false) {
            window.location.replace('offline.html');
            return;
          }
          window.location.replace(targetUrl);
        }, remain);
      }

      if (document.readyState === 'complete') {
        proceed();
      } else {
        window.addEventListener('load', proceed);
      }
    })();
  </script>
</body>
</html>`;
}

function renderOfflineHtml(offline, websiteUrl, appName) {
  const title = offline.title || "No Internet Connection";
  const msg =
    offline.message || "Please check your internet connection and try again.";
  const retry = offline.retryText || "Try Again";
  const bg = offline.themeBg || "#090a0c";
  const accent = offline.accentColor || "amber";
  const iconType = offline.icon || "wifi-off";
  const autoRecon = Boolean(offline.autoReconnect ?? true);
  const interval = (Number(offline.reconnectInterval) || 10) * 1000;
  const helpTxt = offline.helpText || "";
  const helpUrl = offline.helpUrl || "";
  const showDiag = Boolean(offline.showDiagnostics ?? false);
  const offlineMode = offline.mode || "visual";
  const customHtml = offline.customHtml || "";
  const customCss = offline.customCss || "";
  const layout = offline.layout || "glass";
  const graphicMode = offline.graphicMode || "icon";
  const customGraphicUrl = offline.customGraphicUrl || "";

  const colors = {
    emerald: {
      emblemBg: "rgba(16,185,129,0.12)",
      border: "rgba(16,185,129,0.25)",
      text: "#34d399",
      dot: "#10b981",
    },
    cyan: {
      emblemBg: "rgba(6,182,212,0.12)",
      border: "rgba(6,182,212,0.25)",
      text: "#22d3ee",
      dot: "#06b6d4",
    },
    rose: {
      emblemBg: "rgba(244,63,94,0.12)",
      border: "rgba(244,63,94,0.25)",
      text: "#fb7185",
      dot: "#f43f5e",
    },
    violet: {
      emblemBg: "rgba(139,92,246,0.12)",
      border: "rgba(139,92,246,0.25)",
      text: "#a78bfa",
      dot: "#8b5cf6",
    },
    amber: {
      emblemBg: "rgba(245,158,11,0.12)",
      border: "rgba(245,158,11,0.25)",
      text: "#fbbf24",
      dot: "#f59e0b",
    },
  };
  const c = colors[accent] || colors.amber;

  const svgIcons = {
    "cloud-off":
      '<svg style="width:28px;height:28px;color:#e4e4e7" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m2 2 20 20"/><path d="M5.782 5.782A7 7 0 0 0 9 19h8.5a4.5 4.5 0 0 0 1.307-.193"/><path d="M21.532 16.5A4.5 4.5 0 0 0 17.5 10h-1.79A7.008 7.008 0 0 0 10 5.07"/></svg>',
    alert:
      '<svg style="width:28px;height:28px;color:#e4e4e7" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>',
    shield:
      '<svg style="width:28px;height:28px;color:#e4e4e7" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>',
    plug: '<svg style="width:28px;height:28px;color:#e4e4e7" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22v-5"/><path d="M9 8V2"/><path d="M15 8V2"/><path d="M18 8v5a4 4 0 0 1-4 4h-4a4 4 0 0 1-4-4V8Z"/></svg>',
    radar:
      '<svg style="width:28px;height:28px;color:#e4e4e7" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M19.07 4.93A10 10 0 0 0 4.93 19.07"/><path d="M16.24 7.76A6 6 0 0 0 7.76 16.24"/><circle cx="12" cy="12" r="2"/><line x1="12" y1="12" x2="20" y2="4"/></svg>',
    "wifi-off":
      '<svg style="width:28px;height:28px;color:#e4e4e7" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="2" y1="2" x2="22" y2="22"/><path d="M8.5 16.5a5 5 0 0 1 7 0"/><path d="M2 8.82a15 15 0 0 1 4.17-2.65"/><path d="M10.66 5c4.01-.36 8.14.9 11.34 3.82"/><path d="M16.85 11.25a10 10 0 0 1 2.22 1.68"/><path d="M5 13a10 10 0 0 1 5.24-2.76"/><line x1="12" y1="20" x2="12.01" y2="20"/></svg>',
  };
  const chosenIcon = svgIcons[iconType] || svgIcons["wifi-off"];

  let bodyMarkup = "";
  if (offlineMode === "custom" && customHtml.trim().length > 0) {
    bodyMarkup = `<style>${customCss}</style>${customHtml}`;
  } else {
    const helpBtnHtml =
      helpTxt && helpUrl
        ? `<a href="${helpUrl}" target="_blank" rel="noopener noreferrer" style="display:inline-flex;align-items:center;gap:.4rem;padding:.6rem 1.2rem;border:1px solid rgba(255,255,255,0.1);border-radius:10px;background:rgba(255,255,255,0.04);color:#d4d4d8;text-decoration:none;font-size:.825rem;font-weight:500;cursor:pointer;transition:background .15s ease"><span>${helpTxt}</span></a>`
        : "";

    const diagHtml = showDiag
      ? `<div style="margin-bottom:1.5rem;width:100%;max-width:340px;border-radius:12px;border:1px solid rgba(255,255,255,0.08);background:rgba(10,10,12,0.7);backdrop-filter:blur(12px);padding:.85rem 1rem;text-align:left;font-size:.75rem;color:#a1a1aa;box-sizing:border-box">
          <div style="display:flex;justify-content:space-between;color:#e4e4e7;margin-bottom:.4rem;font-weight:500"><span>Network Inspector</span><span style="color:#fb7185;font-family:monospace;font-size:.7rem">ERR_NETWORK_DISCONNECTED</span></div>
          <div style="display:flex;justify-content:space-between;border-top:1px solid rgba(255,255,255,0.06);padding-top:.4rem;font-family:monospace;font-size:.7rem"><span>Auto-Reconnect:</span><span style="color:#d4d4d8">${autoRecon ? `Active (${interval / 1000}s)` : "Disabled"}</span></div>
        </div>`
      : "";

    const graphicHtml =
      graphicMode === "custom" && customGraphicUrl
        ? `<div style="width:72px;height:72px;border-radius:18px;background:rgba(255,255,255,0.06);border:1px solid rgba(255,255,255,0.15);display:flex;align-items:center;justify-content:center;box-shadow:0 8px 24px rgba(0,0,0,0.4);overflow:hidden;margin:0 auto 1.25rem"><img src="${customGraphicUrl}" alt="Offline Mascot" style="width:100%;height:100%;object-fit:cover" /></div>`
        : `<div style="margin:0 auto 1.25rem;width:56px;height:56px;border-radius:16px;background:rgba(255,255,255,0.05);border:1px solid rgba(255,255,255,0.1);display:flex;align-items:center;justify-content:center;box-shadow:0 4px 16px rgba(0,0,0,0.3)">${chosenIcon}</div>`;

    let cardStyle =
      "max-width:420px;width:100%;background:rgba(24,24,27,0.75);border:1px solid rgba(255,255,255,0.08);border-radius:20px;padding:2.25rem;backdrop-filter:blur(20px);-webkit-backdrop-filter:blur(20px);box-shadow:0 24px 48px rgba(0,0,0,0.6);display:flex;flex-direction:column;align-items:center;box-sizing:border-box";
    if (layout === "minimal") {
      cardStyle =
        "max-width:380px;width:100%;background:transparent;border:none;box-shadow:none;padding:1.5rem;display:flex;flex-direction:column;align-items:center;box-sizing:border-box";
    } else if (layout === "hero") {
      cardStyle =
        "max-width:480px;width:100%;background:rgba(24,24,27,0.85);border:1px solid rgba(255,255,255,0.08);border-radius:24px;padding:2.5rem 2rem;box-shadow:0 24px 48px rgba(0,0,0,0.6);display:flex;flex-direction:column;align-items:center;box-sizing:border-box;position:relative";
    }

    bodyMarkup = `
      <div style="${cardStyle}">
        ${layout === "hero" ? '<div style="position:absolute;top:1rem;left:50%;transform:translateX(-50%);width:140px;height:140px;background:rgba(255,255,255,0.04);filter:blur(50px);border-radius:50%;pointer-events:none"></div>' : ""}
        ${graphicHtml}
        <h2 style="margin:0 0 .5rem;font-size:1.25rem;font-weight:600;letter-spacing:-0.02em;color:#fff">${title}</h2>
        <p style="margin:0 0 1.5rem;font-size:.85rem;color:#a1a1aa;max-width:360px;line-height:1.5">${msg}</p>
        ${diagHtml}
        <div style="display:flex;flex-wrap:wrap;align-items:center;justify-content:center;gap:.65rem">
          <button id="velora-retry-btn" style="display:inline-flex;align-items:center;gap:.5rem;padding:.6rem 1.4rem;border:none;border-radius:10px;background:#ffffff;color:#09090b;font-size:.85rem;font-weight:600;cursor:pointer;box-shadow:0 1px 3px rgba(0,0,0,0.2);transition:background .15s ease,transform .1s ease">
            <svg id="velora-retry-icon" style="width:14px;height:14px" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/><path d="M3 3v5h5"/><path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16"/><path d="M16 21h5v-5"/></svg>
            <span id="velora-retry-text">${retry}</span>
          </button>
          ${helpBtnHtml}
        </div>
      </div>
    `;
  }

  return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>${title} — ${appName}</title>
  <style>
    *, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }
    html, body {
      width: 100%;
      height: 100%;
      overflow: hidden;
      background: ${bg};
      color: #fff;
      font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
      user-select: none;
      -webkit-user-select: none;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      padding: 1.5rem;
    }
    @keyframes velora-spin-retry{to{transform:rotate(360deg)}}
  </style>
</head>
<body>
  ${bodyMarkup}
  <script>
    (function(){
      var targetUrl = ${JSON.stringify(websiteUrl)};
      var autoRecon = ${autoRecon};
      var interval = ${interval};
      var btn = document.getElementById('velora-retry-btn');
      var icon = document.getElementById('velora-retry-icon');
      var text = document.getElementById('velora-retry-text');
      var testing = false;

      function testAndReload(){
        if (testing) return;
        testing = true;
        if (btn) btn.disabled = true;
        if (icon) icon.style.animation = 'velora-spin-retry 0.8s linear infinite';
        if (text) text.textContent = 'Connecting...';

        if (!navigator.onLine) {
          setTimeout(function(){
            testing = false;
            if (btn) btn.disabled = false;
            if (icon) icon.style.animation = '';
            if (text) text.textContent = ${JSON.stringify(retry)};
          }, 800);
          return;
        }

        window.location.replace(targetUrl);
      }

      if (btn) {
        btn.addEventListener('click', testAndReload);
      }

      var retryTargets = document.querySelectorAll('[data-retry], .retry-btn');
      retryTargets.forEach(function(b){
        b.addEventListener('click', testAndReload);
      });

      if (autoRecon) {
        setInterval(function(){
          if (navigator.onLine) {
            testAndReload();
          }
        }, interval);
      }

      window.addEventListener('online', function(){
        testAndReload();
      });
    })();
  </script>
</body>
</html>`;
}

async function main() {
  const base64Input = process.env.CONFIG_JSON_BASE64 || process.argv[2] || "";
  const platform = process.env.BUILD_PLATFORM || process.argv[3] || "windows";
  const callbackUrl = process.env.CALLBACK_URL || process.argv[4] || "";
  const projectIdArg = process.env.PROJECT_ID || process.argv[5] || "";

  let rawJson = "{}";
  if (base64Input) {
    try {
      rawJson = Buffer.from(base64Input, "base64").toString("utf8");
    } catch {
      rawJson = base64Input;
    }
  } else {
    const existingConfigPath = path.join(srcTauriDir, "velora-config.json");
    if (fs.existsSync(existingConfigPath)) {
      rawJson = fs.readFileSync(existingConfigPath, "utf8");
    }
  }

  let payload = {};
  try {
    payload = JSON.parse(rawJson);
  } catch (err) {
    console.error("[RenderConfig] Failed to parse config JSON:", err);
  }

  const appName =
    (
      payload.appName ||
      payload.appConfig?.appName ||
      payload.name ||
      "Velora App"
    )
      .replace(/["\\]/g, "")
      .trim() || "Velora App";
  const version = (
    payload.version ||
    payload.appConfig?.version ||
    "1.0.0"
  ).trim();
  const rawBundleId = (
    payload.bundleId ||
    payload.appConfig?.bundleId ||
    ""
  ).trim();
  const safeSlug =
    (payload.slug || payload.appConfig?.slug || appName)
      .toLowerCase()
      .replace(/[^a-z0-9]/g, "") || "app";
  const bundleId =
    rawBundleId && rawBundleId !== "com.velora.application"
      ? rawBundleId
      : `com.velora.${safeSlug}`;

  let websiteUrl =
    payload.websiteUrl ||
    payload.url ||
    payload.appConfig?.websiteUrl ||
    payload.appConfig?.url ||
    "";
  if (!websiteUrl && process.env.WEBSITE_URL) {
    websiteUrl = process.env.WEBSITE_URL;
  }
  if (websiteUrl) {
    websiteUrl = websiteUrl.trim();
    if (
      !websiteUrl.startsWith("http://") &&
      !websiteUrl.startsWith("https://")
    ) {
      websiteUrl = `https://${websiteUrl}`;
    }
  } else {
    console.error(
      "[RenderConfig] CRITICAL WARNING: No websiteUrl found in payload or environment! Defaulting to https://veloradev.site",
    );
    websiteUrl = "https://veloradev.site";
  }

  const projectId =
    payload.projectId ||
    payload.project_id ||
    payload.appConfig?.projectId ||
    projectIdArg;

  let serverUrl = payload.serverUrl || payload.appConfig?.serverUrl || "";
  if (!serverUrl && callbackUrl) {
    try {
      serverUrl = new URL(callbackUrl).origin;
    } catch {}
  }
  if (!serverUrl && process.env.PUBLIC_APP_URL) {
    serverUrl = process.env.PUBLIC_APP_URL;
  }

  console.log(
    `[RenderConfig] Configuring project '${appName}' v${version} (${bundleId}) for ${platform}...`,
  );
  console.log(`[RenderConfig] Target URL: ${websiteUrl}`);

  let rawAllowed =
    payload.navigation?.allowedDomains ||
    payload.allowedDomains ||
    payload.appConfig?.navigation?.allowedDomains ||
    [];
  if (typeof rawAllowed === "string") {
    try {
      rawAllowed = JSON.parse(rawAllowed);
    } catch {
      rawAllowed = rawAllowed
        .split(",")
        .map((s) => s.trim())
        .filter(Boolean);
    }
  }
  const allowedDomains = Array.isArray(rawAllowed) ? [...rawAllowed] : [];
  try {
    const targetHost = new URL(websiteUrl).hostname;
    if (targetHost && !allowedDomains.includes(targetHost)) {
      allowedDomains.push(targetHost);
    }
  } catch {}

  const veloraConfig = {
    websiteUrl,
    appName,
    allowedDomains,
    userAgent:
      payload.navigation?.userAgent ||
      payload.userAgent ||
      payload.appConfig?.navigation?.userAgent ||
      null,
    enableTray: Boolean(
      payload.features?.enableTray ?? payload.enableTray ?? false,
    ),
    enableOfflineFallback: Boolean(
      payload.features?.enableOfflineFallback ??
      payload.enableOfflineFallback ??
      true,
    ),
    enableSingleInstance: Boolean(
      payload.features?.enableSingleInstance ??
      payload.enableSingleInstance ??
      false,
    ),
    enableDevtools: Boolean(
      payload.features?.enableDevtools ?? payload.enableDevtools ?? false,
    ),
    deepLinkScheme:
      payload.features?.deepLinkScheme || payload.deepLinkScheme || "",
    enableUnreadBadge: Boolean(
      payload.features?.enableUnreadBadge ?? payload.enableUnreadBadge ?? false,
    ),
    sandboxExternalLinks: Boolean(
      payload.features?.sandboxExternalLinks ??
      payload.sandboxExternalLinks ??
      true,
    ),
    injectionTiming:
      payload.injection?.injectionTiming ||
      payload.injectionTiming ||
      "document_start",
    trayMenuConfig:
      payload.features?.trayMenuConfig || payload.trayMenuConfig || null,
    customFont: {
      url: payload.injection?.customFontUrl || payload.customFontUrl || "",
      family:
        payload.injection?.customFontFamily || payload.customFontFamily || "",
    },
    version,
    splashScreen: {
      enabled: Boolean(
        payload.splashScreen?.enabled ??
        payload.splashScreenEnabled ??
        payload.features?.splashScreenEnabled ??
        false,
      ),
      durationMs: Number(
        payload.splashScreen?.durationMs ??
          payload.splashScreenDurationMs ??
          2000,
      ),
      bgColor:
        payload.splashScreen?.bgColor ||
        payload.splashScreenBgColor ||
        "#0d0e12",
      bgType:
        payload.splashScreen?.bgType ||
        payload.splashBgType ||
        payload.offlineConfig?.splash?.bgType ||
        "solid",
      gradientPreset:
        payload.splashScreen?.gradientPreset ||
        payload.splashGradientPreset ||
        payload.offlineConfig?.splash?.gradientPreset ||
        "obsidian-violet",
      image: payload.splashScreen?.image || payload.splashScreenImage || "",
      logoSize: Number(
        payload.splashScreen?.logoSize ||
          payload.splashLogoSize ||
          payload.offlineConfig?.splash?.logoSize ||
          72,
      ),
      entranceAnimation:
        payload.splashScreen?.entranceAnimation ||
        payload.splashEntranceAnimation ||
        payload.offlineConfig?.splash?.entranceAnimation ||
        "fade",
      title: payload.splashScreen?.title || payload.appName || appName,
      subtext:
        payload.splashScreen?.subtext !== undefined &&
        payload.splashScreen?.subtext !== null
          ? payload.splashScreen.subtext
          : payload.splashScreen?.loadingText !== undefined &&
              payload.splashScreen?.loadingText !== null
            ? payload.splashScreen.loadingText
            : payload.splashText !== undefined && payload.splashText !== null
              ? payload.splashText
              : "",
      subtextBadge:
        payload.splashScreen?.subtextBadge !== undefined &&
        payload.splashScreen?.subtextBadge !== null
          ? payload.splashScreen.subtextBadge
          : payload.splashSubtext !== undefined &&
              payload.splashSubtext !== null
            ? payload.splashSubtext
            : "",
      spinnerStyle:
        payload.splashScreen?.spinnerStyle ||
        payload.splashSpinnerStyle ||
        "orbit",
      mode:
        payload.splashScreen?.mode ||
        payload.splashMode ||
        payload.offlineConfig?.splash?.mode ||
        "visual",
      customHtml:
        payload.splashScreen?.customHtml ||
        payload.splashCustomHtml ||
        payload.offlineConfig?.splash?.customHtml ||
        "",
      customCss:
        payload.splashScreen?.customCss ||
        payload.splashCustomCss ||
        payload.offlineConfig?.splash?.customCss ||
        "",
      footerText:
        payload.splashScreen?.footerText ||
        payload.splashFooterText ||
        payload.offlineConfig?.splash?.footerText ||
        "",
      versionBadge:
        payload.splashScreen?.versionBadge ||
        payload.splashVersionBadge ||
        payload.offlineConfig?.splash?.versionBadge ||
        "",
      showLogoContainer: Boolean(
        payload.splashScreen?.showLogoContainer ??
        payload.splashShowLogoContainer ??
        payload.offlineConfig?.splash?.showLogoContainer ??
        true,
      ),
      iconDataUrl: "",
    },
    offlineConfig: {
      title: payload.offlineConfig?.title || "No Internet Connection",
      message:
        payload.offlineConfig?.message ||
        "Please check your internet connection and try again.",
      retryText: payload.offlineConfig?.retryText || "Try Again",
      themeBg: payload.offlineConfig?.themeBg || "#090a0c",
      accentColor: payload.offlineConfig?.accentColor || "amber",
      icon: payload.offlineConfig?.icon || "wifi-off",
      layout: payload.offlineConfig?.layout || "glass",
      graphicMode: payload.offlineConfig?.graphicMode || "icon",
      customGraphicUrl: payload.offlineConfig?.customGraphicUrl || "",
      mode: payload.offlineConfig?.mode || "visual",
      customHtml: payload.offlineConfig?.customHtml || "",
      customCss: payload.offlineConfig?.customCss || "",
      autoReconnect: Boolean(payload.offlineConfig?.autoReconnect ?? true),
      reconnectInterval: Number(payload.offlineConfig?.reconnectInterval || 10),
      helpText: payload.offlineConfig?.helpText || "",
      helpUrl: payload.offlineConfig?.helpUrl || "",
      showDiagnostics: Boolean(payload.offlineConfig?.showDiagnostics ?? false),
      showNetworkStatusBanner: Boolean(
        payload.offlineConfig?.showNetworkStatusBanner ??
        payload.showNetworkStatusBanner ??
        true,
      ),
    },
    precacheAssets:
      payload.precacheAssets || payload.offlineConfig?.precacheAssets || [],
    showNetworkStatusBanner: Boolean(
      payload.showNetworkStatusBanner ??
      payload.offlineConfig?.showNetworkStatusBanner ??
      true,
    ),
    window: {
      width: payload.window?.width || 1280,
      height: payload.window?.height || 800,
      minWidth: payload.window?.minWidth ?? null,
      minHeight: payload.window?.minHeight ?? null,
      resizable: payload.window?.resizable ?? true,
      fullscreen: payload.window?.fullscreen ?? false,
      decorations: payload.window?.decorations ?? true,
      alwaysOnTop: payload.window?.alwaysOnTop ?? false,
      titlebarStyle: payload.window?.titleBarStyle || "standard",
      rememberWindowState: Boolean(
        payload.window?.rememberWindowState ??
        payload.rememberWindowState ??
        true,
      ),
      showMinimizeButton: Boolean(
        payload.window?.showMinimizeButton ??
        payload.showMinimizeButton ??
        true,
      ),
      showMaximizeButton: Boolean(
        payload.window?.showMaximizeButton ??
        payload.showMaximizeButton ??
        true,
      ),
      showCloseButton: Boolean(
        payload.window?.showCloseButton ?? payload.showCloseButton ?? true,
      ),
    },
    metadata: {
      copyright: payload.copyright || payload.metadata?.copyright || "",
      companyName: payload.companyName || payload.metadata?.companyName || "",
    },
  };

  const veloraConfigPath = path.join(srcTauriDir, "velora-config.json");
  fs.writeFileSync(
    veloraConfigPath,
    JSON.stringify(veloraConfig, null, 2),
    "utf8",
  );
  console.log(`[RenderConfig] Wrote ${veloraConfigPath}`);

  const cssPath = path.join(srcTauriDir, "injection.css");
  const jsPath = path.join(srcTauriDir, "injection.js");
  let customCss =
    payload.injection?.customCss ||
    payload.customCss ||
    "/* No custom CSS injected */";
  const customFontUrl =
    payload.injection?.customFontUrl || payload.customFontUrl;
  const customFontFamily =
    payload.injection?.customFontFamily || payload.customFontFamily;
  if (customFontUrl && customFontFamily) {
    const fontFace = `@font-face {\n  font-family: '${customFontFamily}';\n  src: url('${customFontUrl}');\n  font-display: swap;\n}\nbody, button, input, select, textarea {\n  font-family: '${customFontFamily}', -apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif !important;\n}\n`;
    customCss = fontFace + "\n" + customCss;
  }
  const customJs =
    payload.injection?.customJs ||
    payload.customJs ||
    "// No custom JS injected";
  fs.writeFileSync(cssPath, customCss, "utf8");
  fs.writeFileSync(jsPath, customJs, "utf8");
  console.log(
    `[RenderConfig] Wrote injection scripts (CSS length: ${customCss.length}, JS length: ${customJs.length})`,
  );

  const tauriConfPath = path.join(srcTauriDir, "tauri.conf.json");
  if (fs.existsSync(tauriConfPath)) {
    try {
      const conf = JSON.parse(fs.readFileSync(tauriConfPath, "utf8"));
      conf.productName = appName;
      conf.version = version;
      conf.identifier = bundleId;

      const safeBinaryName = appName.replace(/[^a-zA-Z0-9_-]/g, "") || "app";
      conf.mainBinaryName = safeBinaryName;

      if (!conf.app) conf.app = {};
      conf.app.windows = [];

      if (!conf.bundle) conf.bundle = {};
      if (payload.copyright || payload.metadata?.copyright) {
        conf.bundle.copyright =
          payload.copyright || payload.metadata?.copyright;
      }
      if (payload.companyName || payload.metadata?.companyName) {
        conf.bundle.publisher =
          payload.companyName || payload.metadata?.companyName;
      }
      // Deep link scheme is stored in velora-config.json.
      // In Tauri v2, bundle.deepLink is rejected by schema validation.
      if (!conf.bundle.windows) conf.bundle.windows = {};
      if (!conf.bundle.windows.nsis) conf.bundle.windows.nsis = {};
      conf.bundle.windows.nsis.installMode = "currentUser";
      conf.bundle.windows.nsis.installerIcon = "icons/icon.ico";
      conf.bundle.windows.nsis.uninstallerIcon = "icons/icon.ico";

      const devPerms = payload.devicePermissions || payload.appConfig?.devicePermissions || {};
      if (devPerms.camera || devPerms.microphone || devPerms.geolocation) {
        if (!conf.bundle.macOS) conf.bundle.macOS = {};
        if (!conf.bundle.macOS.infoPlist) conf.bundle.macOS.infoPlist = {};
        if (devPerms.camera) {
          conf.bundle.macOS.infoPlist.NSCameraUsageDescription = `${appName} requires camera access.`;
        }
        if (devPerms.microphone) {
          conf.bundle.macOS.infoPlist.NSMicrophoneUsageDescription = `${appName} requires microphone access.`;
        }
        if (devPerms.geolocation) {
          conf.bundle.macOS.infoPlist.NSLocationWhenInUseUsageDescription = `${appName} requires location access.`;
        }
      }

      fs.writeFileSync(tauriConfPath, JSON.stringify(conf, null, 2), "utf8");
      console.log(
        `[RenderConfig] Updated tauri.conf.json productName: "${appName}", mainBinaryName: "${safeBinaryName}", version: "${version}", identifier: "${bundleId}"`,
      );
    } catch (err) {
      console.error("[RenderConfig] Failed to update tauri.conf.json:", err);
    }
  }

  if (projectId && serverUrl) {
    console.log(
      `[RenderConfig] Attempting to download custom icon bundle for project ${projectId} from ${serverUrl}...`,
    );
    const iconsDir = path.join(srcTauriDir, "icons");
    if (!fs.existsSync(iconsDir)) {
      fs.mkdirSync(iconsDir, { recursive: true });
    }

    const iconFiles = [
      "icon.ico",
      "32x32.png",
      "128x128.png",
      "128x128@2x.png",
      "icon-512.png",
      "master.png",
    ];

    let downloadedCount = 0;
    for (const file of iconFiles) {
      const queryFile = file === "128x128@2x.png" ? "128x128.png" : file;
      const iconUrl = `${serverUrl}/api/projects/${projectId}/icon?file=${encodeURIComponent(queryFile)}`;
      try {
        const res = await fetch(iconUrl, {
          headers: { "User-Agent": "Velora-Compiler/1.0" },
        });
        if (res.ok) {
          const arrayBuffer = await res.arrayBuffer();
          const buffer = Buffer.from(arrayBuffer);
          if (buffer.length > 100) {
            fs.writeFileSync(path.join(iconsDir, file), buffer);
            downloadedCount++;
            console.log(
              `[RenderConfig] Downloaded icon ${file} (${buffer.length} bytes)`,
            );
          }
        } else {
          console.log(
            `[RenderConfig] Icon ${file} not available at ${iconUrl} (HTTP ${res.status})`,
          );
        }
      } catch (err) {
        console.warn(
          `[RenderConfig] Failed to fetch icon ${file}: ${err.message}`,
        );
      }
    }

    if (downloadedCount > 0) {
      console.log(
        `[RenderConfig] Successfully applied ${downloadedCount} custom icons to project.`,
      );
    } else {
      console.log(
        "[RenderConfig] Custom icons not found or unreachable. Using default template icons.",
      );
    }
  }

  // Guarantee every icon in src-tauri/icons/ is 32-bit RGBA for Tauri compile-time macros
  const iconsDir = path.join(srcTauriDir, "icons");
  if (fs.existsSync(iconsDir)) {
    try {
      const iconEntries = fs.readdirSync(iconsDir);
      for (const entry of iconEntries) {
        if (entry.toLowerCase().endsWith(".png")) {
          ensureRgbaPng(path.join(iconsDir, entry));
        }
      }
      const icon128 = path.join(iconsDir, "128x128.png");
      const icon128x2 = path.join(iconsDir, "128x128@2x.png");
      if (fs.existsSync(icon128) && !fs.existsSync(icon128x2)) {
        fs.copyFileSync(icon128, icon128x2);
      }
    } catch (err) {
      console.warn("[RenderConfig] Error checking icons for RGBA compliance:", err);
    }
  }

  for (const candidate of [
    "128x128.png",
    "32x32.png",
    "icon-512.png",
    "master.png",
  ]) {
    const p = path.join(iconsDir, candidate);
    if (fs.existsSync(p)) {
      try {
        const buf = fs.readFileSync(p);
        if (buf.length > 50) {
          veloraConfig.splashScreen.iconDataUrl = `data:image/png;base64,${buf.toString("base64")}`;
          fs.writeFileSync(
            veloraConfigPath,
            JSON.stringify(veloraConfig, null, 2),
            "utf8",
          );
          console.log(
            `[RenderConfig] Embedded icon (${candidate}, ${buf.length} bytes) as base64 into splashScreen.iconDataUrl`,
          );
          break;
        }
      } catch (err) {
        console.warn(
          "[RenderConfig] Failed to read icon for base64 splash:",
          err,
        );
      }
    }
  }

  try {
    const indexHtmlPath = path.join(templateRoot, "src", "index.html");
    const indexHtmlContent = renderIndexHtml(
      veloraConfig.splashScreen,
      websiteUrl,
      appName,
    );
    fs.writeFileSync(indexHtmlPath, indexHtmlContent, "utf8");
    console.log(
      `[RenderConfig] Successfully rendered native splash screen into ${indexHtmlPath}`,
    );
  } catch (err) {
    console.warn(
      "[RenderConfig] Failed to render index.html splash screen:",
      err,
    );
  }

  try {
    const offlineHtmlPath = path.join(templateRoot, "src", "offline.html");
    const offlineHtmlContent = renderOfflineHtml(
      veloraConfig.offlineConfig,
      websiteUrl,
      appName,
    );
    fs.writeFileSync(offlineHtmlPath, offlineHtmlContent, "utf8");
    console.log(
      `[RenderConfig] Successfully rendered custom offline fallback into ${offlineHtmlPath}`,
    );
  } catch (err) {
    console.warn("[RenderConfig] Failed to render offline.html fallback:", err);
  }

  if (platform === "android") {
    const androidGenDir = path.join(srcTauriDir, "gen", "android");

    function findAndroidMainDirs(baseDir) {
      const results = [];
      if (!fs.existsSync(baseDir)) return results;
      function scan(currentDir, depth = 0) {
        if (depth > 6) return;
        try {
          const entries = fs.readdirSync(currentDir, { withFileTypes: true });
          for (const entry of entries) {
            if (!entry.isDirectory()) continue;
            const fullPath = path.join(currentDir, entry.name);
            if (
              entry.name === "main" &&
              path.basename(path.dirname(fullPath)) === "src"
            ) {
              results.push(fullPath);
            } else if (entry.name !== "build" && entry.name !== ".gradle") {
              scan(fullPath, depth + 1);
            }
          }
        } catch {}
      }
      scan(baseDir);
      return results;
    }

    function findFilesRecursive(dir, filename) {
      let results = [];
      if (!fs.existsSync(dir)) return results;
      try {
        const entries = fs.readdirSync(dir, { withFileTypes: true });
        for (const entry of entries) {
          const fullPath = path.join(dir, entry.name);
          if (entry.isDirectory()) {
            results = results.concat(findFilesRecursive(fullPath, filename));
          } else if (
            entry.isFile() &&
            (entry.name === filename ||
              (filename instanceof RegExp && filename.test(entry.name)))
          ) {
            results.push(fullPath);
          }
        }
      } catch {}
      return results;
    }

    const androidMainDirs = findAndroidMainDirs(androidGenDir);

    if (androidMainDirs.length > 0) {
      for (const mainDir of androidMainDirs) {
        const androidResDir = path.join(mainDir, "res");
        const androidJavaDir = path.join(mainDir, "java");
        const manifestPath = path.join(mainDir, "AndroidManifest.xml");

        // 1. Update App Name in strings.xml
        const stringsXmlPath = path.join(
          androidResDir,
          "values",
          "strings.xml",
        );
        if (fs.existsSync(stringsXmlPath)) {
          try {
            let stringsContent = fs.readFileSync(stringsXmlPath, "utf8");
            const cleanAppName = appName
              .replace(/&/g, "&amp;")
              .replace(/</g, "&lt;")
              .replace(/>/g, "&gt;");
            stringsContent = stringsContent.replace(
              /<string name="app_name">.*?<\/string>/s,
              `<string name="app_name">${cleanAppName}</string>`,
            );
            fs.writeFileSync(stringsXmlPath, stringsContent, "utf8");
            console.log(
              `[RenderConfig] Updated Android strings.xml with app_name: "${appName}"`,
            );
          } catch (err) {
            console.warn("[RenderConfig] Failed to update strings.xml:", err);
          }
        }

        // 2. Inject Android Mipmap Icons
        const iconsDir = path.join(srcTauriDir, "icons");
        let fallbackIconBuf = null;
        for (const candidate of [
          "master.png",
          "icon-512.png",
          "128x128.png",
          "32x32.png",
        ]) {
          const p = path.join(iconsDir, candidate);
          if (fs.existsSync(p)) {
            fallbackIconBuf = fs.readFileSync(p);
            break;
          }
        }

        const mipmaps = ["mdpi", "hdpi", "xhdpi", "xxhdpi", "xxxhdpi"];
        for (const m of mipmaps) {
          const dir = path.join(androidResDir, `mipmap-${m}`);
          if (!fs.existsSync(dir)) fs.mkdirSync(dir, { recursive: true });

          let iconBuf = null;
          if (projectId && serverUrl) {
            const iconUrl = `${serverUrl}/api/projects/${projectId}/icon?file=mipmap-${m}/ic_launcher.png`;
            try {
              const res = await fetch(iconUrl, {
                headers: { "User-Agent": "Velora-Compiler/1.0" },
              });
              if (res.ok) {
                const b = Buffer.from(await res.arrayBuffer());
                if (b.length > 50) iconBuf = b;
              }
            } catch {}
          }

          if (!iconBuf) {
            iconBuf = fallbackIconBuf;
          }

          if (iconBuf) {
            const launcherPath = path.join(dir, "ic_launcher.png");
            const roundPath = path.join(dir, "ic_launcher_round.png");
            const foregroundPath = path.join(dir, "ic_launcher_foreground.png");
            fs.writeFileSync(launcherPath, iconBuf);
            fs.writeFileSync(roundPath, iconBuf);
            fs.writeFileSync(foregroundPath, iconBuf);
            ensureRgbaPng(launcherPath);
            ensureRgbaPng(roundPath);
            ensureRgbaPng(foregroundPath);
            console.log(
              `[RenderConfig] Injected Android icons into mipmap-${m} (launcher, round, foreground)`,
            );
          }
        }

        // 3. Harden AndroidManifest.xml
        if (fs.existsSync(manifestPath)) {
          try {
            let manifest = fs.readFileSync(manifestPath, "utf8");

            if (!manifest.includes("android:usesCleartextTraffic")) {
              manifest = manifest.replace(
                "<application",
                '<application android:usesCleartextTraffic="true" android:hardwareAccelerated="true"',
              );
            } else {
              manifest = manifest.replace(
                /android:usesCleartextTraffic="false"/g,
                'android:usesCleartextTraffic="true"',
              );
            }

            const devPerms =
              payload.devicePermissions ||
              payload.appConfig?.devicePermissions ||
              {};
            const permissions = [
              "android.permission.INTERNET",
              "android.permission.ACCESS_NETWORK_STATE",
            ];
            if (devPerms.storage !== false) {
              permissions.push(
                "android.permission.READ_EXTERNAL_STORAGE",
                "android.permission.WRITE_EXTERNAL_STORAGE",
              );
            }
            if (devPerms.notifications !== false) {
              permissions.push("android.permission.POST_NOTIFICATIONS");
            }
            if (devPerms.camera) {
              permissions.push("android.permission.CAMERA");
            }
            if (devPerms.microphone) {
              permissions.push("android.permission.RECORD_AUDIO");
              permissions.push("android.permission.MODIFY_AUDIO_SETTINGS");
            }
            if (devPerms.geolocation) {
              permissions.push(
                "android.permission.ACCESS_FINE_LOCATION",
                "android.permission.ACCESS_COARSE_LOCATION",
              );
            }

            for (const perm of permissions) {
              if (!manifest.includes(`android:name="${perm}"`)) {
                manifest = manifest.replace(
                  "</manifest>",
                  `    <uses-permission android:name="${perm}" />\n</manifest>`,
                );
              }
            }

            if (devPerms.camera && !manifest.includes('android.hardware.camera')) {
              manifest = manifest.replace(
                "</manifest>",
                `    <uses-feature android:name="android.hardware.camera" android:required="false" />\n</manifest>`,
              );
            }
            if (devPerms.microphone && !manifest.includes('android.hardware.microphone')) {
              manifest = manifest.replace(
                "</manifest>",
                `    <uses-feature android:name="android.hardware.microphone" android:required="false" />\n</manifest>`,
              );
            }
            if (devPerms.geolocation && !manifest.includes('android.hardware.location.gps')) {
              manifest = manifest.replace(
                "</manifest>",
                `    <uses-feature android:name="android.hardware.location.gps" android:required="false" />\n</manifest>`,
              );
            }

            if (!manifest.includes("android:windowSoftInputMode")) {
              manifest = manifest.replace(
                "<activity",
                '<activity android:windowSoftInputMode="adjustResize"',
              );
            }

            fs.writeFileSync(manifestPath, manifest, "utf8");
            console.log(
              "[RenderConfig] Hardened AndroidManifest.xml (cleartextTraffic, hardwareAccelerated, network/notification permissions, adjustResize)",
            );
          } catch (err) {
            console.warn(
              "[RenderConfig] Failed to patch AndroidManifest.xml:",
              err,
            );
          }
        }

        // 4. Configure themes.xml and styles.xml with fitsSystemWindows & edge-to-edge opt-out
        const xmlStyleFiles = [
          ...findFilesRecursive(androidResDir, "styles.xml"),
          ...findFilesRecursive(androidResDir, "themes.xml"),
        ];
        const barColor =
          veloraConfig.splashScreen?.bgColor ||
          veloraConfig.offlineConfig?.themeBg ||
          "#090a0c";

        function isColorLight(hexColor) {
          try {
            const hex = String(hexColor).replace("#", "");
            if (hex.length < 6) return false;
            const r = parseInt(hex.substring(0, 2), 16);
            const g = parseInt(hex.substring(2, 4), 16);
            const b = parseInt(hex.substring(4, 6), 16);
            const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255;
            return luminance > 0.6;
          } catch {
            return false;
          }
        }
        const isLightBar = isColorLight(barColor);

        for (const styleFile of xmlStyleFiles) {
          try {
            let xml = fs.readFileSync(styleFile, "utf8");
            let modified = false;

            if (!xml.includes("android:fitsSystemWindows")) {
              xml = xml.replace(/<style\s+name="([^"]+)"[^>]*>/g, (match) => {
                return `${match}\n        <item name="android:fitsSystemWindows">true</item>\n        <item name="android:windowOptOutEdgeToEdgeEnforcement">true</item>\n        <item name="android:windowDrawsSystemBarBackgrounds">true</item>\n        <item name="android:statusBarColor">${barColor}</item>\n        <item name="android:navigationBarColor">${barColor}</item>`;
              });
              modified = true;
            } else {
              xml = xml.replace(
                /<item name="android:fitsSystemWindows">false<\/item>/g,
                '<item name="android:fitsSystemWindows">true</item>',
              );
              modified = true;
            }

            if (!xml.includes("android:windowOptOutEdgeToEdgeEnforcement")) {
              xml = xml.replace(
                /<item name="android:fitsSystemWindows">true<\/item>/g,
                '<item name="android:fitsSystemWindows">true</item>\n        <item name="android:windowOptOutEdgeToEdgeEnforcement">true</item>',
              );
              modified = true;
            }

            if (!xml.includes("android:statusBarColor")) {
              xml = xml.replace(
                /<item name="android:fitsSystemWindows">true<\/item>/g,
                `<item name="android:fitsSystemWindows">true</item>\n        <item name="android:statusBarColor">${barColor}</item>\n        <item name="android:navigationBarColor">${barColor}</item>`,
              );
              modified = true;
            }

            if (xml.includes("android:windowTranslucentNavigation")) {
              xml = xml.replace(
                /<item name="android:windowTranslucentNavigation">true<\/item>/g,
                '<item name="android:windowTranslucentNavigation">false</item>',
              );
              modified = true;
            }
            if (xml.includes("android:windowTranslucentStatus")) {
              xml = xml.replace(
                /<item name="android:windowTranslucentStatus">true<\/item>/g,
                '<item name="android:windowTranslucentStatus">false</item>',
              );
              modified = true;
            }

            if (modified) {
              fs.writeFileSync(styleFile, xml, "utf8");
              console.log(
                `[RenderConfig] Configured ${path.relative(androidResDir, styleFile)} with fitsSystemWindows: true, edge-to-edge opt-out, and bar colors`,
              );
            }
          } catch (err) {
            console.warn(
              `[RenderConfig] Failed to patch style file ${styleFile}:`,
              err,
            );
          }
        }

        // 5. Patch MainActivity.kt to apply WindowInsets padding to root content view
        const mainActivityFiles = findFilesRecursive(
          androidJavaDir,
          "MainActivity.kt",
        );
        for (const mainActivityPath of mainActivityFiles) {
          try {
            let mainActivityContent = fs.readFileSync(mainActivityPath, "utf8");
            if (
              !mainActivityContent.includes("setOnApplyWindowInsetsListener")
            ) {
              const insetsSnippet = `
    try {
      androidx.core.view.WindowCompat.setDecorFitsSystemWindows(window, true)
      val insetsController = androidx.core.view.WindowCompat.getInsetsController(window, window.decorView)
      if (insetsController != null) {
        insetsController.isAppearanceLightStatusBars = ${isLightBar}
        insetsController.isAppearanceLightNavigationBars = ${isLightBar}
      }
      val rootView = findViewById<android.view.View>(android.R.id.content)
      if (rootView != null) {
        androidx.core.view.ViewCompat.setOnApplyWindowInsetsListener(rootView) { v, insets ->
          val systemBars = insets.getInsets(
            androidx.core.view.WindowInsetsCompat.Type.systemBars() or
            androidx.core.view.WindowInsetsCompat.Type.displayCutout()
          )
          v.setPadding(systemBars.left, systemBars.top, systemBars.right, systemBars.bottom)
          insets
        }
        rootView.requestApplyInsets()
      }
    } catch (e: Exception) {
      android.util.Log.w("Velora", "Failed to apply system insets: " + e.message)
    }
`;
              if (
                mainActivityContent.includes(
                  "super.onCreate(savedInstanceState)",
                )
              ) {
                mainActivityContent = mainActivityContent.replace(
                  "super.onCreate(savedInstanceState)",
                  `super.onCreate(savedInstanceState)\n${insetsSnippet}`,
                );
                fs.writeFileSync(mainActivityPath, mainActivityContent, "utf8");
                console.log(
                  `[RenderConfig] Patched MainActivity.kt with WindowInsets listener at ${path.relative(templateRoot, mainActivityPath)}`,
                );
              } else if (mainActivityContent.includes("class MainActivity")) {
                mainActivityContent = mainActivityContent.replace(
                  /class\s+MainActivity[^{]*\{/,
                  (match) =>
                    `${match}\n  override fun onCreate(savedInstanceState: android.os.Bundle?) {\n    super.onCreate(savedInstanceState)\n${insetsSnippet}\n  }`,
                );
                fs.writeFileSync(mainActivityPath, mainActivityContent, "utf8");
                console.log(
                  `[RenderConfig] Patched MainActivity.kt (injected onCreate) with WindowInsets listener at ${path.relative(templateRoot, mainActivityPath)}`,
                );
              }
            }
          } catch (err) {
            console.warn(
              "[RenderConfig] Failed to patch MainActivity.kt:",
              err,
            );
          }
        }
      }
    } else {
      console.log(
        "[RenderConfig] Android gen dir does not contain src/main yet; will be populated after tauri android init.",
      );
    }
  }
}

main().catch((err) => {
  console.error("[RenderConfig] Fatal error:", err);
  process.exit(1);
});
