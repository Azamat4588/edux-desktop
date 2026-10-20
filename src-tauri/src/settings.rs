// Ilova sozlamalari: server manzili, til va masshtab. Fayl: <config dir>/settings.json
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use url::Url;

/// Yig'ish paytida beriladigan sukut server (CI: vars.EDUX_SERVER), bo'lmasa edux.kuchkarov.uz.
pub fn default_server() -> String {
    match option_env!("EDUX_SERVER") {
        Some(s) if !s.trim().is_empty() => normalize_server(s, None).unwrap_or_else(|_| s.trim().to_string()),
        _ => "https://edux.kuchkarov.uz".to_string(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    /// Tanlangan server (origin, oxirida «/» yo'q). None — hali tanlanmagan (birinchi ishga tushish).
    #[serde(default)]
    pub server: Option<String>,
    /// "uz" | "ru" | "en"; None — tizim tili.
    #[serde(default)]
    pub lang: Option<String>,
    #[serde(default = "one")]
    pub zoom: f64,
}

fn one() -> f64 {
    1.0
}

impl Settings {
    pub fn load(path: &PathBuf) -> Settings {
        let mut s: Settings = std::fs::read_to_string(path)
            .ok()
            .and_then(|t| serde_json::from_str(&t).ok())
            .unwrap_or_default();
        if !(0.3..=3.0).contains(&s.zoom) {
            s.zoom = 1.0;
        }
        s
    }

    pub fn save(&self, path: &PathBuf) {
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        if let Ok(text) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(path, text);
        }
    }

    pub fn effective_lang(&self) -> String {
        match self.lang.as_deref() {
            Some(l @ ("uz" | "ru" | "en")) => l.to_string(),
            _ => system_lang(),
        }
    }
}

/// Tizim tili: ru*/en*/uz*; boshqalari — o'zbekcha.
pub fn system_lang() -> String {
    let locale = sys_locale::get_locale().unwrap_or_default().to_lowercase();
    if locale.starts_with("ru") {
        "ru".into()
    } else if locale.starts_with("en") {
        "en".into()
    } else {
        "uz".into()
    }
}

fn is_ip_or_local(host: &str) -> bool {
    host == "localhost" || host.parse::<std::net::IpAddr>().is_ok() || host.ends_with(".local") || host.ends_with(".test")
}

/// Kiritilgan manzilni origin ko'rinishiga keltiradi:
/// "markaz" → https://markaz.<asosiy domen>, "markaz.edux.uz" → https://markaz.edux.uz,
/// "127.0.0.1:8090" → http://127.0.0.1:8090. Yo'l, so'rov va «/» olib tashlanadi.
pub fn normalize_server(input: &str, base: Option<&str>) -> Result<String, String> {
    let raw = input.trim().trim_end_matches('/');
    if raw.is_empty() {
        return Err("empty".into());
    }
    let with_scheme = if raw.contains("://") {
        raw.to_string()
    } else {
        let host_part = raw.split(['/', ':']).next().unwrap_or("");
        if !host_part.contains('.') && host_part != "localhost" {
            // Faqat subdomen: asosiy domen sukut serverdan olinadi (app.edux.uz → edux.uz).
            let base = base.map(str::to_string).unwrap_or_else(default_server);
            let base_host = Url::parse(&base).ok().and_then(|u| u.host_str().map(str::to_string)).unwrap_or_else(|| "edux.uz".into());
            let domain = site_domain(&base_host);
            format!("https://{}.{}", raw.to_lowercase(), domain)
        } else if is_ip_or_local(host_part) {
            format!("http://{raw}")
        } else {
            format!("https://{raw}")
        }
    };
    let url = Url::parse(&with_scheme).map_err(|_| "invalid".to_string())?;
    if !matches!(url.scheme(), "http" | "https") || url.host_str().map(|h| h.is_empty()).unwrap_or(true) {
        return Err("invalid".into());
    }
    Ok(url.origin().ascii_serialization())
}

/// Saytning asosiy domeni: app.edux.uz → edux.uz; IP/localhost — o'zi.
pub fn site_domain(host: &str) -> String {
    if is_ip_or_local(host) {
        return host.to_string();
    }
    let parts: Vec<&str> = host.split('.').collect();
    if parts.len() <= 2 {
        host.to_string()
    } else {
        parts[parts.len() - 2..].join(".")
    }
}

/// Manzil tanlangan server saytiga tegishlimi (o'sha host yoki uning subdomenlari, o'sha sxema).
pub fn same_site(url: &Url, server: &str) -> bool {
    let Ok(s) = Url::parse(server) else { return false };
    let (Some(host), Some(server_host)) = (url.host_str(), s.host_str()) else { return false };
    if host == server_host {
        return url.port_or_known_default() == s.port_or_known_default();
    }
    if is_ip_or_local(server_host) {
        return false;
    }
    let domain = site_domain(server_host);
    url.scheme() == "https" && (host == domain || host.ends_with(&format!(".{domain}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_addresses() {
        let base = Some("https://app.edux.uz");
        assert_eq!(normalize_server("markaz", base).unwrap(), "https://markaz.edux.uz");
        assert_eq!(normalize_server("Markaz.edux.uz/login", base).unwrap(), "https://markaz.edux.uz");
        assert_eq!(normalize_server("127.0.0.1:8090", base).unwrap(), "http://127.0.0.1:8090");
        assert_eq!(normalize_server("http://localhost:8000/", base).unwrap(), "http://localhost:8000");
        assert_eq!(normalize_server("https://school.uz", base).unwrap(), "https://school.uz");
        assert!(normalize_server("  ", base).is_err());
        assert!(normalize_server("ftp://x.uz", base).is_err());
    }

    #[test]
    fn same_site_rules() {
        let s = "https://app.edux.uz";
        assert!(same_site(&Url::parse("https://app.edux.uz/students/5").unwrap(), s));
        assert!(same_site(&Url::parse("https://markaz.edux.uz/").unwrap(), s));
        assert!(!same_site(&Url::parse("https://t.me/edux").unwrap(), s));
        assert!(!same_site(&Url::parse("http://markaz.edux.uz/").unwrap(), s));
        let local = "http://127.0.0.1:8090";
        assert!(same_site(&Url::parse("http://127.0.0.1:8090/x").unwrap(), local));
        assert!(!same_site(&Url::parse("http://127.0.0.1:9000/x").unwrap(), local));
    }
}
