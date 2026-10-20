// EduX kompyuter ilovasi (Windows, macOS, Linux): tanlangan EduX serverining veb ilovasini
// oynada ochadi. Hamma rollar (platforma egasi, direktor, admin, kassir, marketolog, o'qituvchi,
// o'quvchi, ota-ona) bir xil veb ilovadan foydalanadi; ilova esa native imkoniyatlarni qo'shadi.
mod i18n;
mod net;
mod settings;

use i18n::t;
use settings::{same_site, Settings};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;
use std::time::Duration;
use tauri::menu::{Menu, MenuBuilder, MenuItemBuilder, PredefinedMenuItem, SubmenuBuilder};
use tauri::webview::{DownloadEvent, NewWindowResponse, PermissionKind, PermissionResponse};
use tauri::{AppHandle, Manager, Url, Webview, WebviewUrl, WebviewWindow, WebviewWindowBuilder};
use tauri_plugin_deep_link::DeepLinkExt;
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_opener::OpenerExt;

const INJECT: &str = include_str!("inject.js");
const UPDATE_INTERVAL: Duration = Duration::from_secs(6 * 60 * 60);

/// Tashqi saytlardan iframe orqali qo'yiladigan kontent (video, Telegram vidjeti) — ilova ichida qoladi.
/// (Linux WebKitGTK navigatsiya tekshiruvi iframe'larga ham tegadi.)
const EMBED_HOSTS: &[&str] = &[
    "www.youtube.com",
    "youtube.com",
    "www.youtube-nocookie.com",
    "player.vimeo.com",
    "rutube.ru",
    "oauth.telegram.org",
    "telegram.org",
    "www.google.com",
    "docs.google.com",
    "drive.google.com",
];

pub struct AppState {
    settings: Mutex<Settings>,
    path: PathBuf,
    /// Server ochilgach o'tiladigan yo'l (deep link).
    pending_path: Mutex<Option<String>>,
    /// Oxirgi yuklash xatosi (mahalliy xato sahifasi uchun).
    last_error: Mutex<Option<String>>,
    notified_version: Mutex<Option<String>>,
    popup_seq: AtomicU32,
}

impl AppState {
    fn settings(&self) -> Settings {
        self.settings.lock().unwrap().clone()
    }
    /// Faol server: foydalanuvchi o'zgartirgan bo'lsa — o'sha, aks holda yig'ishdagi sukut server.
    /// (Birinchi ishga tushishda manzil so'ralmaydi — darhol <server>/login ochiladi.)
    fn server(&self) -> Option<String> {
        Some(self.settings.lock().unwrap().server.clone().unwrap_or_else(settings::default_server))
    }
    fn lang(&self) -> String {
        self.settings.lock().unwrap().effective_lang()
    }
    fn update<F: FnOnce(&mut Settings)>(&self, f: F) {
        let mut s = self.settings.lock().unwrap();
        f(&mut s);
        s.save(&self.path);
    }
}

/// Ilovaning o'z sahifasi (ui/index.html) manzili.
fn local_url(hash: &str) -> Url {
    let base = if cfg!(windows) { "http://tauri.localhost/index.html" } else { "tauri://localhost/index.html" };
    Url::parse(&format!("{base}#{hash}")).unwrap()
}

fn is_local(url: &Url) -> bool {
    url.scheme() == "tauri" || url.host_str() == Some("tauri.localhost")
}

/// Buyruq ilova sahifasidan yoki tanlangan serverdan kelganmi.
fn trusted(app: &AppHandle, webview: &Webview) -> bool {
    let Ok(url) = webview.url() else { return false };
    if is_local(&url) {
        return true;
    }
    app.state::<AppState>().server().map(|s| same_site(&url, &s)).unwrap_or(false)
}

fn main_window(app: &AppHandle) -> Option<WebviewWindow> {
    app.get_webview_window("main")
}

