const COMMANDS: &[&str] = &[
    "get_settings",
    "check_server",
    "save_settings",
    "boot",
    "open_external",
    "open_window",
    "save_file",
    "notify_cmd",
    "print_page",
    "quit",
];

fn main() {
    // Sukut bo'yicha server yig'ish paytida beriladi (CI: vars.EDUX_SERVER).
    println!("cargo:rerun-if-env-changed=EDUX_SERVER");
    // Ilova buyruqlari uchun ruxsatlar (allow-<buyruq>): server sahifasi faqat o'ziga keraklisini oladi.
    tauri_build::try_build(
        tauri_build::Attributes::new().app_manifest(tauri_build::AppManifest::new().commands(COMMANDS)),
    )
    .expect("tauri build");
}
