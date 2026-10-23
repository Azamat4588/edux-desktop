// Server bilan HTTP: manzilni tekshirish (auth/options), versiya va yangilanishlar (public/apps).
use serde_json::Value;
use std::time::Duration;

fn agent(timeout: u64) -> ureq::Agent {
    ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(timeout))
        .user_agent(&format!("EduX-Desktop/{} ({})", env!("CARGO_PKG_VERSION"), std::env::consts::OS))
        .build()
}

#[derive(Debug)]
pub enum CheckError {
    /// Server javob bermadi (internet yo'q, DNS, vaqt tugadi).
    Unreachable(String),
    /// Javob keldi, lekin bu EduX emas (404, HTML, noto'g'ri JSON).
    NotEdux(String),
}

/// GET <server>/api/v1/auth/options — EduX serveri ekanini tekshiradi, JSON qaytaradi.
pub fn check_options(server: &str, timeout: u64) -> Result<Value, CheckError> {
    let url = format!("{server}/api/v1/auth/options");
    match agent(timeout).get(&url).set("Accept", "application/json").call() {
        Ok(resp) => {
            let text = resp.into_string().map_err(|e| CheckError::Unreachable(e.to_string()))?;
            match serde_json::from_str::<Value>(&text) {
                Ok(v) if v.is_object() && (v.get("mode").is_some() || v.get("password_reset").is_some()) => Ok(v),
                _ => Err(CheckError::NotEdux("not edux json".into())),
            }
        }
        Err(ureq::Error::Status(code, _)) => Err(CheckError::NotEdux(format!("HTTP {code}"))),
        Err(e) => Err(CheckError::Unreachable(e.to_string())),
    }
}

/// GET <server>/api/v1/public/apps — mavjud bo'lmasa (404, xato) None.
pub fn fetch_apps(server: &str) -> Option<Value> {
    let url = format!("{server}/api/v1/public/apps");
    let resp = agent(15).get(&url).set("Accept", "application/json").call().ok()?;
    serde_json::from_str::<Value>(&resp.into_string().ok()?).ok().filter(|v| v.is_object())
}

/// GET <server>/api/v1/site → `domain` (tashkilot subdomenlari uchun asosiy domen).
/// Some(Some(d)) — domen bor, Some(None) — server subdomen ishlatmaydi, None — so'rab bo'lmadi.
pub fn fetch_site_domain(server: &str) -> Option<Option<String>> {
    let url = format!("{server}/api/v1/site");
    let resp = agent(8).get(&url).set("Accept", "application/json").call().ok()?;
    let value = serde_json::from_str::<Value>(&resp.into_string().ok()?).ok()?;
    site_domain_of(&value)
}

fn site_domain_of(value: &Value) -> Option<Option<String>> {
    let obj = value.as_object()?;
    if !obj.contains_key("domain") {
        return None; // eski server — domen taxmin qilinadi
    }
    Some(obj.get("domain").and_then(Value::as_str).map(str::trim).filter(|d| !d.is_empty()).map(str::to_lowercase))
}

/// Server versiyasi: options.version yoki apps.server.version / apps.version (qaysi biri bo'lsa).
pub fn server_version(server: &str) -> Option<String> {
    let pick = |v: &Value| -> Option<String> {
        for path in [&["version"][..], &["server_version"][..], &["server", "version"][..], &["app", "version"][..]] {
            let mut cur = v;
            let mut ok = true;
            for key in path {
                match cur.get(*key) {
                    Some(next) => cur = next,
                    None => {
                        ok = false;
                        break;
                    }
                }
            }
            if ok {
                if let Some(s) = cur.as_str() {
                    return Some(s.to_string());
                }
            }
        }
        None
    };
    if let Ok(v) = check_options(server, 8) {
        if let Some(s) = pick(&v) {
            return Some(s);
        }
    }
    fetch_apps(server).as_ref().and_then(pick)
}

pub fn platform_key() -> &'static str {
    if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        "linux"
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct UpdateInfo {
    pub version: String,
    pub url: String,
    pub notes: String,
}

