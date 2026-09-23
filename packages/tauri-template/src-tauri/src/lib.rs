use serde::Deserialize;
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_opener::OpenerExt;
use url::Url;

const VELORA_CONFIG: &str = include_str!("../velora-config.json");
const INJECTION_CSS: &str = include_str!("../injection.css");
const INJECTION_JS: &str = include_str!("../injection.js");

fn default_true() -> bool {
    true
}

fn default_window_width() -> f64 {
    1280.0
}

fn default_window_height() -> f64 {
    800.0
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct WindowConfig {
    #[serde(default = "default_window_width")]
    width: f64,
    #[serde(default = "default_window_height")]
    height: f64,
    min_width: Option<f64>,
    min_height: Option<f64>,
    #[serde(default = "default_true")]
    resizable: bool,
    #[serde(default)]
    fullscreen: bool,
    #[serde(default = "default_true")]
    decorations: bool,
    #[serde(default)]
    always_on_top: bool,
    titlebar_style: Option<String>,
    #[serde(default = "default_true")]
    remember_window_state: bool,
    #[serde(default = "default_true")]
    show_minimize_button: bool,
    #[serde(default = "default_true")]
    show_maximize_button: bool,
    #[serde(default = "default_true")]
    show_close_button: bool,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct SplashScreenConfig {
    #[serde(default)]
    enabled: bool,
    #[serde(default = "default_splash_duration")]
    duration_ms: u64,
    #[serde(default = "default_splash_bg")]
    bg_color: String,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    subtext: Option<String>,
    #[serde(default)]
    spinner_style: Option<String>,
    #[serde(default)]
    image: Option<String>,
    #[serde(default)]
    icon_data_url: Option<String>,
    #[serde(default)]
    bg_type: Option<String>,
    #[serde(default)]
    gradient_preset: Option<String>,
    #[serde(default)]
    logo_size: Option<u32>,
    #[serde(default)]
    entrance_animation: Option<String>,
    #[serde(default)]
    mode: Option<String>,
    #[serde(default)]
    custom_html: Option<String>,
    #[serde(default)]
    custom_css: Option<String>,
    #[serde(default)]
    footer_text: Option<String>,
    #[serde(default)]
    version_badge: Option<String>,
    #[serde(default = "default_true")]
    show_logo_container: bool,
}

fn default_splash_duration() -> u64 {
    2000
}

fn default_splash_bg() -> String {
    "#0d0e12".to_string()
}

fn default_offline_title() -> String {
    "No Internet Connection".to_string()
}

fn default_offline_message() -> String {
    "Please check your internet connection and try again.".to_string()
}

fn default_offline_retry() -> String {
    "Try Again".to_string()
}

fn default_offline_bg() -> String {
    "#090a0c".to_string()
}

fn default_offline_accent() -> String {
    "amber".to_string()
}

fn default_offline_icon() -> String {
    "wifi-off".to_string()
}

fn default_reconnect_interval() -> u64 {
    10
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct OfflineConfig {
    #[serde(default = "default_offline_title")]
    title: String,
    #[serde(default = "default_offline_message")]
    message: String,
    #[serde(default = "default_offline_retry")]
    retry_text: String,
    #[serde(default = "default_offline_bg")]
    theme_bg: String,
    #[serde(default = "default_offline_accent")]
    accent_color: String,
    #[serde(default = "default_offline_icon")]
    icon: String,
    #[serde(default = "default_true")]
    auto_reconnect: bool,
    #[serde(default = "default_reconnect_interval")]
    reconnect_interval: u64,
    #[serde(default)]
    help_text: Option<String>,
    #[serde(default)]
    help_url: Option<String>,
    #[serde(default)]
    show_diagnostics: bool,
    #[serde(default = "default_true")]
    show_network_status_banner: bool,
    #[serde(default)]
    layout: Option<String>,
    #[serde(default)]
    graphic_mode: Option<String>,
    #[serde(default)]
    custom_graphic_url: Option<String>,
    #[serde(default)]
    mode: Option<String>,
    #[serde(default)]
    custom_html: Option<String>,
    #[serde(default)]
    custom_css: Option<String>,
}

fn default_version() -> String {
    "1.0.0".to_string()
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct MetadataConfig {
    #[serde(default)]
    copyright: Option<String>,
    #[serde(default)]
    company_name: Option<String>,
}

#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct TrayMenuItemConfig {
    label: String,
    action: String,
    url: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct VeloraConfig {
    website_url: String,
    app_name: String,
    #[serde(default = "default_version")]
    version: String,
    #[serde(default)]
    allowed_domains: Vec<String>,
    #[serde(default)]
    user_agent: Option<String>,
    #[serde(default)]
    enable_tray: bool,
    #[serde(default = "default_true")]
    enable_offline_fallback: bool,
    #[serde(default)]
    enable_single_instance: bool,
    #[serde(default)]
    enable_devtools: bool,
    #[serde(default = "default_true")]
    sandbox_external_links: bool,
    #[serde(default)]
    injection_timing: Option<String>,
    #[serde(default)]
    window: WindowConfig,
    #[serde(default)]
    splash_screen: SplashScreenConfig,
    #[serde(default)]
    offline_config: OfflineConfig,
    #[serde(default)]
    metadata: MetadataConfig,
    #[serde(default)]
    tray_menu_config: Option<Vec<TrayMenuItemConfig>>,
    #[serde(default)]
    precache_assets: Vec<String>,
    #[serde(default = "default_true")]
    show_network_status_banner: bool,
}

fn is_domain_allowed(nav_url: &Url, main_host: &str, allowed: &[String]) -> bool {
    let scheme = nav_url.scheme();

    if scheme == "about" || scheme == "data" || scheme == "blob" || scheme == "tauri" {
        return true;
    }

    let nav_host = match nav_url.host_str() {
        Some(h) => h,
        None => return false,
    };

    if nav_host == "localhost" || nav_host == "tauri.localhost" {
        return true;
    }

    if scheme != "http" && scheme != "https" {
        return false;
    }

    let stripped_main = main_host.strip_prefix("www.").unwrap_or(main_host);
    let stripped_nav = nav_host.strip_prefix("www.").unwrap_or(nav_host);

    if stripped_nav == stripped_main || stripped_nav.ends_with(&format!(".{}", stripped_main)) {
        return true;
    }

    for domain in allowed {
        let d = domain.trim();
        if d.is_empty() {
            continue;
        }
        let stripped_d = d.strip_prefix("*.").unwrap_or(d).strip_prefix("www.").unwrap_or(d);
        if stripped_nav == stripped_d || stripped_nav.ends_with(&format!(".{}", stripped_d)) {
            return true;
        }
    }

    false
}

fn css_injection_script(css: &str) -> String {
    if css.trim().is_empty() {
        return String::new();
    }
    let escaped = serde_json::to_string(css).unwrap_or_default();
    format!(
        r#"(function(){{var s=document.createElement("style");s.textContent={escaped};(document.head||document.documentElement).appendChild(s)}})()"#,
    )
}

fn offline_detection_script(offline: &OfflineConfig) -> String {
    let title_escaped = serde_json::to_string(&offline.title).unwrap_or_else(|_| "\"No Internet Connection\"".to_string());
    let message_escaped = serde_json::to_string(&offline.message).unwrap_or_else(|_| "\"Please check your internet connection and try again.\"".to_string());
    let retry_escaped = serde_json::to_string(&offline.retry_text).unwrap_or_else(|_| "\"Try Again\"".to_string());
    let bg_color = if offline.theme_bg.trim().is_empty() { "#090a0c" } else { &offline.theme_bg };
    let bg_escaped = serde_json::to_string(bg_color).unwrap_or_else(|_| "\"#090a0c\"".to_string());
    let accent_escaped = serde_json::to_string(&offline.accent_color).unwrap_or_else(|_| "\"amber\"".to_string());
    let icon_escaped = serde_json::to_string(&offline.icon).unwrap_or_else(|_| "\"wifi-off\"".to_string());
    let auto_reconnect = offline.auto_reconnect;
    let interval_sec = if offline.reconnect_interval < 2 { 5 } else { offline.reconnect_interval };
    let help_text_escaped = serde_json::to_string(offline.help_text.as_deref().unwrap_or("")).unwrap_or_else(|_| "\"\"".to_string());
    let help_url_escaped = serde_json::to_string(offline.help_url.as_deref().unwrap_or("")).unwrap_or_else(|_| "\"\"".to_string());
    let show_diagnostics = offline.show_diagnostics;
    let show_network_status_banner = offline.show_network_status_banner;
    let mode_escaped = serde_json::to_string(offline.mode.as_deref().unwrap_or("visual")).unwrap_or_else(|_| "\"visual\"".to_string());
    let custom_html_escaped = serde_json::to_string(offline.custom_html.as_deref().unwrap_or("")).unwrap_or_else(|_| "\"\"".to_string());
    let custom_css_escaped = serde_json::to_string(offline.custom_css.as_deref().unwrap_or("")).unwrap_or_else(|_| "\"\"".to_string());
    let layout_escaped = serde_json::to_string(offline.layout.as_deref().unwrap_or("glass")).unwrap_or_else(|_| "\"glass\"".to_string());
    let graphic_mode_escaped = serde_json::to_string(offline.graphic_mode.as_deref().unwrap_or("icon")).unwrap_or_else(|_| "\"icon\"".to_string());
    let custom_graphic_url_escaped = serde_json::to_string(offline.custom_graphic_url.as_deref().unwrap_or("")).unwrap_or_else(|_| "\"\"".to_string());

    format!(
        r##"(function(){{
  var o = null;
  var timer = null;
  var bannerEl = null;
  var showBanner = {show_network_status_banner};
  var title = {title_escaped};
  var msg = {message_escaped};
  var retry = {retry_escaped};
  var bg = {bg_escaped};
  var accent = {accent_escaped};
  var iconType = {icon_escaped};
  var autoRecon = {auto_reconnect};
  var interval = {interval_sec} * 1000;
  var helpTxt = {help_text_escaped};
  var helpUrl = {help_url_escaped};
  var showDiag = {show_diagnostics};
  var offlineMode = {mode_escaped};
  var customHtml = {custom_html_escaped};
  var customCss = {custom_css_escaped};
  var layout = {layout_escaped};
  var graphicMode = {graphic_mode_escaped};
  var customGraphicUrl = {custom_graphic_url_escaped};

  function showBannerOnly() {{
    if(bannerEl) return;
    bannerEl = document.createElement("div");
    bannerEl.id = "velora-network-status-banner";
    bannerEl.style.cssText = "position:fixed;top:12px;left:50%;transform:translateX(-50%);z-index:2147483646;display:flex;align-items:center;gap:8px;padding:6px 14px;border-radius:9999px;background:rgba(24,24,27,0.92);backdrop-filter:blur(12px);-webkit-backdrop-filter:blur(12px);border:1px solid rgba(245,158,11,0.35);color:#fbbf24;font-family:system-ui,-apple-system,sans-serif;font-size:12px;font-weight:500;box-shadow:0 10px 25px rgba(0,0,0,0.5);pointer-events:none;transition:opacity 0.3s ease;";
    bannerEl.innerHTML = '<span style="display:inline-block;width:7px;height:7px;border-radius:50%;background:#f59e0b;box-shadow:0 0 8px #f59e0b"></span><span>Offline — Viewing cached content</span>';
    (document.body || document.documentElement).appendChild(bannerEl);
  }}

  function hideBannerOnly() {{
    if(bannerEl) {{ bannerEl.remove(); bannerEl = null; }}
  }}

  var colors = {{
    emerald: {{ emblemBg: 'rgba(16,185,129,0.12)', border: 'rgba(16,185,129,0.25)', text: '#34d399', dot: '#10b981' }},
    cyan: {{ emblemBg: 'rgba(6,182,212,0.12)', border: 'rgba(6,182,212,0.25)', text: '#22d3ee', dot: '#06b6d4' }},
    rose: {{ emblemBg: 'rgba(244,63,94,0.12)', border: 'rgba(244,63,94,0.25)', text: '#fb7185', dot: '#f43f5e' }},
    violet: {{ emblemBg: 'rgba(139,92,246,0.12)', border: 'rgba(139,92,246,0.25)', text: '#a78bfa', dot: '#8b5cf6' }},
    amber: {{ emblemBg: 'rgba(245,158,11,0.12)', border: 'rgba(245,158,11,0.25)', text: '#fbbf24', dot: '#f59e0b' }}
  }};
  var c = colors[accent] || colors.amber;

  var svgIcons = {{
    'cloud-off': '<svg style="width:28px;height:28px;color:#e4e4e7" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m2 2 20 20"/><path d="M5.782 5.782A7 7 0 0 0 9 19h8.5a4.5 4.5 0 0 0 1.307-.193"/><path d="M21.532 16.5A4.5 4.5 0 0 0 17.5 10h-1.79A7.008 7.008 0 0 0 10 5.07"/></svg>',
    'alert': '<svg style="width:28px;height:28px;color:#e4e4e7" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>',
    'shield': '<svg style="width:28px;height:28px;color:#e4e4e7" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>',
    'plug': '<svg style="width:28px;height:28px;color:#e4e4e7" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22v-5"/><path d="M9 8V2"/><path d="M15 8V2"/><path d="M18 8v5a4 4 0 0 1-4 4h-4a4 4 0 0 1-4-4V8Z"/></svg>',
    'radar': '<svg style="width:28px;height:28px;color:#e4e4e7" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M19.07 4.93A10 10 0 0 0 4.93 19.07"/><path d="M16.24 7.76A6 6 0 0 0 7.76 16.24"/><circle cx="12" cy="12" r="2"/><line x1="12" y1="12" x2="20" y2="4"/></svg>',
    'wifi-off': '<svg style="width:28px;height:28px;color:#e4e4e7" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="2" y1="2" x2="22" y2="22"/><path d="M8.5 16.5a5 5 0 0 1 7 0"/><path d="M2 8.82a15 15 0 0 1 4.17-2.65"/><path d="M10.66 5c4.01-.36 8.14.9 11.34 3.82"/><path d="M16.85 11.25a10 10 0 0 1 2.22 1.68"/><path d="M5 13a10 10 0 0 1 5.24-2.76"/><line x1="12" y1="20" x2="12.01" y2="20"/></svg>'
  }};
  var chosenIcon = svgIcons[iconType] || svgIcons['wifi-off'];

  function show(){{
    if(bannerEl) hideBannerOnly();
    if(o) return;
    var target = document.body || document.documentElement;
    if(!target){{
      if(document.readyState === "loading"){{
        document.addEventListener("DOMContentLoaded", show);
      }}
      return;
    }}
    o = document.createElement("div");
    o.id = "velora-offline-overlay";
    o.style.cssText = "position:fixed;inset:0;z-index:2147483647;display:flex;flex-direction:column;align-items:center;justify-content:center;background:" + bg + ";font-family:system-ui,-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;color:#fff;padding:2rem;text-align:center;box-sizing:border-box;-webkit-user-select:none;user-select:none;";

    if (offlineMode === 'custom' && customHtml.trim().length > 0) {{
      o.innerHTML = '<style>' + customCss + '</style>' + customHtml;
      target.appendChild(o);

      var retryTargets = o.querySelectorAll('#velora-retry-btn, [data-retry], .retry-btn');
      retryTargets.forEach(function(b) {{
        b.addEventListener('click', function() {{
          b.disabled = true;
          setTimeout(function(){{ location.reload(); }}, 600);
        }});
      }});
    }} else {{
      var helpBtnHtml = '';
      if(helpTxt && helpUrl){{
        helpBtnHtml = '<a href="' + helpUrl + '" target="_blank" rel="noopener noreferrer" style="display:inline-flex;align-items:center;gap:.4rem;padding:.6rem 1.2rem;border:1px solid rgba(255,255,255,0.1);border-radius:10px;background:rgba(255,255,255,0.04);color:#d4d4d8;text-decoration:none;font-size:.825rem;font-weight:500;cursor:pointer;transition:background .15s ease"><span>' + helpTxt + '</span></a>';
      }}

      var diagHtml = '';
      if(showDiag){{
        diagHtml = '<div style="margin-bottom:1.5rem;width:100%;max-width:340px;border-radius:12px;border:1px solid rgba(255,255,255,0.08);background:rgba(10,10,12,0.7);backdrop-filter:blur(12px);padding:.85rem 1rem;text-align:left;font-size:.75rem;color:#a1a1aa;box-sizing:border-box">' +
          '<div style="display:flex;justify-content:space-between;color:#e4e4e7;margin-bottom:.4rem;font-weight:500"><span>Network Inspector</span><span style="color:#fb7185;font-family:monospace;font-size:.7rem">ERR_NETWORK_DISCONNECTED</span></div>' +
          '<div style="display:flex;justify-content:space-between;border-top:1px solid rgba(255,255,255,0.06);padding-top:.4rem;font-family:monospace;font-size:.7rem"><span>Auto-Reconnect:</span><span style="color:#d4d4d8">' + (autoRecon ? 'Active (' + (interval/1000) + 's)' : 'Disabled') + '</span></div>' +
          '</div>';
      }}

      var graphicHtml = '';
      if(graphicMode === 'custom' && customGraphicUrl){{
        graphicHtml = '<div style="width:72px;height:72px;border-radius:18px;background:rgba(255,255,255,0.06);border:1px solid rgba(255,255,255,0.15);display:flex;align-items:center;justify-content:center;box-shadow:0 8px 24px rgba(0,0,0,0.4);overflow:hidden;margin:0 auto 1.25rem"><img src="' + customGraphicUrl + '" alt="Offline Mascot" style="width:100%;height:100%;object-fit:cover" /></div>';
      }} else {{
        graphicHtml = '<div style="margin:0 auto 1.25rem;width:56px;height:56px;border-radius:16px;background:rgba(255,255,255,0.05);border:1px solid rgba(255,255,255,0.1);display:flex;align-items:center;justify-content:center;box-shadow:0 4px 16px rgba(0,0,0,0.3)">' + chosenIcon + '</div>';
      }}

      var cardStyle = 'max-width:420px;width:100%;background:rgba(24,24,27,0.75);border:1px solid rgba(255,255,255,0.08);border-radius:20px;padding:2.25rem;backdrop-filter:blur(20px);-webkit-backdrop-filter:blur(20px);box-shadow:0 24px 48px rgba(0,0,0,0.6);display:flex;flex-direction:column;align-items:center;box-sizing:border-box';
      if(layout === 'minimal'){{
        cardStyle = 'max-width:380px;width:100%;background:transparent;border:none;box-shadow:none;padding:1.5rem;display:flex;flex-direction:column;align-items:center;box-sizing:border-box';
      }} else if(layout === 'hero'){{
        cardStyle = 'max-width:480px;width:100%;background:rgba(24,24,27,0.85);border:1px solid rgba(255,255,255,0.08);border-radius:24px;padding:2.5rem 2rem;box-shadow:0 24px 48px rgba(0,0,0,0.6);display:flex;flex-direction:column;align-items:center;box-sizing:border-box;position:relative';
      }}

      o.innerHTML = '<div style="' + cardStyle + '">' +
        (layout === 'hero' ? '<div style="position:absolute;top:1rem;left:50%;transform:translateX(-50%);width:140px;height:140px;background:rgba(255,255,255,0.04);filter:blur(50px);border-radius:50%;pointer-events:none"></div>' : '') +
        graphicHtml +
        '<h2 style="margin:0 0 .5rem;font-size:1.25rem;font-weight:600;letter-spacing:-0.02em;color:#fff">' + title + '</h2>' +
        '<p style="margin:0 0 1.5rem;font-size:.85rem;color:#a1a1aa;max-width:360px;line-height:1.5">' + msg + '</p>' +
        diagHtml +
        '<div style="display:flex;flex-wrap:wrap;align-items:center;justify-content:center;gap:.65rem">' +
        '<button id="velora-retry-btn" style="display:inline-flex;align-items:center;gap:.5rem;padding:.6rem 1.4rem;border:none;border-radius:10px;background:#ffffff;color:#09090b;font-size:.85rem;font-weight:600;cursor:pointer;box-shadow:0 1px 3px rgba(0,0,0,0.2);transition:background .15s ease,transform .1s ease">' +
        '<svg id="velora-retry-icon" style="width:14px;height:14px" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/><path d="M3 3v5h5"/><path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16"/><path d="M16 21h5v-5"/></svg>' +
        '<span id="velora-retry-text">' + retry + '</span>' +
        '</button>' +
        helpBtnHtml +
        '</div>' +
        '</div>' +
        '<style>@keyframes velora-spin-retry{{to{{transform:rotate(360deg)}}}}</style>';

      target.appendChild(o);

      var btn = document.getElementById("velora-retry-btn");
      if(btn){{
        btn.addEventListener("click", function(){{
          var ic = document.getElementById("velora-retry-icon");
          var tx = document.getElementById("velora-retry-text");
          if(ic) ic.style.animation = "velora-spin-retry 0.8s linear infinite";
          if(tx) tx.textContent = "Connecting...";
          btn.disabled = true;
          setTimeout(function(){{ location.reload(); }}, 600);
        }});
      }}
    }}

    if(autoRecon && !timer){{
      timer = setInterval(function(){{
        if(navigator.onLine){{
          fetch(window.location.origin, {{ method: 'HEAD', cache: 'no-store' }})
            .then(function(res){{
              if(res.ok || res.status < 500){{
                clearInterval(timer);
                timer = null;
                hideBannerOnly();
                location.reload();
              }}
            }})
            .catch(function(){{}});
        }}
      }}, interval);
    }}
  }}

  function hide(){{
    hideBannerOnly();
    if(timer){{ clearInterval(timer); timer = null; }}
    if(o){{ o.remove(); o = null; location.reload(); }}
  }}

  // Intercept any link clicks while offline so WebView2 does not navigate to a dead network and trigger the Edge ERR_INTERNET_DISCONNECTED error screen
  document.addEventListener("click", function(e){{
    if(!navigator.onLine){{
      var a = e.target && e.target.closest ? e.target.closest("a") : null;
      if(a && a.href && !a.href.startsWith('javascript:') && !a.href.startsWith('#')){{
        e.preventDefault();
        e.stopPropagation();
        show();
      }}
    }}
  }}, true);

  window.addEventListener("offline", show);
  window.addEventListener("online", hide);
  if(!navigator.onLine) show();
}})()"##
    )
}

fn draggable_header_script() -> String {
    String::from(
        r#"(function(){
  if(document.getElementById("velora-drag-region"))return;
  function injectDrag(){
    var d=document.createElement("div");
    d.id="velora-drag-region";
    d.setAttribute("data-tauri-drag-region","");
    d.style.cssText="position:fixed;top:0;left:0;right:0;height:28px;z-index:2147483646;pointer-events:auto;user-select:none;-webkit-user-select:none;-webkit-app-region:drag;";
    document.body.appendChild(d);
  }
  if(document.readyState==="loading"){
    document.addEventListener("DOMContentLoaded",injectDrag);
  }else{
    injectDrag();
  }
})()"#,
    )
}

