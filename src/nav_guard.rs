//! The navigation guard: a window stays on the app's own pages (its local page, or the hosted web app it shows); a web link
//! elsewhere opens in the person's browser; anything else is refused.

use url::Url;

/// A hosted app must be HTTPS (plain http only for a loopback address), carry no credentials in its URL, and is reduced
/// to its origin.
pub fn normalize_origin(input: &str) -> Result<String, String> {
    let url = Url::parse(input.trim()).map_err(|_| "not a valid URL".to_string())?;
    if !url.username().is_empty() || url.password().is_some() {
        return Err("no credentials in the URL".into());
    }
    let host = url.host_str().ok_or("missing host")?;
    let loopback = matches!(host, "localhost" | "127.0.0.1" | "[::1]");
    match url.scheme() {
        "https" => {}
        "http" if loopback => {}
        _ => return Err("the app must be served over https".into()),
    }
    Ok(url.origin().ascii_serialization())
}

/// True when `target` is on the same origin as `allowed`. A look-alike host (`app.example.com.evil.example`) is not.
pub fn same_origin(allowed: &str, target: &Url) -> bool {
    Url::parse(allowed)
        .map(|a| a.origin() == target.origin())
        .unwrap_or(false)
}

/// What the window does with a navigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Navigation {
    /// One of the app's own places: the window goes there.
    Stay,
    /// A web page elsewhere: the window stays, the person's browser opens it.
    Browser,
    /// Anything else (`file:`, `javascript:`, `data:`, another app's scheme): nothing happens.
    Refuse,
}

/// The same place: scheme, host and port. Unlike [`same_origin`], it also matches the app's own local page
/// (`tauri://localhost`), whose origin the URL standard calls opaque.
pub fn same_place(allowed: &str, target: &Url) -> bool {
    let Ok(a) = Url::parse(allowed) else {
        return false;
    };
    a.scheme() == target.scheme()
        && a.host_str().is_some()
        && a.host_str() == target.host_str()
        && a.port_or_known_default() == target.port_or_known_default()
        && target.username().is_empty()
        && target.password().is_none()
}

/// Decides a navigation of a window whose own places are `allowed` (the local page, the hosted app's origin).
pub fn decide(allowed: &[String], target: &Url) -> Navigation {
    if allowed.iter().any(|a| same_place(a, target)) {
        return Navigation::Stay;
    }
    match target.scheme() {
        // The page's own blank frame and the object URLs it made (a PDF shown from memory).
        "about" if target.as_str() == "about:blank" => Navigation::Stay,
        "blob" => match Url::parse(target.path()) {
            Ok(inner) if allowed.iter().any(|a| same_place(a, &inner)) => Navigation::Stay,
            _ => Navigation::Refuse,
        },
        "https" | "http" | "mailto"
            if target.username().is_empty() && target.password().is_none() =>
        {
            Navigation::Browser
        }
        _ => Navigation::Refuse,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn places() -> Vec<String> {
        vec![
            "tauri://localhost".into(),
            "https://www.yomizaka.com".into(),
        ]
    }

    fn at(url: &str) -> Url {
        Url::parse(url).unwrap()
    }

    #[test]
    fn the_window_stays_on_its_own_places() {
        for own in [
            "tauri://localhost/index.html",
            "https://www.yomizaka.com/app/rapports",
            "about:blank",
            "blob:tauri://localhost/5f1c-77",
        ] {
            assert_eq!(decide(&places(), &at(own)), Navigation::Stay, "{own}");
        }
    }

    #[test]
    fn a_web_page_elsewhere_opens_in_the_browser_and_the_rest_is_refused() {
        for web in [
            "https://www.krizaka.com/privacy",
            "http://example.com/",
            "mailto:support@krizaka.com",
        ] {
            assert_eq!(decide(&places(), &at(web)), Navigation::Browser, "{web}");
        }
        for refused in [
            "file:///etc/passwd",
            "javascript:alert(1)",
            "data:text/html,<script>1</script>",
            "orazaka://app",
            "tauri://evil",
            "blob:https://evil.example/1",
            "https://user:pw@evil.example/",
        ] {
            assert_eq!(
                decide(&places(), &at(refused)),
                Navigation::Refuse,
                "{refused}"
            );
        }
        assert_eq!(
            decide(&places(), &at("https://www.yomizaka.com.evil.example/")),
            Navigation::Browser
        );
        assert_eq!(
            decide(&places(), &at("https://www.yomizaka.com:8443/")),
            Navigation::Browser
        );
    }

    #[test]
    fn an_origin_is_https_or_loopback_and_reduced_to_its_origin() {
        assert_eq!(
            normalize_origin("https://www.yomizaka.com/some/path?x=1").unwrap(),
            "https://www.yomizaka.com"
        );
        assert_eq!(
            normalize_origin("http://localhost:3000/").unwrap(),
            "http://localhost:3000"
        );
    }

    #[test]
    fn refuses_plain_http_credentials_and_other_schemes() {
        for bad in [
            "http://www.yomizaka.com",
            "https://user:pw@www.yomizaka.com",
            "file:///etc/passwd",
            "javascript:alert(1)",
            "nonsense",
        ] {
            assert!(normalize_origin(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn compares_origins_and_refuses_look_alikes() {
        let allowed = "https://www.yomizaka.com";
        assert!(same_origin(
            allowed,
            &Url::parse("https://www.yomizaka.com/login").unwrap()
        ));
        assert!(!same_origin(
            allowed,
            &Url::parse("https://evil.example/").unwrap()
        ));
        assert!(!same_origin(
            allowed,
            &Url::parse("https://www.yomizaka.com.evil.example/").unwrap()
        ));
        assert!(!same_origin(
            "not a url",
            &Url::parse("https://www.yomizaka.com/").unwrap()
        ));
    }
}
