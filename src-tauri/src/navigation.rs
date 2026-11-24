//! Navigation policy for the main webview: only Spotify (and the app's own
//! bundled pages) may load in-app; everything else goes to the system browser.

/// Returns true when a navigation to `scheme://host` may stay inside the app.
///
/// Allowed: `https` on `spotify.com` and its subdomains (login lives on
/// `accounts.spotify.com`), plus the app's own local pages (`tauri` scheme,
/// `tauri.localhost`, and `localhost` for `tauri dev`).
pub fn is_allowed(scheme: &str, host: Option<&str>) -> bool {
    match (scheme, host) {
        ("tauri", _) => true,
        ("http" | "https", Some("tauri.localhost" | "localhost")) => true,
        ("https", Some(host)) => host == "spotify.com" || host.ends_with(".spotify.com"),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::is_allowed;

    #[test]
    fn allows_spotify_origins() {
        assert!(is_allowed("https", Some("open.spotify.com")));
        assert!(is_allowed("https", Some("accounts.spotify.com")));
        assert!(is_allowed("https", Some("spotify.com")));
    }

    #[test]
    fn allows_local_app_pages() {
        assert!(is_allowed("tauri", Some("localhost")));
        assert!(is_allowed("http", Some("tauri.localhost")));
        assert!(is_allowed("http", Some("localhost")));
    }

    #[test]
    fn rejects_lookalike_and_foreign_hosts() {
        assert!(!is_allowed("https", Some("evilspotify.com")));
        assert!(!is_allowed("https", Some("spotify.com.evil.io")));
        assert!(!is_allowed("https", Some("example.com")));
    }

    #[test]
    fn rejects_insecure_and_other_schemes() {
        assert!(!is_allowed("http", Some("open.spotify.com")));
        assert!(!is_allowed("file", None));
        assert!(!is_allowed("javascript", None));
        assert!(!is_allowed("https", None));
    }
}