#[allow(dead_code)]
fn splash_screen_script(splash: &SplashScreenConfig, app_name: &str) -> String {
    if !splash.enabled {
        return String::new();
    }

    let bg_color = if splash.bg_color.trim().is_empty() { "#0d0e12" } else { &splash.bg_color };
    let duration = splash.duration_ms;
    let title = splash.title.as_deref().unwrap_or(app_name);
    let subtext = splash.subtext.as_deref().unwrap_or("Loading application...");
    let spinner_style = splash.spinner_style.as_deref().unwrap_or("orbit");
    let image_mode = splash.image.as_deref().unwrap_or("");
    let icon_data_url = splash.icon_data_url.as_deref().unwrap_or("");
    let bg_type = splash.bg_type.as_deref().unwrap_or("solid");
    let gradient_preset = splash.gradient_preset.as_deref().unwrap_or("obsidian-violet");
    let logo_size = splash.logo_size.unwrap_or(72).clamp(48, 120);
    let entrance_animation = splash.entrance_animation.as_deref().unwrap_or("fade");
    let mode = splash.mode.as_deref().unwrap_or("visual");
    let custom_html = splash.custom_html.as_deref().unwrap_or("");
    let custom_css = splash.custom_css.as_deref().unwrap_or("");
    let footer_text = splash.footer_text.as_deref().unwrap_or("");
    let version_badge = splash.version_badge.as_deref().unwrap_or("");

    let show_logo_container = splash.show_logo_container;

    let title_escaped = serde_json::to_string(title).unwrap_or_else(|_| format!("\"{}\"", app_name));
    let subtext_escaped = serde_json::to_string(subtext).unwrap_or_else(|_| "\"Loading...\"".to_string());
    let bg_escaped = serde_json::to_string(bg_color).unwrap_or_else(|_| "\"#0d0e12\"".to_string());
    let spinner_style_escaped = serde_json::to_string(spinner_style).unwrap_or_else(|_| "\"orbit\"".to_string());
    let image_mode_escaped = serde_json::to_string(image_mode).unwrap_or_else(|_| "\"\"".to_string());
    let icon_data_url_escaped = serde_json::to_string(icon_data_url).unwrap_or_else(|_| "\"\"".to_string());
    let bg_type_escaped = serde_json::to_string(bg_type).unwrap_or_else(|_| "\"solid\"".to_string());
    let gradient_preset_escaped = serde_json::to_string(gradient_preset).unwrap_or_else(|_| "\"obsidian-violet\"".to_string());
    let entrance_animation_escaped = serde_json::to_string(entrance_animation).unwrap_or_else(|_| "\"fade\"".to_string());
    let mode_escaped = serde_json::to_string(mode).unwrap_or_else(|_| "\"visual\"".to_string());
    let custom_html_escaped = serde_json::to_string(custom_html).unwrap_or_else(|_| "\"\"".to_string());
    let custom_css_escaped = serde_json::to_string(custom_css).unwrap_or_else(|_| "\"\"".to_string());
    let footer_text_escaped = serde_json::to_string(footer_text).unwrap_or_else(|_| "\"\"".to_string());
    let version_badge_escaped = serde_json::to_string(version_badge).unwrap_or_else(|_| "\"\"".to_string());
    let app_initial = app_name.chars().find(|c| c.is_alphanumeric()).unwrap_or('V').to_uppercase().to_string();
    let app_initial_escaped = serde_json::to_string(&app_initial).unwrap_or_else(|_| "\"V\"".to_string());

    format!(
        r#"(function(){{
  var s = document.createElement("div");
  s.id = "velora-splash-screen";

  var bgType = {bg_type_escaped};
  var gradPreset = {gradient_preset_escaped};
  var solidBg = {bg_escaped};
  var gradients = {{
    sunset: 'linear-gradient(135deg, #1e1b4b 0%, #431407 50%, #18181b 100%)',
    aurora: 'linear-gradient(135deg, #022c22 0%, #082f49 50%, #0f172a 100%)',
    cosmic: 'linear-gradient(135deg, #09090b 0%, #2e1065 50%, #030712 100%)',
    ocean: 'linear-gradient(135deg, #0c4a6e 0%, #1e1b4b 100%)',
    dark: 'linear-gradient(180deg, #18181b 0%, #09090b 100%)',
    'obsidian-violet': 'linear-gradient(135deg, #0f0728 0%, #180b38 40%, #09090b 100%)',
    'cyber-emerald': 'linear-gradient(135deg, #022c22 0%, #064e3b 50%, #021f18 100%)',
    'midnight-blue': 'linear-gradient(135deg, #031525 0%, #0c2340 50%, #020b14 100%)'
  }};
  var effectiveBg = (bgType === 'gradient' && gradients[gradPreset]) ? gradients[gradPreset] : solidBg;

  s.style.cssText = "position:fixed;inset:0;z-index:2147483647;display:flex;flex-direction:column;align-items:center;justify-content:center;background:" + effectiveBg + ";color:#fff;font-family:system-ui,-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;transition:opacity 0.4s ease,transform 0.4s ease;user-select:none;-webkit-user-select:none;";

  var splashMode = {mode_escaped};
  var customHtml = {custom_html_escaped};
  var customCss = {custom_css_escaped};

  if (splashMode === 'custom' && customHtml.trim().length > 0) {{
    s.innerHTML = '<style>' + customCss + '</style>' + customHtml;
  }} else {{
    var imgMode = {image_mode_escaped};
    var iconUrl = {icon_data_url_escaped};
    var initial = {app_initial_escaped};
    var logoSz = {logo_size};
    var showContainer = {show_logo_container};
    var iconHtml = '';

    var boxStyle = showContainer
      ? 'width:' + logoSz + 'px;height:' + logoSz + 'px;border-radius:18px;background:rgba(24,24,27,0.9);border:1px solid rgba(255,255,255,0.1);display:flex;align-items:center;justify-content:center;box-shadow:0 12px 30px rgba(0,0,0,0.5);backdrop-filter:blur(12px);-webkit-backdrop-filter:blur(12px);overflow:hidden'
      : 'width:' + logoSz + 'px;height:' + logoSz + 'px;display:flex;align-items:center;justify-content:center;overflow:hidden';

    if (imgMode === 'sparkles' || imgMode === 'emblem') {{
      iconHtml = '<div style="position:relative;margin-bottom:1.25rem"><div style="' + boxStyle + '"><svg style="width:' + Math.round(logoSz * 0.45) + 'px;height:' + Math.round(logoSz * 0.45) + 'px;color:#e4e4e7" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="5"/><circle cx="12" cy="12" r="4"/></svg></div></div>';
    }} else if (imgMode === 'none') {{
      iconHtml = '';
    }} else if (imgMode && (imgMode.indexOf('data:image') === 0 || imgMode.indexOf('http') === 0)) {{
      iconHtml = '<div style="position:relative;margin-bottom:1.25rem"><div style="' + boxStyle + '"><img src="' + imgMode + '" alt="Splash Graphic" style="width:100%;height:100%;object-fit:' + (showContainer ? 'cover' : 'contain') + ';border-radius:' + (showContainer ? '18px' : '0') + ';" /></div></div>';
    }} else if (iconUrl && iconUrl.length > 50) {{
      iconHtml = '<div style="position:relative;margin-bottom:1.25rem"><div style="' + boxStyle + '"><img src="' + iconUrl + '" alt="App Icon" style="width:' + (showContainer ? '70%' : '100%') + ';height:' + (showContainer ? '70%' : '100%') + ';object-fit:contain;border-radius:' + (showContainer ? '10px' : '0') + ';" /></div></div>';
    }} else {{
      iconHtml = '<div style="position:relative;margin-bottom:1.25rem"><div style="' + (showContainer ? boxStyle : 'width:' + logoSz + 'px;height:' + logoSz + 'px;border-radius:18px;background:linear-gradient(180deg,#27272a,#18181b);border:1px solid rgba(255,255,255,0.15);display:flex;align-items:center;justify-content:center;box-shadow:0 12px 30px rgba(0,0,0,0.5);color:#fff;font-weight:600;font-size:' + Math.round(logoSz * 0.38) + 'px') + '">' + initial + '</div></div>';
    }}

    var sp = {spinner_style_escaped};
    var spHtml = '';
    if (sp === 'bar') {{
      spHtml = '<div style="position:relative;width:140px;height:3px;border-radius:9999px;background:rgba(255,255,255,0.12);overflow:hidden;margin:0 auto 1.5rem"><div style="position:absolute;top:0;bottom:0;left:0;width:50px;border-radius:9999px;background:#e4e4e7;animation:velora-bar-slide 1.2s cubic-bezier(0.4,0,0.2,1) infinite"></div></div>';
    }} else if (sp === 'pulse') {{
      spHtml = '<div style="display:flex;align-items:center;justify-content:center;height:20px;margin:0 auto 1.5rem"><span style="width:8px;height:8px;border-radius:50%;background:#d4d4d8;animation:velora-pulse-fade 1.5s ease-in-out infinite"></span></div>';
    }} else if (sp === 'dots') {{
      spHtml = '<div style="display:flex;align-items:center;justify-content:center;gap:6px;height:20px;margin:0 auto 1.5rem"><span style="width:5px;height:5px;border-radius:50%;background:#e4e4e7;animation:velora-bounce-dot 1s cubic-bezier(0.34,1.56,0.64,1) infinite"></span><span style="width:5px;height:5px;border-radius:50%;background:#e4e4e7;animation:velora-bounce-dot 1s cubic-bezier(0.34,1.56,0.64,1) infinite;animation-delay:0.15s"></span><span style="width:5px;height:5px;border-radius:50%;background:#e4e4e7;animation:velora-bounce-dot 1s cubic-bezier(0.34,1.56,0.64,1) infinite;animation-delay:0.3s"></span></div>';
    }} else if (sp === 'progress') {{
      spHtml = '<div style="width:170px;margin:0 auto 1.5rem"><div style="height:3px;border-radius:9999px;background:rgba(255,255,255,0.12);overflow:hidden;margin-bottom:6px"><div id="velora-progress-bar" style="height:100%;width:0%;border-radius:9999px;background:#e4e4e7"></div></div><div style="display:flex;justify-content:space-between;font-size:10px;font-family:ui-monospace,monospace;color:rgba(255,255,255,0.5)"><span>Loading</span><span id="velora-progress-val">0%</span></div></div>';
    }} else {{
      spHtml = '<div style="display:flex;align-items:center;justify-content:center;height:24px;margin:0 auto 1.5rem"><svg style="width:22px;height:22px;animation:velora-spin-cw 0.9s linear infinite" viewBox="0 0 24 24" fill="none"><circle cx="12" cy="12" r="10" stroke="rgba(255,255,255,0.15)" stroke-width="2.5"></circle><path fill="currentColor" opacity="0.9" d="M4 12a8 8 0 018-8v4a4 4 0 00-4 4H4z"></path></svg></div>';
    }}

    var subtextVal = {subtext_escaped};
    var subtextHtml = '';
    if (subtextVal && subtextVal.trim() !== '') {{
      subtextHtml = '<div style="font-size:0.75rem;color:rgba(255,255,255,0.45);margin-bottom:1.5rem;letter-spacing:0.01em">' + subtextVal + '</div>';
    }}

    var verBadge = {version_badge_escaped};
    var titleHtml = '<h1 style="margin:0 0 .25rem;font-size:1.25rem;font-weight:600;letter-spacing:-0.02em;color:#ffffff">' + {title_escaped} + '</h1>';

    var footerVal = {footer_text_escaped};
    var footerHtml = (verBadge || footerVal) ? '<div style="position:absolute;bottom:1.5rem;left:0;right:0;text-align:center;font-size:11px;color:rgba(255,255,255,0.4);font-family:system-ui,sans-serif">' + (verBadge ? '<span style="font-family:monospace;font-size:10px;color:rgba(255,255,255,0.5);display:block;margin-bottom:2px">v' + verBadge + '</span>' : '') + footerVal + '</div>' : '';

    var entrance = {entrance_animation_escaped};
    var entranceStyle = 'animation: velora-splash-fade 0.4s ease forwards;';
    if(entrance === 'zoom') entranceStyle = 'animation: velora-splash-zoom 0.5s cubic-bezier(0.16, 1, 0.3, 1) forwards;';
    else if(entrance === 'shimmer') entranceStyle = 'animation: velora-splash-breathe 2.5s ease-in-out infinite alternate;';
    else if(entrance === 'none') entranceStyle = '';

    var keyframes = '<style>' +
      '@keyframes velora-splash-fade{{from{{opacity:0;transform:translateY(6px)}}to{{opacity:1;transform:translateY(0)}}}}' +
      '@keyframes velora-splash-zoom{{from{{opacity:0;transform:scale(0.9)}}to{{opacity:1;transform:scale(1)}}}}' +
      '@keyframes velora-splash-breathe{{0%{{opacity:0.8;transform:scale(0.98)}}100%{{opacity:1;transform:scale(1.02)}}}}' +
      '@keyframes velora-spin-cw{{0%{{transform:rotate(0deg)}}100%{{transform:rotate(360deg)}}}}' +
      '@keyframes velora-bar-slide{{0%{{transform:translateX(-50px)}}100%{{transform:translateX(140px)}}}}' +
      '@keyframes velora-pulse-fade{{0%,100%{{opacity:0.3;transform:scale(0.85)}}50%{{opacity:1;transform:scale(1.15)}}}}' +
      '@keyframes velora-bounce-dot{{0%,100%{{transform:translateY(0);opacity:0.4}}50%{{transform:translateY(-6px);opacity:1}}}}' +
      '</style>';

    s.innerHTML = '<div style="text-align:center;padding:2rem;max-width:380px;display:flex;flex-direction:column;align-items:center;' + entranceStyle + '">' +
      iconHtml +
      titleHtml +
      (subtextHtml ? subtextHtml : '<div style="margin-bottom:1.5rem"></div>') +
      spHtml +
      '</div>' + footerHtml + keyframes;

    if (sp === 'progress') {{
      var progStart = Date.now();
      var progDuration = {duration};
      var pBar = s.querySelector('#velora-progress-bar');
      var pVal = s.querySelector('#velora-progress-val');
      var progTimer = setInterval(function() {{
        var pct = Math.min(100, Math.round(((Date.now() - progStart) / progDuration) * 100));
        if (pBar) pBar.style.width = pct + '%';
        if (pVal) pVal.textContent = pct + '%';
        if (pct >= 100) clearInterval(progTimer);
      }}, 50);
    }}
  }}

  function ensureSplashMounted(){{
    var existing = document.getElementById("velora-splash-screen");
    if(!existing){{
      var target = document.body || document.documentElement;
      if(target) target.appendChild(s);
    }} else if(document.body && existing.parentNode !== document.body){{
      document.body.appendChild(existing);
    }}
  }}

  ensureSplashMounted();
  if(document.readyState === "loading"){{
    document.addEventListener("DOMContentLoaded", ensureSplashMounted);
  }}

  var obs = new MutationObserver(function(){{
    ensureSplashMounted();
  }});
  if(document.documentElement){{
    obs.observe(document.documentElement, {{ childList: true, subtree: true }});
  }}

  var start = Date.now();
  var minDuration = {duration};
  var dismissed = false;

  function dismiss(){{
    if(dismissed) return;
    var elapsed = Date.now() - start;
    var remain = Math.max(0, minDuration - elapsed);
    setTimeout(function(){{
      if(dismissed) return;
      dismissed = true;
      if(obs) {{ obs.disconnect(); obs = null; }}
      s.style.opacity = "0";
      s.style.transform = "scale(1.02)";
      s.style.pointerEvents = "none";
      setTimeout(function(){{
        if(s.parentNode) s.parentNode.removeChild(s);
      }}, 450);
    }}, remain);
  }}

  if(document.readyState === "complete"){{
    dismiss();
  }} else {{
    window.addEventListener("load", dismiss);
    setTimeout(dismiss, minDuration + 6000);
  }}
}})()"#
    )
}