/// Menyu buyrug'i qaysi oynaga: fokusdagi, bo'lmasa asosiy.
fn target_window(app: &AppHandle) -> Option<WebviewWindow> {
    app.webview_windows()
        .into_values()
        .find(|w| w.is_focused().unwrap_or(false))
        .or_else(|| main_window(app))
}

fn open_external_url(app: &AppHandle, url: &str) {
    let Ok(parsed) = Url::parse(url) else { return };
    // Faqat xavfsiz sxemalar (file:// va dasturlar emas).
    let allowed = ["http", "https", "mailto", "tel", "sms", "tg", "whatsapp", "viber", "skype", "zoommtg"];
    if allowed.contains(&parsed.scheme()) {
        let _ = app.opener().open_url(parsed.as_str(), None::<&str>);
    }
}

// ---------------------------------------------------------------- fayl saqlash

fn default_dir(app: &AppHandle) -> Option<PathBuf> {
    app.path().download_dir().ok()
}

fn sanitize_name(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| if matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|') || c.is_control() { '_' } else { c })
        .collect();
    let cleaned = cleaned.trim().trim_matches('.').to_string();
    if cleaned.is_empty() { "fayl".into() } else { cleaned }
}

/// Native «Saqlash» oynasi. Faqat asosiy (UI) oqimda chaqiriladi.
fn save_dialog_on_main(app: &AppHandle, name: &str) -> Option<PathBuf> {
    let name = sanitize_name(name);
    let title = t(&app.state::<AppState>().lang(), "save.title");
    let dir = default_dir(app);
    #[cfg(target_os = "linux")]
    {
        use gtk::prelude::*;
        let parent = main_window(app).and_then(|w| w.gtk_window().ok());
        let dialog = gtk::FileChooserDialog::with_buttons(
            Some(title.as_str()),
            parent.as_ref(),
            gtk::FileChooserAction::Save,
            &[("_Cancel", gtk::ResponseType::Cancel), ("_Save", gtk::ResponseType::Accept)],
        );
        dialog.set_do_overwrite_confirmation(true);
        dialog.set_modal(true);
        if let Some(dir) = dir {
            let _ = dialog.set_current_folder(dir);
        }
        dialog.set_current_name(&name);
        let response = dialog.run();
        let path = if response == gtk::ResponseType::Accept { dialog.filename() } else { None };
        dialog.close();
        path
    }
    #[cfg(not(target_os = "linux"))]
    {
        let mut dialog = rfd::FileDialog::new().set_title(&title).set_file_name(&name);
        if let Some(dir) = dir {
            dialog = dialog.set_directory(dir);
        }
        if let Some(ext) = std::path::Path::new(&name).extension().and_then(|e| e.to_str()) {
            dialog = dialog.add_filter(ext.to_uppercase(), &[ext]);
        }
        dialog.save_file()
    }
}

/// Ishchi oqimdan: dialog asosiy oqimda ochiladi, natija kutiladi.
fn save_dialog(app: &AppHandle, name: &str) -> Option<PathBuf> {
    let (tx, rx) = std::sync::mpsc::channel();
    let handle = app.clone();
    let name = name.to_string();
    app.run_on_main_thread(move || {
        let _ = tx.send(save_dialog_on_main(&handle, &name));
    })
    .ok()?;
    rx.recv().ok().flatten()
}

fn notify(app: &AppHandle, title: &str, body: &str) {
    let _ = app.notification().builder().title(title).body(body).show();
}

fn message_on_main(app: &AppHandle, title: String, text: String) {
    let handle = app.clone();
    let _ = app.run_on_main_thread(move || {
        #[cfg(target_os = "linux")]
        {
            use gtk::prelude::*;
            let parent = main_window(&handle).and_then(|w| w.gtk_window().ok());
            let dialog = gtk::MessageDialog::new(
                parent.as_ref(),
                gtk::DialogFlags::MODAL,
                gtk::MessageType::Info,
                gtk::ButtonsType::Ok,
                &text,
            );
            dialog.set_title(&title);
            dialog.run();
            dialog.close();
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = &handle;
            rfd::MessageDialog::new()
                .set_level(rfd::MessageLevel::Info)
                .set_title(&title)
                .set_description(&text)
                .set_buttons(rfd::MessageButtons::Ok)
                .show();
        }
    });
}

