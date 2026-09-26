use serde::Deserialize;
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_opener::OpenerExt;
use url::Url;

mod scripts;
use scripts::*;

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
pub(crate) struct SplashScreenConfig {
    #[serde(default)]
    pub(crate) enabled: bool,
    #[serde(default = "default_splash_duration")]
    pub(crate) duration_ms: u64,
    #[serde(default = "default_splash_bg")]
    pub(crate) bg_color: String,
    #[serde(default)]
    pub(crate) title: Option<String>,
    #[serde(default)]
    pub(crate) subtext: Option<String>,
    #[serde(default)]
    pub(crate) spinner_style: Option<String>,
    #[serde(default)]
    pub(crate) image: Option<String>,
    #[serde(default)]
    pub(crate) icon_data_url: Option<String>,
    #[serde(default)]
    pub(crate) bg_type: Option<String>,
    #[serde(default)]
    pub(crate) gradient_preset: Option<String>,
    #[serde(default)]
    pub(crate) logo_size: Option<u32>,
    #[serde(default)]
    pub(crate) entrance_animation: Option<String>,
    #[serde(default)]
    pub(crate) mode: Option<String>,
    #[serde(default)]
    pub(crate) custom_html: Option<String>,
    #[serde(default)]
    pub(crate) custom_css: Option<String>,
    #[serde(default)]
    pub(crate) footer_text: Option<String>,
    #[serde(default)]
    pub(crate) version_badge: Option<String>,
    #[serde(default = "default_true")]
    pub(crate) show_logo_container: bool,
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
pub(crate) struct OfflineConfig {
    #[serde(default = "default_offline_title")]
    pub(crate) title: String,
    #[serde(default = "default_offline_message")]
    pub(crate) message: String,
    #[serde(default = "default_offline_retry")]
    pub(crate) retry_text: String,
    #[serde(default = "default_offline_bg")]
    pub(crate) theme_bg: String,
    #[serde(default = "default_offline_accent")]
    pub(crate) accent_color: String,
    #[serde(default = "default_offline_icon")]
    pub(crate) icon: String,
    #[serde(default = "default_true")]
    pub(crate) auto_reconnect: bool,
    #[serde(default = "default_reconnect_interval")]
    pub(crate) reconnect_interval: u64,
    #[serde(default)]
    pub(crate) help_text: Option<String>,
    #[serde(default)]
    pub(crate) help_url: Option<String>,
    #[serde(default)]
    pub(crate) show_diagnostics: bool,
    #[serde(default = "default_true")]
    pub(crate) show_network_status_banner: bool,
    #[serde(default)]
    pub(crate) layout: Option<String>,
    #[serde(default)]
    pub(crate) graphic_mode: Option<String>,
    #[serde(default)]
    pub(crate) custom_graphic_url: Option<String>,
    #[serde(default)]
    pub(crate) mode: Option<String>,
    #[serde(default)]
    pub(crate) custom_html: Option<String>,
    #[serde(default)]
    pub(crate) custom_css: Option<String>,
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

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct DevicePermissionsConfig {
    #[serde(default = "default_true")]
    storage: bool,
    #[serde(default)]
    camera: bool,
    #[serde(default)]
    microphone: bool,
    #[serde(default)]
    geolocation: bool,
    #[serde(default = "default_true")]
    notifications: bool,
    #[serde(default = "default_true")]
    external_app_schemes: bool,
}

impl Default for DevicePermissionsConfig {
    fn default() -> Self {
        Self {
            storage: true,
            camera: false,
            microphone: false,
            geolocation: false,
            notifications: true,
            external_app_schemes: true,
        }
    }
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
    #[serde(default)]
    device_permissions: DevicePermissionsConfig,
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
        let mut d = domain.trim();
        if d.is_empty() {
            continue;
        }
        if let Some(stripped) = d.strip_prefix("https://") {
            d = stripped;
        } else if let Some(stripped) = d.strip_prefix("http://") {
            d = stripped;
        }
        d = d.trim_end_matches('/');
        let stripped_d = d.strip_prefix("*.").unwrap_or(d).strip_prefix("www.").unwrap_or(d);
        if stripped_nav == stripped_d || stripped_nav.ends_with(&format!(".{}", stripped_d)) {
            return true;
        }
    }