fn android_navigation_script() -> String {
    String::from(
        r#"(function(){
  if(window.__VELORA_ANDROID_BRIDGE__) return;
  window.__VELORA_ANDROID_BRIDGE__ = true;

  window.addEventListener("popstate", function(){});
  window.addEventListener("tauri://back-button", function(){
    if(window.history.length > 1){
      window.history.back();
    }
  });

  function applySafeViewport(){
    try {
      var meta = document.querySelector('meta[name="viewport"]');
      if (meta) {
        if (!meta.content.includes('viewport-fit')) {
          meta.content += ', viewport-fit=cover';
        }
      } else {
        var m = document.createElement('meta');
        m.name = 'viewport';
        m.content = 'width=device-width, initial-scale=1.0, viewport-fit=cover';
        (document.head || document.documentElement).appendChild(m);
      }

      var styleId = 'velora-android-safe-insets';
      if (!document.getElementById(styleId)) {
        var s = document.createElement('style');
        s.id = styleId;
        s.textContent = ':root{--sat:env(safe-area-inset-top,0px);--sab:env(safe-area-inset-bottom,0px);--sal:env(safe-area-inset-left,0px);--sar:env(safe-area-inset-right,0px);}';
        (document.head || document.documentElement).appendChild(s);
      }
    } catch(e) {}
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', applySafeViewport);
  } else {
    applySafeViewport();
  }
})()"#,
    )
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config: VeloraConfig =
        serde_json::from_str(VELORA_CONFIG).expect("Failed to parse velora-config.json");

    let main_url: Url = Url::parse(&config.website_url)
        .or_else(|_| Url::parse(&format!("https://{}", config.website_url)))
        .expect("Invalid website URL in velora-config.json");

    let main_host = main_url
        .host_str()
        .expect("Website URL must have a host")
        .to_owned();

    let allowed_domains = config.allowed_domains.clone();
    let app_name = config.app_name.clone();

    let mut scripts: Vec<String> = Vec::new();

    // Note: Splash screen is natively rendered by the local bundled index.html (0ms cold start).
    // It is intentionally omitted from `scripts` to prevent it from re-injecting when remote pages load.

    let css_script = css_injection_script(INJECTION_CSS);
    if !css_script.is_empty() {
        scripts.push(css_script);
    }

    let js = INJECTION_JS.trim();
    if !js.is_empty() {
        if config.injection_timing.as_deref() == Some("document_idle") {
            scripts.push(format!(
                "(function(){{if(document.readyState==='complete'||document.readyState==='interactive'){{{}}}else{{document.addEventListener('DOMContentLoaded',function(){{{}}});}}}})();",
                js, js
            ));
        } else {
            scripts.push(js.to_owned());
        }
    }

    if config.enable_offline_fallback {
        scripts.push(offline_detection_script(&config.offline_config));
    }

    let is_frameless = !config.window.decorations
        || config.window.titlebar_style.as_deref() == Some("borderless")
        || config.window.titlebar_style.as_deref() == Some("overlay");
    if is_frameless {
        scripts.push(draggable_header_script());
    }

    scripts.push(android_navigation_script());

    let combined_script = scripts.join("\n");

    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init());

    #[cfg(desktop)]
    if config.enable_single_instance {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.show();
                let _ = w.unminimize();
                let _ = w.set_focus();
            }
        }));
    }

    #[cfg(desktop)]
    if config.window.remember_window_state {
        builder = builder.plugin(tauri_plugin_window_state::Builder::default().build());
    }

    builder
        .setup(move |app| {
            let _window = if let Some(existing_window) = app.get_webview_window("main") {
                let _ = existing_window.navigate(main_url.clone());
                let _ = existing_window.set_title(&app_name);
                if !combined_script.is_empty() {
                    let _ = existing_window.eval(&combined_script);
                }
                #[cfg(desktop)]
                {
                    let _ = existing_window.show();
                    let _ = existing_window.set_focus();
                }
                existing_window
            } else {
                let start_url = if config.splash_screen.enabled || config.enable_offline_fallback {
                    WebviewUrl::App("index.html".into())
                } else {
                    WebviewUrl::External(main_url.clone())
                };

                #[allow(unused_mut)]
                let mut wb = WebviewWindowBuilder::new(
                    app,
                    "main",
                    start_url,
                )
                .title(&app_name);

                #[cfg(desktop)]
                {
                    wb = wb
                        .inner_size(config.window.width, config.window.height)
                        .resizable(config.window.resizable)
                        .fullscreen(config.window.fullscreen)
                        .decorations(config.window.decorations)
                        .always_on_top(config.window.always_on_top)
                        .minimizable(config.window.show_minimize_button)
                        .maximizable(config.window.show_maximize_button)
                        .closable(config.window.show_close_button)
                        .devtools(config.enable_devtools)
                        .center()
                        .visible(true);

                    if let (Some(mw), Some(mh)) = (config.window.min_width, config.window.min_height) {
                        wb = wb.min_inner_size(mw, mh);
                    }
                }

                if let Some(ref ua) = config.user_agent {
                    wb = wb.user_agent(ua);
                }

                if !combined_script.is_empty() {
                    wb = wb.initialization_script(&combined_script);
                }

                let handle = app.handle().clone();
                let main_host_clone = main_host.clone();
                let allowed_domains_clone = allowed_domains.clone();
                let sandbox_links = config.sandbox_external_links;
                wb = wb.on_navigation(move |url| {
                    let path = url.path().to_lowercase();
                    let is_download_asset = path.ends_with(".pdf")
                        || path.ends_with(".docx")
                        || path.ends_with(".doc")
                        || path.ends_with(".xlsx")
                        || path.ends_with(".xls")
                        || path.ends_with(".zip")
                        || path.ends_with(".rar")
                        || path.ends_with(".apk")
                        || path.ends_with(".csv")
                        || url.as_str().contains("/storage/v1/object/");

                    #[cfg(mobile)]
                    if is_download_asset {
                        let _ = handle.opener().open_url(url.as_str(), None::<&str>);
                        return false;
                    }

                    if !sandbox_links || is_domain_allowed(url, &main_host_clone, &allowed_domains_clone) {
                        return true;
                    }
                    let _ = handle.opener().open_url(url.as_str(), None::<&str>);
                    false
                });

                wb = wb.on_page_load(move |window, payload| {
                    if let tauri::webview::PageLoadEvent::Finished = payload.event() {
                        let u = payload.url().as_str();
                        if u.contains("chromewebdata") || u.starts_with("chrome-error://") || u.starts_with("edge://") {
                            if let Ok(off_url) = tauri::Url::parse("http://tauri.localhost/offline.html") {
                                let _ = window.navigate(off_url);
                            }
                        }
                    }
                });

                let w = wb.build()?;
                #[cfg(desktop)]
                {
                    let _ = w.show();
                    let _ = w.set_focus();
                }
                w
            };

            #[cfg(desktop)]
            if config.enable_tray {
                use tauri::menu::{Menu, MenuItem};
                use tauri::tray::TrayIconBuilder;

                if let Some(icon) = app.default_window_icon() {
                    let mut menu_items: Vec<MenuItem<tauri::Wry>> = Vec::new();

                    if let Some(ref items) = config.tray_menu_config {
                        for (idx, item) in items.iter().enumerate() {
                            let item_id = if item.action == "url" {
                                format!("url:{}", item.url.as_deref().unwrap_or(""))
                            } else {
                                format!("{}:{}", item.action, idx)
                            };
                            if let Ok(mi) = MenuItem::with_id(app, &item_id, &item.label, true, None::<&str>) {
                                menu_items.push(mi);
                            }
                        }
                    }

                    if menu_items.is_empty() {
                        if let (Ok(show_item), Ok(quit_item)) = (
                            MenuItem::with_id(app, "show:0", "Open App", true, None::<&str>),
                            MenuItem::with_id(app, "quit:1", "Quit", true, None::<&str>),
                        ) {
                            menu_items.push(show_item);
                            menu_items.push(quit_item);
                        }
                    }

                    let item_refs: Vec<&dyn tauri::menu::IsMenuItem<tauri::Wry>> = menu_items
                        .iter()
                        .map(|item| item as &dyn tauri::menu::IsMenuItem<tauri::Wry>)
                        .collect();
                    if let Ok(menu) = Menu::with_items(app, &item_refs) {
                        let tray_builder = TrayIconBuilder::new()
                            .menu(&menu)
                            .icon(icon.clone())
                            .tooltip(&config.app_name)
                            .on_menu_event(|app_handle, event| {
                                let id_str = event.id.as_ref();
                                if id_str.starts_with("show") {
                                    if let Some(w) = app_handle.get_webview_window("main") {
                                        let _ = w.show();
                                        let _ = w.unminimize();
                                        let _ = w.set_focus();
                                    }
                                } else if id_str.starts_with("hide") {
                                    if let Some(w) = app_handle.get_webview_window("main") {
                                        let _ = w.hide();
                                    }
                                } else if id_str.starts_with("quit") {
                                    app_handle.exit(0);
                                } else if id_str.starts_with("url:") {
                                    let url_target = &id_str[4..];
                                    if !url_target.is_empty() {
                                        let _ = app_handle.opener().open_url(url_target, None::<&str>);
                                    }
                                }
                            });

                        let _ = tray_builder.build(app);
                    }
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