// ---------------------------------------------------------------- oynalar

fn init_script(app: &AppHandle) -> String {
    let cfg = serde_json::json!({
        "platform": net::platform_key(),
        "version": app.package_info().version.to_string(),
    });
    format!("window.__EDUX_DESKTOP__ = {cfg};\n{INJECT}")
}

fn build_window(app: &AppHandle, label: &str, url: WebviewUrl, title: &str) -> tauri::Result<WebviewWindow> {
    let nav_app = app.clone();
    let dl_app = app.clone();
    let perm_app = app.clone();
    let new_app = app.clone();
    let is_main = label == "main";
    let mut builder = WebviewWindowBuilder::new(app, label, url)
        .title(title)
        .inner_size(if is_main { 1280.0 } else { 1000.0 }, if is_main { 820.0 } else { 760.0 })
        .min_inner_size(820.0, 560.0)
        .initialization_script(init_script(app))
        .on_navigation(move |url| navigation_allowed(&nav_app, url))
        .on_new_window(move |url, _features| {
            let app = new_app.clone();
            let server = app.state::<AppState>().server();
            if matches!(url.scheme(), "http" | "https") && server.as_deref().map(|s| same_site(&url, s)).unwrap_or(false) {
                std::thread::spawn(move || {
                    let _ = open_popup(&app, url);
                });
            } else if url.scheme() != "about" {
                open_external_url(&app, url.as_str());
            }
            NewWindowResponse::Deny
        })
        .on_download(move |_webview, event| handle_download(&dl_app, event))
        .on_permission_request(move |webview, kind| match kind {
            PermissionKind::Camera | PermissionKind::Microphone | PermissionKind::Notifications
                if trusted(&perm_app, &webview) =>
            {
                PermissionResponse::Allow
            }
            _ => PermissionResponse::Default,
        })
        .on_document_title_changed(|window, title| {
            let title = title.trim();
            let _ = window.set_title(if title.is_empty() { "EduX" } else { title });
        });
    if is_main {
        builder = builder.visible(true);
    } else {
        builder = builder.center();
    }
    let window = builder.build()?;
    let zoom = app.state::<AppState>().settings().zoom;
    if (zoom - 1.0).abs() > f64::EPSILON {
        let _ = window.set_zoom(zoom);
    }
    #[cfg(target_os = "linux")]
    {
        // WebKitGTK da getUserMedia (kamera/mikrofon) sukut bo'yicha o'chiq.
        let _ = window.with_webview(|wv| {
            use webkit2gtk::{SettingsExt, WebViewExt};
            if let Some(s) = wv.inner().settings() {
                s.set_enable_media_stream(true);
                s.set_enable_mediasource(true);
                s.set_enable_encrypted_media(true);
            }
        });
    }
    Ok(window)
}

/// O'sha server sahifasini alohida oynada ochish (target=_blank, window.open).
fn open_popup(app: &AppHandle, url: Url) -> tauri::Result<WebviewWindow> {
    let n = app.state::<AppState>().popup_seq.fetch_add(1, Ordering::SeqCst) + 1;
    build_window(app, &format!("popup-{n}"), WebviewUrl::External(url), "EduX")
}

fn navigation_allowed(app: &AppHandle, url: &Url) -> bool {
    match url.scheme() {
        "tauri" | "about" | "blob" | "data" => true,
        "http" | "https" => {
            if is_local(url) {
                return true;
            }
            if let Some(server) = app.state::<AppState>().server() {
                if same_site(url, &server) {
                    return true;
                }
            }
            // Iframe ichidagi video/vidjetlar (faqat embed manzillari).
            let host = url.host_str().unwrap_or("");
            if EMBED_HOSTS.contains(&host) && (url.path().contains("embed") || host == "oauth.telegram.org") {
                return true;
            }
            open_external_url(app, url.as_str());
            false
        }
        _ => {
            open_external_url(app, url.as_str());
            false
        }
    }
}