/// apps javobidan shu OS uchun yangiroq versiyani topadi.
pub fn newer_desktop(apps: &Value, current: &str) -> Option<UpdateInfo> {
    let entry = apps.get("desktop")?.get(platform_key())?;
    let version = entry.get("version")?.as_str()?.trim().to_string();
    let url = entry.get("url")?.as_str()?.trim().to_string();
    if url.is_empty() || !(url.starts_with("https://") || url.starts_with("http://")) {
        return None;
    }
    if !is_newer(&version, current) {
        return None;
    }
    let notes = entry.get("notes").and_then(Value::as_str).unwrap_or("").to_string();
    Some(UpdateInfo { version, url, notes })
}

/// Serverdagi doimiy yuklab olish havolasi (docs/apps.md): `<server>/download/windows|macos|linux`.
/// Server faylni o'zi beradi yoki tashqi havolaga (GitHub relizi) yo'naltiradi — havola hech qachon o'zgarmaydi.
/// Linux'da .deb dan o'rnatilgan ilova (AppImage emas) — `/download/linux/deb`.
pub fn download_url(server: &str) -> String {
    download_url_for(server, platform_key(), std::env::var_os("APPIMAGE").is_some())
}

fn download_url_for(server: &str, platform: &str, appimage: bool) -> String {
    let base = server.trim_end_matches('/');
    if platform == "linux" && !appimage {
        format!("{base}/download/linux/deb")
    } else {
        format!("{base}/download/{platform}")
    }
}

fn parts(v: &str) -> Vec<u64> {
    v.trim_start_matches(['v', 'V'])
        .split(['-', '+'])
        .next()
        .unwrap_or("")
        .split('.')
        .map(|p| p.trim().parse::<u64>().unwrap_or(0))
        .collect()
}

/// 1.2.10 > 1.2.9; "v" prefiksi va -beta/+build qismi hisobga olinmaydi.
pub fn is_newer(candidate: &str, current: &str) -> bool {
    let (a, b) = (parts(candidate), parts(current));
    let n = a.len().max(b.len()).max(3);
    for i in 0..n {
        let (x, y) = (a.get(i).copied().unwrap_or(0), b.get(i).copied().unwrap_or(0));
        if x != y {
            return x > y;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_compare() {
        assert!(is_newer("1.0.1", "1.0.0"));
        assert!(is_newer("v1.10.0", "1.9.9"));
        assert!(!is_newer("1.0.0", "1.0.0"));
        assert!(!is_newer("0.9", "1.0.0"));
        assert!(!is_newer("garbage", "1.0.0"));
    }

    #[test]
    fn picks_platform_entry() {
        let apps = serde_json::json!({ "desktop": {
            "windows": {"version": "1.2.0", "url": "https://x/EduX-Setup-x64.exe", "notes": "n"},
            "macos": {"version": "1.2.0", "url": "https://x/EduX-macOS-universal.dmg"},
            "linux": {"version": "1.2.0", "url": "https://x/EduX-linux-x86_64.AppImage"}
        }});
        let u = newer_desktop(&apps, "1.0.0").unwrap();
        assert_eq!(u.version, "1.2.0");
        assert!(newer_desktop(&apps, "1.2.0").is_none());
        assert!(newer_desktop(&serde_json::json!({}), "1.0.0").is_none());
        // Platforma yashirilgan yoki havolasi yo'q (null) — xabar chiqmaydi.
        let hidden = serde_json::json!({ "desktop": { "windows": null, "macos": null, "linux": null } });
        assert!(newer_desktop(&hidden, "1.0.0").is_none());
    }

    #[test]
    fn site_domain_answer() {
        assert_eq!(site_domain_of(&serde_json::json!({"domain": "edux.uz", "kind": "combined"})), Some(Some("edux.uz".into())));
        assert_eq!(site_domain_of(&serde_json::json!({"domain": null})), Some(None));
        assert_eq!(site_domain_of(&serde_json::json!({"domain": " "})), Some(None));
        assert_eq!(site_domain_of(&serde_json::json!({"kind": "combined"})), None);
        assert_eq!(site_domain_of(&serde_json::json!([])), None);
    }

    #[test]
    fn permanent_download_links() {
        assert_eq!(download_url_for("https://edux.uz/", "windows", false), "https://edux.uz/download/windows");
        assert_eq!(download_url_for("https://edux.uz", "macos", false), "https://edux.uz/download/macos");
        assert_eq!(download_url_for("https://edux.uz", "linux", true), "https://edux.uz/download/linux");
        assert_eq!(download_url_for("http://10.0.0.5:8000", "linux", false), "http://10.0.0.5:8000/download/linux/deb");
    }
}
