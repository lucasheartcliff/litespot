# Litespot

> **Status: archived.** Development stopped after benchmarking: Litespot is far
> smaller on disk (3 MB vs 436 MB) and starts ~4.5x faster than the official
> client, but ties it on memory and uses ~10x more idle CPU. See
> [docs/RESULTS.md](docs/RESULTS.md) and [ADR 0002](docs/adr/0002-archive-project.md).

A **lightweight native shell for [open.spotify.com](https://open.spotify.com)**
built on **Rust + Tauri 2**, using the OS-native **WebView2** on Windows instead
of bundling Chromium like the official CEF-based app. Login and playback are
handled by the Spotify web player itself, so **no API key or app registration
is needed**.

> **Requires Spotify Premium** for full-length playback.

## How it works

- Tauri opens a window that loads `https://open.spotify.com`.
- Global OS media keys (play/pause, next, previous) are captured in Rust and
  forwarded by clicking the matching player buttons on the page
  (`src-tauri/src/media.rs`).
- On first page load the host prints `LITESPOT_READY_MS=<ms>`, which the
  startup benchmark scrapes.

The previous custom client (Svelte UI, OAuth PKCE, Web API) is kept on the
`archive/custom-client` branch. See
[ADR 0001](docs/adr/0001-webview-shell-vs-custom-client.md).

## Prerequisites

- **Windows 10/11** (WebView2 has the best Widevine support), Rust 1.85+, Node 18+
  and the [Tauri prerequisites](https://tauri.app/start/prerequisites/).

## Run

```bash
npm install
npm run tauri dev      # develop
npm run tauri build    # produce an MSI/NSIS installer
```

Rust unit tests: `cd src-tauri && cargo test`.

## Caveats

- Media keys rely on Spotify's `data-testid` selectors and may stop working if
  the site changes them.
- Logging in through third-party providers (Google, Facebook) may be refused
  inside an embedded webview; use email/password.
- **Widevine check:** with `npm run tauri dev`, the bundled page
  `web/drm-test.html` tests DRM support in your WebView2.

## Benchmarks

`benchmarks/` contains a PowerShell harness that compares Litespot against the
official Spotify client for **memory, CPU, startup time, and disk/installer
size**, summing over each app's whole process tree and reporting median ± std
dev across repeated runs. See [`benchmarks/README.md`](benchmarks/README.md).

```powershell
cd benchmarks
copy config.example.json config.json   # then edit paths
.\run.ps1 -Fresh                        # writes REPORT.md
```

## Legal

Litespot uses Spotify's official Web API and Web Playback SDK and requires a
Premium account. It is an independent project and is not affiliated with or
endorsed by Spotify. Respect the
[Spotify Developer Terms](https://developer.spotify.com/terms).

## License

MIT