fn handle_download(app: &AppHandle, event: DownloadEvent<'_>) -> bool {
    match event {
        DownloadEvent::Requested { url, destination } => {
            let name = destination
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .filter(|n| !n.is_empty())
                .or_else(|| url.path_segments().and_then(|mut s| s.next_back().map(str::to_string)).filter(|n| !n.is_empty()))
                .unwrap_or_else(|| "fayl".into());
            match save_dialog_on_main(app, &name) {
                Some(path) => {
                    *destination = path;
                    true
                }
                None => false,
            }
        }
        DownloadEvent::Finished { path, success, .. } => {
            let lang = app.state::<AppState>().lang();
            if success {
                let name = path.as_ref().and_then(|p| p.file_name()).map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                notify(app, &t(&lang, "save.done"), &name);
            } else {
                notify(app, &t(&lang, "save.failed"), "");
            }
            true
        }
        _ => true,
    }
}

// ---------------------------------------------------------------- server bilan ishlash

/// Serverni tekshirib, asosiy oynani unga (yoki kutilayotgan deep link yo'liga) o'tkazadi.
fn open_server(app: &AppHandle, window: &WebviewWindow) -> Result<(), String> {
    let state = app.state::<AppState>();
    let Some(server) = state.server() else { return Err("setup".into()) };
    match net::check_options(&server, 12) {
        Ok(_) => {
            let path = state.pending_path.lock().unwrap().take().unwrap_or_else(|| "/login".into());
            let url = Url::parse(&format!("{server}{path}")).map_err(|e| e.to_string())?;
            *state.last_error.lock().unwrap() = None;
            window.navigate(url).map_err(|e| e.to_string())
        }
        Err(net::CheckError::Unreachable(e)) => {
            *state.last_error.lock().unwrap() = Some(e);
            Err("unreachable".into())
        }
        Err(net::CheckError::NotEdux(e)) => {
            *state.last_error.lock().unwrap() = Some(e);
            Err("not_edux".into())
        }
    }
}

fn show_local(window: &WebviewWindow, hash: &str) {
    let _ = window.navigate(local_url(hash));
}

fn handle_deep_link(app: &AppHandle, url: &Url) {
    if url.scheme() != "edux" {
        return;
    }
    // edux://open?path=/students/5  yoki  edux://students/5
    let path = if url.host_str() == Some("open") || url.host_str().is_none() {
        url.query_pairs().find(|(k, _)| k == "path").map(|(_, v)| v.to_string()).unwrap_or_else(|| "/".into())
    } else {
        format!("/{}{}", url.host_str().unwrap_or(""), url.path())
    };
    if !path.starts_with('/') || path.starts_with("//") || path.contains('\\') {
        return;
    }
    let state = app.state::<AppState>();
    let Some(window) = main_window(app) else {
        *state.pending_path.lock().unwrap() = Some(path);
        return;
    };
    let _ = window.unminimize();
    let _ = window.show();
    let _ = window.set_focus();
    let on_server = window.url().ok().zip(state.server()).map(|(u, s)| same_site(&u, &s)).unwrap_or(false);
    if on_server {
        let _ = window.eval(format!("window.__eduxNavigate && window.__eduxNavigate({})", serde_json::json!(path)));
    } else {
        *state.pending_path.lock().unwrap() = Some(path);
        if state.server().is_some() {
            show_local(&window, "boot");
        }
    }
}

