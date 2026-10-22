//! The navigation guard of a window that shows a hosted web app: it may never leave the app's origin.

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

#[cfg(test)]
mod tests {
    use super::*;

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
