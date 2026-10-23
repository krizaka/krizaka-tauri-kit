//! Deep links: `yomizaka://app/bibliotheque/bail?tags=2026` or `https://www.yomizaka.com/app/bibliotheque/bail` opens the
//! same screen as the web (the URL is the state). A link becomes a **route inside the app** (`/app/bibliotheque/bail?tags=2026`)
//! or is refused; it is never a place the window navigates to.
//!
//! The custom scheme reads like Expo Router's: `yomizaka://app/x` is the route `/app/x` (the "host" is the first segment).
//! A web link is accepted only on the product's own hosts. The route keeps its query, drops its fragment, and may only start
//! with one of the product's prefixes when it gives some.

use url::Url;

/// The longest link the app accepts; anything longer is refused rather than cut.
// @fixed: the length of a link a person can follow, not a setting
pub const MAX_LINK: usize = 2048;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkRules {
    /// The product's scheme, lowercase (`yomizaka`).
    pub scheme: String,
    /// The product's web hosts whose links open the app (`www.yomizaka.com`, `dev.yomizaka.com`).
    pub hosts: Vec<String>,
    /// The routes a link may open (`/app`, `/verify`); empty means every route.
    pub prefixes: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LinkError {
    TooLong,
    Invalid,
    /// Another scheme, or a web link on a host that is not the product's.
    NotOurs,
    /// A route the product does not open from a link, or that could leave the app (`//host`, `..`, a backslash).
    Refused(String),
}

impl std::fmt::Display for LinkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinkError::TooLong => write!(f, "the link is too long"),
            LinkError::Invalid => write!(f, "not a valid link"),
            LinkError::NotOurs => write!(f, "not a link to this app"),
            LinkError::Refused(route) => write!(f, "this link does not open in the app: {route}"),
        }
    }
}

impl std::error::Error for LinkError {}

/// The route a link opens, or why it does not.
pub fn route(rules: &LinkRules, link: &str) -> Result<String, LinkError> {
    let link = link.trim();
    if link.len() > MAX_LINK {
        return Err(LinkError::TooLong);
    }
    if link.chars().any(|c| c.is_control() || c == '\\') {
        return Err(LinkError::Invalid);
    }
    let url = Url::parse(link).map_err(|_| LinkError::Invalid)?;
    if !url.username().is_empty() || url.password().is_some() {
        return Err(LinkError::Invalid);
    }
    let path = if url.scheme() == rules.scheme {
        match url.host_str() {
            Some(first) if !first.is_empty() => format!("/{first}{}", url.path()),
            _ => url.path().to_string(),
        }
    } else if url.scheme() == "https"
        && url.port().is_none()
        && url
            .host_str()
            .is_some_and(|h| rules.hosts.iter().any(|ours| ours.eq_ignore_ascii_case(h)))
    {
        url.path().to_string()
    } else {
        return Err(LinkError::NotOurs);
    };
    let path = match path.trim_end_matches('/') {
        "" => "/".to_string(),
        trimmed => trimmed.to_string(),
    };
    if !path.starts_with('/')
        || path.starts_with("//")
        || path
            .split('/')
            .any(|s| s == ".." || s == "." || s.eq_ignore_ascii_case("%2e%2e"))
    {
        return Err(LinkError::Refused(path));
    }
    let allowed = rules.prefixes.is_empty()
        || rules
            .prefixes
            .iter()
            .any(|p| path == *p || path.starts_with(&format!("{}/", p.trim_end_matches('/'))));
    if !allowed {
        return Err(LinkError::Refused(path));
    }
    Ok(match url.query() {
        Some(q) if !q.is_empty() => format!("{path}?{q}"),
        _ => path,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rules() -> LinkRules {
        LinkRules {
            scheme: "yomizaka".into(),
            hosts: vec!["www.yomizaka.com".into(), "dev.yomizaka.com".into()],
            prefixes: vec!["/app".into(), "/verify".into()],
        }
    }

    #[test]
    fn the_scheme_reads_like_expo_router_and_keeps_the_query() {
        assert_eq!(
            route(&rules(), "yomizaka://app/bibliotheque/bail?tags=2026").unwrap(),
            "/app/bibliotheque/bail?tags=2026"
        );
        assert_eq!(
            route(&rules(), "yomizaka:///app/rapports").unwrap(),
            "/app/rapports"
        );
        assert_eq!(route(&rules(), "yomizaka://app/").unwrap(), "/app");
        assert_eq!(
            route(&rules(), "yomizaka://verify/abc#top").unwrap(),
            "/verify/abc"
        );
    }

    #[test]
    fn a_web_link_opens_the_app_only_on_the_products_hosts() {
        assert_eq!(
            route(&rules(), "https://www.yomizaka.com/app/verifications/42").unwrap(),
            "/app/verifications/42"
        );
        assert_eq!(
            route(&rules(), "https://DEV.yomizaka.com/app").unwrap(),
            "/app"
        );
        for other in [
            "https://www.yomizaka.com.evil.example/app",
            "https://evil.example/app",
            "http://www.yomizaka.com/app",
            "https://www.yomizaka.com:8443/app",
            "orazaka://app/x",
            "javascript:alert(1)",
            "file:///etc/passwd",
        ] {
            assert_eq!(route(&rules(), other), Err(LinkError::NotOurs), "{other}");
        }
    }

    #[test]
    fn a_route_outside_the_prefixes_or_one_that_could_leave_the_app_is_refused() {
        for bad in [
            "yomizaka://admin/users",
            "yomizaka://application",
            "https://www.yomizaka.com/",
            "yomizaka:////evil.example/app",
        ] {
            assert!(
                matches!(route(&rules(), bad), Err(LinkError::Refused(_))),
                "{bad}"
            );
        }
        // Dot segments are resolved under the link's first segment: they can never climb out of it.
        assert_eq!(
            route(&rules(), "yomizaka://app/a/../../admin").unwrap(),
            "/app/admin"
        );
        assert_eq!(
            route(&rules(), "https://www.yomizaka.com/app/../admin"),
            Err(LinkError::Refused("/admin".into()))
        );
        for invalid in [
            "yomizaka://app/a\\b",
            "yomizaka://app/\u{0}",
            "https://u:p@www.yomizaka.com/app",
            "not a link",
        ] {
            assert_eq!(
                route(&rules(), invalid),
                Err(LinkError::Invalid),
                "{invalid}"
            );
        }
    }

    #[test]
    fn no_prefixes_means_every_route_and_a_long_link_is_refused_not_cut() {
        let open = LinkRules {
            prefixes: vec![],
            ..rules()
        };
        assert_eq!(route(&open, "yomizaka://tarifs").unwrap(), "/tarifs");
        assert_eq!(route(&open, "yomizaka://").unwrap(), "/");
        let long = format!("yomizaka://app/{}", "a".repeat(MAX_LINK));
        assert_eq!(route(&rules(), &long), Err(LinkError::TooLong));
    }
}