fn check_updates(app: &AppHandle, manual: bool) {
    let state = app.state::<AppState>();
    let lang = state.lang();
    let Some(server) = state.server() else { return };
    let current = app.package_info().version.to_string();
    let found = net::fetch_apps(&server).and_then(|apps| net::newer_desktop(&apps, &current));
    match found {
        Some(info) => {
            let mut notified = state.notified_version.lock().unwrap();
            if !manual && notified.as_deref() == Some(info.version.as_str()) {
                return;
            }
            *notified = Some(info.version.clone());
            drop(notified);
            let payload = serde_json::json!({
                "version": info.version, "url": info.url, "notes": info.notes,
                "title": t(&lang, "update.title"), "download": t(&lang, "update.download"), "later": t(&lang, "update.later"),
            });
            if let Some(w) = main_window(app) {
                let _ = w.eval(format!("window.__eduxUpdate && window.__eduxUpdate({payload})"));
            }
        }
        None if manual => message_on_main(app, t(&lang, "menu.updates"), t(&lang, "update.none")),
        None => {}
    }
}

fn show_about(app: &AppHandle) {
    let app = app.clone();
    std::thread::spawn(move || {
        let state = app.state::<AppState>();
        let lang = state.lang();
        let server = state.server();
        let server_version = server.as_deref().and_then(net::server_version).unwrap_or_else(|| t(&lang, "about.unknown"));
        let text = format!(
            "{}: {}\n{}: {}\n{}: {}\n\n© EduX",
            t(&lang, "about.app"),
            app.package_info().version,
            t(&lang, "about.server"),
            server.unwrap_or_else(|| "—".into()),
            t(&lang, "about.server_version"),
            server_version,
        );
        message_on_main(&app, t(&lang, "about.title"), text);
    });
}

// ---------------------------------------------------------------- menyu

fn build_menu(app: &AppHandle) -> tauri::Result<Menu<tauri::Wry>> {
    let lang = app.state::<AppState>().lang();
    let l = |k: &str| t(&lang, k);
    let item = |id: &str, key: &str, accel: Option<&str>| {
        let mut b = MenuItemBuilder::with_id(id, l(key));
        if let Some(a) = accel {
            b = b.accelerator(a);
        }
        b.build(app)
    };

    // Kengaytirilgan: o'z serveriga o'rnatgan mijozlar uchun server manzilini o'zgartirish.
    let advanced = SubmenuBuilder::new(app, l("menu.advanced"))
        .item(&item("server", "menu.server", None)?)
        .build()?;
    let app_menu = SubmenuBuilder::new(app, l("menu.app"))
        .item(&item("about", "menu.about", None)?)
        .separator()
        .item(&advanced)
        .item(&item("browser", "menu.browser", None)?)
        .item(&item("updates", "menu.updates", None)?)
        .separator();
    #[cfg(target_os = "macos")]
    let app_menu = app_menu
        .item(&PredefinedMenuItem::hide(app, None)?)
        .item(&PredefinedMenuItem::hide_others(app, None)?)
        .item(&PredefinedMenuItem::show_all(app, None)?)
        .separator();
    let app_menu = app_menu.item(&item("quit", "menu.quit", Some("CmdOrCtrl+Q"))?).build()?;

    // Tahrirlash menyusi macOS da nusxa olish/qo'yish tugmalari ishlashi uchun shart.
    let edit_menu = SubmenuBuilder::new(app, l("menu.edit"))
        .item(&PredefinedMenuItem::undo(app, Some(&l("edit.undo")))?)
        .item(&PredefinedMenuItem::redo(app, Some(&l("edit.redo")))?)
        .separator()
        .item(&PredefinedMenuItem::cut(app, Some(&l("edit.cut")))?)
        .item(&PredefinedMenuItem::copy(app, Some(&l("edit.copy")))?)
        .item(&PredefinedMenuItem::paste(app, Some(&l("edit.paste")))?)
        .item(&PredefinedMenuItem::select_all(app, Some(&l("edit.select_all")))?)
        .build()?;

    let back_accel = if cfg!(target_os = "macos") { "Cmd+[" } else { "Alt+Left" };
    let forward_accel = if cfg!(target_os = "macos") { "Cmd+]" } else { "Alt+Right" };
    let view_menu = SubmenuBuilder::new(app, l("menu.view"))
        .item(&item("reload", "menu.reload", Some("CmdOrCtrl+R"))?)
        .item(&item("back", "menu.back", Some(back_accel))?)
        .item(&item("forward", "menu.forward", Some(forward_accel))?)
        .separator()
        .item(&item("zoom_in", "menu.zoom_in", Some("CmdOrCtrl+="))?)
        .item(&item("zoom_out", "menu.zoom_out", Some("CmdOrCtrl+-"))?)
        .item(&item("zoom_reset", "menu.zoom_reset", Some("CmdOrCtrl+0"))?)
        .separator()
        .item(&item("fullscreen", "menu.fullscreen", Some(if cfg!(target_os = "macos") { "Ctrl+Cmd+F" } else { "F11" }))?)
        .build()?;

    let help_menu = SubmenuBuilder::new(app, l("menu.help"))
        .item(&item("help", "menu.help_center", Some("F1"))?)
        .item(&item("about2", "menu.about", None)?)
        .build()?;

    MenuBuilder::new(app).items(&[&app_menu, &edit_menu, &view_menu, &help_menu]).build()
}

