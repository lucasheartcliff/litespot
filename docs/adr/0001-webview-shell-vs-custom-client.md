# 0001 — Webview shell around open.spotify.com instead of a custom client

Issue: #2 · Status: accepted

## Context
Litespot began as a custom Spotify client: a Svelte UI, a Rust host doing OAuth
PKCE and proxying the Web API, and the Web Playback SDK for audio. That needs a
registered Spotify app (Client ID), token storage and a UI to maintain, for a
project whose goal is a *lightweight* desktop wrapper.

## Decision
Litespot is a native Tauri window that loads `https://open.spotify.com`. Login
and playback are handled by the site itself. Rust only owns the window, the
startup-ready marker for benchmarks, and OS media keys, which it forwards by
clicking the site's player buttons through injected JavaScript.

## Consequences
- No Client ID, OAuth, keyring or Web API code; far less to build and ship.
- Media keys depend on Spotify's `data-testid` values and can break silently
  when the site changes (see `src-tauri/src/media.rs`).
- Embedded webviews may be restricted by Spotify (for example third-party
  logins such as Google); email/password login is the safe path.
- No control over the UI beyond injecting CSS/JS.
- The webview loads remote content, so navigation is restricted to `*.spotify.com`
  (`src-tauri/src/navigation.rs`); other links open in the system browser, no
  capability grants `remote` access, and SmartScreen stays enabled.
- Wrapping the site is not an officially supported use; Spotify may block it.
- Widevine support in the system webview remains the main technical risk.

## Alternatives considered
- **Custom client (Web API + Web Playback SDK):** stable and sanctioned, but
  needs app registration and a full UI. Preserved on branch
  `archive/custom-client`.
