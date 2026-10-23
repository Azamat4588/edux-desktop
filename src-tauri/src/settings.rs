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

/// Ikki bo'lakli ommaviy qo'shimchalar (co.uz, com.uz, org.uk ...): ular ostidagi saytlar bir-biriga begona.
fn is_public_second_level(label: &str, tld: &str) -> bool {
    tld.len() == 2 && matches!(label, "co" | "com" | "net" | "org" | "edu" | "gov" | "ac")
}

/// Saytning asosiy domeni: app.edux.uz → edux.uz, app.maktab.co.uz → maktab.co.uz; IP/localhost — o'zi.
pub fn site_domain(host: &str) -> String {
    if is_ip_or_local(host) {
        return host.to_string();
    }
    let parts: Vec<&str> = host.split('.').collect();
    let keep = if parts.len() >= 3 && is_public_second_level(parts[parts.len() - 2], parts[parts.len() - 1]) { 3 } else { 2 };
    if parts.len() <= keep {
        host.to_string()
    } else {
        parts[parts.len() - keep..].join(".")
    }
}

/// Manzil tanlangan server saytiga tegishlimi.
/// [domain] — serverning o'zi aytgan asosiy domen (GET /api/v1/site → `domain`):
///   `Some(Some(d))` — `d` va uning subdomenlari (tashkilot manzillari) ham ilova ichida;
///   `Some(None)` — server subdomen ishlatmaydi: faqat aynan shu manzil;
///   `None` — hali noma'lum (eski server): domen manzildan taxmin qilinadi.
pub fn same_site_in(url: &Url, server: &str, domain: Option<Option<&str>>) -> bool {
    let Ok(s) = Url::parse(server) else { return false };
    let (Some(host), Some(server_host)) = (url.host_str(), s.host_str()) else { return false };
    if host == server_host {
        return url.port_or_known_default() == s.port_or_known_default();
    }
    if is_ip_or_local(server_host) || url.scheme() != "https" {
        return false;
    }
    let domain = match domain {
        Some(Some(d)) if !d.trim().is_empty() => d.trim().trim_matches('.').to_lowercase(),
        Some(_) => return false,
        None => site_domain(server_host),
    };
    host == domain || host.ends_with(&format!(".{domain}"))
}

/// [same_site_in] domen ma'lumotisiz (taxmin bilan).
#[cfg(test)]
pub fn same_site(url: &Url, server: &str) -> bool {
    same_site_in(url, server, None)
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
        // Ommaviy qo'shimcha ostida qo'shni sayt begona.
        let co = "https://app.maktab.co.uz";
        assert!(same_site(&Url::parse("https://markaz.maktab.co.uz/").unwrap(), co));
        assert!(!same_site(&Url::parse("https://boshqa.co.uz/").unwrap(), co));
        assert_eq!(site_domain("app.maktab.co.uz"), "maktab.co.uz");
        assert_eq!(site_domain("edux.uz"), "edux.uz");
        // Server o'z domenini aytsa — faqat o'sha domen; subdomensiz server — faqat aynan manzil.
        let k = "https://edux.kuchkarov.uz";
        assert!(same_site_in(&Url::parse("https://markaz.edux.kuchkarov.uz/").unwrap(), k, Some(Some("edux.kuchkarov.uz"))));
        assert!(!same_site_in(&Url::parse("https://taxi.kuchkarov.uz/").unwrap(), k, Some(Some("edux.kuchkarov.uz"))));
        assert!(!same_site_in(&Url::parse("https://taxi.kuchkarov.uz/").unwrap(), k, Some(None)));
        assert!(same_site_in(&Url::parse("https://edux.kuchkarov.uz/login").unwrap(), k, Some(None)));
        let local = "http://127.0.0.1:8090";
        assert!(same_site(&Url::parse("http://127.0.0.1:8090/x").unwrap(), local));
        assert!(!same_site(&Url::parse("http://127.0.0.1:9000/x").unwrap(), local));
    }
}