fn refresh_menu(app: &AppHandle) {
    if let Ok(menu) = build_menu(app) {
        let _ = app.set_menu(menu);
    }
}

fn set_zoom(app: &AppHandle, zoom: f64) {
    let zoom = (zoom * 10.0).round() / 10.0;
    let zoom = zoom.clamp(0.5, 2.5);
    app.state::<AppState>().update(|s| s.zoom = zoom);
    for w in app.webview_windows().values() {
        let _ = w.set_zoom(zoom);
    }
}

fn on_menu(app: &AppHandle, id: &str) {
    let state = app.state::<AppState>();
    match id {
        "about" | "about2" => show_about(app),
        "quit" => app.exit(0),
        "server" => {
            if let Some(w) = main_window(app) {
                let _ = w.set_focus();
                show_local(&w, "settings");
            }
        }
        "browser" => {
            let current = target_window(app).and_then(|w| w.url().ok()).filter(|u| !is_local(u)).map(|u| u.to_string());
            if let Some(url) = current.or_else(|| state.server()) {
                open_external_url(app, &url);
            }
        }
        "updates" => {
            let app = app.clone();
            std::thread::spawn(move || check_updates(&app, true));
        }
        "help" => {
            if let (Some(w), Some(server)) = (main_window(app), state.server()) {
                let on_server = w.url().map(|u| same_site(&u, &server)).unwrap_or(false);
                if on_server {
                    let _ = w.eval("window.__eduxNavigate && window.__eduxNavigate('/help')");
                } else if let Ok(url) = Url::parse(&format!("{server}/help")) {
                    *state.pending_path.lock().unwrap() = Some(url.path().to_string());
                    show_local(&w, "boot");
                }
            }
        }
        "reload" => {
            let Some(w) = target_window(app) else { return };
            let app = app.clone();
            std::thread::spawn(move || {
                let on_local = w.url().map(|u| is_local(&u)).unwrap_or(true);
                if on_local || w.label() != "main" {
                    let _ = w.eval("location.reload()");
                    return;
                }
                let server = app.state::<AppState>().server();
                match server.as_deref().map(|s| net::check_options(s, 8)) {
                    Some(Ok(_)) => {
                        let _ = w.eval("location.reload()");
                    }
                    _ => show_local(&w, "boot"),
                }
            });
        }
        "back" => {
            if let Some(w) = target_window(app) {
                let _ = w.eval("history.back()");
            }
        }
        "forward" => {
            if let Some(w) = target_window(app) {
                let _ = w.eval("history.forward()");
            }
        }
        "zoom_in" => set_zoom(app, state.settings().zoom + 0.1),
        "zoom_out" => set_zoom(app, state.settings().zoom - 0.1),
        "zoom_reset" => set_zoom(app, 1.0),
        "fullscreen" => {
            if let Some(w) = target_window(app) {
                let full = w.is_fullscreen().unwrap_or(false);
                let _ = w.set_fullscreen(!full);
            }
        }
        _ => {}
    }
}

// ---------------------------------------------------------------- buyruqlar (IPC)