    false
}

fn is_download_url(url: &Url) -> bool {
    let path = url.path().to_lowercase();
    let path_no_query = path.split('?').next().unwrap_or(&path);
    let path_no_frag = path_no_query.split('#').next().unwrap_or(path_no_query);

    let download_extensions = [
        ".pdf", ".docx", ".doc", ".xlsx", ".xls", ".pptx", ".ppt",
        ".zip", ".rar", ".7z", ".tar", ".gz",
        ".apk", ".csv", ".mp3", ".mp4", ".avi", ".mkv",
        ".epub", ".iso", ".dmg", ".exe", ".msi",
    ];
    for ext in &download_extensions {
        if path_no_frag.ends_with(ext) {
            return true;
        }
    }

    let full = url.as_str().to_lowercase();
    if full.contains("/storage/v1/object/") {
        return true;
    }
    if full.contains("download=true") || full.contains("response-content-disposition=attachment") {
        return true;
    }

    false
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
    scripts.push(android_print_script());

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
                let allow_external_schemes = config.device_permissions.external_app_schemes;
                let _allow_storage = config.device_permissions.storage;
                wb = wb.on_navigation(move |url| {
                    let scheme = url.scheme().to_lowercase();
                    let host = url.host_str().unwrap_or("").to_lowercase();
                    let is_wa = scheme == "whatsapp"
                        || host == "wa.me"
                        || host.ends_with(".wa.me")
                        || host == "api.whatsapp.com"
                        || host == "chat.whatsapp.com";

                    if is_wa
                        || scheme == "tel"
                        || scheme == "mailto"
                        || scheme == "sms"
                        || scheme == "intent"
                        || scheme == "market"
                        || scheme == "rawbt"
                        || scheme == "printer"
                        || scheme == "escpos"
                        || scheme == "quickprinter"
                    {
                        if allow_external_schemes {
                            #[allow(unused_mut)]
                            let mut u_str = url.as_str().to_string();
                            #[cfg(target_os = "android")]
                            if is_wa {
                                if host == "wa.me" || host.ends_with(".wa.me") {
                                    let path = url.path().trim_start_matches('/');
                                    let mut phone = String::new();
                                    if !path.is_empty() && !path.starts_with("send") {
                                        phone = path.to_string();
                                    }
                                    let mut text = String::new();
                                    for (k, v) in url.query_pairs() {
                                        if k == "phone" && phone.is_empty() {
                                            phone = v.to_string();
                                        } else if k == "text" {
                                            text = v.to_string();
                                        }
                                    }
                                    let clean_phone: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();
                                    if !clean_phone.is_empty() || !text.is_empty() {
                                        let mut wa_url = "whatsapp://send?".to_string();
                                        if !clean_phone.is_empty() {
                                            wa_url.push_str(&format!("phone={}", clean_phone));
                                        }
                                        if !text.is_empty() {
                                            if !clean_phone.is_empty() {
                                                wa_url.push('&');
                                            }
                                            wa_url.push_str("text=");
                                            for byte in text.bytes() {
                                                if byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_' || byte == b'.' || byte == b'~' {
                                                    wa_url.push(byte as char);
                                                } else {
                                                    wa_url.push_str(&format!("%{:02X}", byte));
                                                }
                                            }
                                        }
                                        u_str = wa_url;
                                    }
                                } else if host == "api.whatsapp.com" || host.ends_with(".whatsapp.com") {
                                    let mut phone = String::new();
                                    let mut text = String::new();
                                    for (k, v) in url.query_pairs() {
                                        if k == "phone" {
                                            phone = v.to_string();
                                        } else if k == "text" {
                                            text = v.to_string();
                                        }
                                    }
                                    let clean_phone: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();
                                    if !clean_phone.is_empty() || !text.is_empty() {
                                        let mut wa_url = "whatsapp://send?".to_string();
                                        if !clean_phone.is_empty() {
                                            wa_url.push_str(&format!("phone={}", clean_phone));
                                        }
                                        if !text.is_empty() {
                                            if !clean_phone.is_empty() {
                                                wa_url.push('&');
                                            }
                                            wa_url.push_str("text=");
                                            for byte in text.bytes() {
                                                if byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_' || byte == b'.' || byte == b'~' {
                                                    wa_url.push(byte as char);
                                                } else {
                                                    wa_url.push_str(&format!("%{:02X}", byte));
                                                }
                                            }
                                        }
                                        u_str = wa_url;
                                    }
                                }
                            }
                            let h = handle.clone();
                            tauri::async_runtime::spawn(async move {
                                let _ = h.opener().open_url(&u_str, None::<&str>);
                            });
                        }
                        return false;
                    }

                    if (scheme == "http" || scheme == "https") && is_download_url(url) {
                        let u_str = url.as_str().to_string();
                        let h = handle.clone();
                        tauri::async_runtime::spawn(async move {
                            let _ = h.opener().open_url(&u_str, None::<&str>);
                        });
                        return false;
                    }

                    if !sandbox_links || is_domain_allowed(url, &main_host_clone, &allowed_domains_clone) {
                        return true;
                    }

                    let u_str = url.as_str().to_string();
                    let h = handle.clone();
                    tauri::async_runtime::spawn(async move {
                        let _ = h.opener().open_url(&u_str, None::<&str>);
                    });
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
