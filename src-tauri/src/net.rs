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
    }
}