#[derive(serde::Serialize)]
struct SettingsView {
    server: Option<String>,
    /// Foydalanuvchi o'z serverini tanlaganmi (aks holda sukut server).
    custom: bool,
    default_server: String,
    lang: Option<String>,
    effective_lang: String,
    system_lang: String,
    version: String,
    os: &'static str,
    last_error: Option<String>,
}

#[tauri::command]
fn get_settings(app: AppHandle) -> SettingsView {
    let state = app.state::<AppState>();
    let s = state.settings();
    let last_error = state.last_error.lock().unwrap().clone();
    SettingsView {
        server: state.server(),
        custom: s.server.is_some(),
        default_server: settings::default_server(),
        lang: s.lang.clone(),
        effective_lang: s.effective_lang(),
        system_lang: settings::system_lang(),
        version: app.package_info().version.to_string(),
        os: net::platform_key(),
        last_error,
    }
}

#[derive(serde::Serialize)]
struct CheckResult {
    url: String,
}

/// Manzilni normallashtirib, GET <server>/api/v1/auth/options bilan tekshiradi.
#[tauri::command(async)]
fn check_server(input: String) -> Result<CheckResult, String> {
    let url = settings::normalize_server(&input, None).map_err(|_| "invalid".to_string())?;
    match net::check_options(&url, 12) {
        Ok(_) => Ok(CheckResult { url }),
        Err(net::CheckError::Unreachable(_)) => Err("unreachable".into()),
        Err(net::CheckError::NotEdux(_)) => Err("not_edux".into()),
    }
}

#[tauri::command]
fn save_settings(
    app: AppHandle,
    webview: Webview,
    server: Option<String>,
    lang: Option<String>,
    reset: Option<bool>,
) -> Result<(), String> {
    if !webview.url().map(|u| is_local(&u)).unwrap_or(false) {
        return Err("forbidden".into());
    }
    let server = match server {
        Some(s) if !s.trim().is_empty() => Some(settings::normalize_server(&s, None).map_err(|_| "invalid".to_string())?),
        _ => None,
    };
    let lang = lang.filter(|l| matches!(l.as_str(), "uz" | "ru" | "en"));
    let state = app.state::<AppState>();
    let reset = reset.unwrap_or(false);
    state.update(|s| {
        if reset {
            s.server = None;
        } else if let Some(server) = server.clone() {
            // Sukut server bilan bir xil bo'lsa — saqlanmaydi (keyingi yig'ishdagi sukut serverga ergashadi).
            s.server = if server == settings::default_server() { None } else { Some(server) };
        }
        s.lang = lang;
    });
    *state.notified_version.lock().unwrap() = None;
    refresh_menu(&app);
    Ok(())
}

#[derive(serde::Serialize)]
struct BootResult {
    setup: bool,
}

/// Mahalliy sahifa ochilganda: server tanlangan bo'lsa — tekshirib, unga o'tadi.
#[tauri::command(async)]
fn boot(app: AppHandle, window: WebviewWindow) -> Result<BootResult, String> {
    open_server(&app, &window).map(|_| BootResult { setup: false })
}

#[tauri::command]
fn open_external(app: AppHandle, webview: Webview, url: String) -> Result<(), String> {
    if !trusted(&app, &webview) {
        return Err("forbidden".into());
    }
    open_external_url(&app, &url);
    Ok(())
}

#[tauri::command(async)]
fn open_window(app: AppHandle, webview: Webview, url: String) -> Result<(), String> {
    if !trusted(&app, &webview) {
        return Err("forbidden".into());
    }
    let parsed = Url::parse(&url).map_err(|e| e.to_string())?;
    let server = app.state::<AppState>().server().ok_or("no server")?;
    if !same_site(&parsed, &server) {
        open_external_url(&app, &url);
        return Ok(());
    }
    open_popup(&app, parsed).map(|_| ()).map_err(|e| e.to_string())
}

/// Veb ilovadagi blob/data fayllar: baytlar to'g'ridan-to'g'ri keladi, nom — x-edux-filename sarlavhasida.
#[tauri::command(async)]
fn save_file(app: AppHandle, webview: Webview, request: tauri::ipc::Request<'_>) -> Result<bool, String> {
    if !trusted(&app, &webview) {
        return Err("forbidden".into());
    }
    let tauri::ipc::InvokeBody::Raw(bytes) = request.body() else {
        return Err("raw body expected".into());
    };
    let name = request
        .headers()
        .get("x-edux-filename")
        .and_then(|v| v.to_str().ok())
        .map(|v| url::form_urlencoded::parse(format!("n={}", v.replace('+', "%2B")).as_bytes()).next().map(|(_, n)| n.to_string()).unwrap_or_default())
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| "fayl".into());
    let lang = app.state::<AppState>().lang();
    let Some(path) = save_dialog(&app, &name) else { return Ok(false) };
    match std::fs::write(&path, bytes) {
        Ok(_) => {
            notify(&app, &t(&lang, "save.done"), &path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default());
            Ok(true)
        }
        Err(e) => {
            notify(&app, &t(&lang, "save.failed"), &e.to_string());
            Err(e.to_string())
        }
    }
}

#[tauri::command]
fn notify_cmd(app: AppHandle, webview: Webview, title: String, body: Option<String>) -> Result<(), String> {
    if !trusted(&app, &webview) {
        return Err("forbidden".into());
    }
    notify(&app, &title, body.as_deref().unwrap_or(""));
    Ok(())
}

#[tauri::command]
fn print_page(app: AppHandle, webview: Webview, window: WebviewWindow) -> Result<(), String> {
    if !trusted(&app, &webview) {
        return Err("forbidden".into());
    }
    window.print().map_err(|e| e.to_string())
}

#[tauri::command]
fn quit(app: AppHandle) {
    app.exit(0);
}

// ---------------------------------------------------------------- ishga tushirish

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Bitta nusxa: ikkinchi marta ochilsa (yoki edux:// havola bosilsa) — mavjud oyna oldinga chiqadi.
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            if let Some(w) = main_window(app) {
                let _ = w.unminimize();
                let _ = w.show();
                let _ = w.set_focus();
            }
        }))
        .plugin(tauri_plugin_deep_link::init())
        .plugin(
            tauri_plugin_window_state::Builder::default()
                .with_filter(|label| label == "main")
                .build(),
        )
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_settings,
            check_server,
            save_settings,
            boot,
            open_external,
            open_window,
            save_file,
            notify_cmd,
            print_page,
            quit
        ])
        .on_menu_event(|app, event| on_menu(app, event.id().as_ref()))
        .setup(|app| {
            let handle = app.handle().clone();
            let path = app.path().app_config_dir()?.join("settings.json");
            let settings = Settings::load(&path);
            app.manage(AppState {
                settings: Mutex::new(settings),
                path,
                pending_path: Mutex::new(None),
                last_error: Mutex::new(None),
                notified_version: Mutex::new(None),
                popup_seq: AtomicU32::new(0),
            });

            // edux:// sxemasi (Linux AppImage va Windows'da ishga tushganda ham ro'yxatdan o'tadi).
            #[cfg(any(target_os = "linux", windows))]
            let _ = app.deep_link().register_all();
            if let Ok(Some(urls)) = app.deep_link().get_current() {
                for url in urls {
                    handle_deep_link(&handle, &url);
                }
            }
            let dl = handle.clone();
            app.deep_link().on_open_url(move |event| {
                for url in event.urls() {
                    handle_deep_link(&dl, &url);
                }
            });

            refresh_menu(&handle);
            build_window(&handle, "main", WebviewUrl::App("index.html".into()), "EduX")?;

            // Yangilanishlar: ishga tushganda (biroz kutib) va har 6 soatda.
            let upd = handle.clone();
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_secs(20));
                loop {
                    check_updates(&upd, false);
                    std::thread::sleep(UPDATE_INTERVAL);
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("EduX ilovasini ishga tushirib bo'lmadi");
}
