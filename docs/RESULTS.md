# Litespot - final results

Status: **archived** (see [ADR 0002](adr/0002-archive-project.md)).

Litespot is a native Tauri 2 shell that loads `https://open.spotify.com` in the
system WebView2, as a lighter alternative to the official CEF-based Spotify
desktop client. This page records what was measured, what was not, and why the
project was closed. Raw data is in [`docs/results/`](results/).

## Setup

- One Windows 11 machine, 20 logical cores, PowerShell 5.1.
- Litespot 0.1.0 (NSIS installer built by CI, commit `8ba418e`) vs Spotify
  1.3.10 (installed with `winget`).
- Harness: `benchmarks/` after the fixes in issue #4 / PR #5. Memory and CPU are
  summed over each app's process tree (Litespot: `litespot` + `msedgewebview2`;
  Spotify: `Spotify`). Median across runs; 30 s sample window.

## Results

| Metric | Litespot | Spotify | Notes |
|---|---|---|---|
| Installed size | **3 MB** | 436 MB | one 3 MB executable; WebView2 is a system component |
| Installer | **1.1 MB** | not measured | |
| Startup (launch -> window visible) | **229 ms** (sd 87) | 1031 ms (sd 18.5) | 5 runs each |
| Working set, idle, logged out | 968 MB (sd 18) | 954 MB (sd 45) | 3 runs each |
| Private memory, idle, logged out | 625 MB | 710 MB | |
| CPU, idle, logged out | 4.5% | 0.2% | |
| Working set, idle, second pass | 967 MB | 988 MB | login state unverified |
| Private memory, idle, second pass | 625 MB | 721 MB | |
| CPU, idle, second pass | 5.3% | 0.6% | |

CPU is a percentage of the whole machine (100% = every core fully busy).

## What the numbers say

- **Disk and startup: a clear win.** About 145x smaller on disk and about 4.5x
  faster to show a window.
- **Memory: a tie.** The Rust host uses about 24 MB. Nearly everything else is
  eight WebView2 processes running the Spotify web player, which Litespot does
  not control. Private memory is somewhat lower than the official client;
  working set is about equal.
- **Idle CPU: a loss.** Litespot used roughly 10x more CPU than the official
  client while idle (about 5% of a 20-core machine, close to one core). The same
  value appeared in both passes, so the cause is the web player itself, not
  login. The official client appears to pause work when not in focus; the web
  player inside WebView2 does not.

So the "lightweight" claim holds for installation and startup, but not for the
runtime cost that matters most for an app left open all day.

## What was not measured or verified

- **Playback.** Memory and CPU while playing were never measured; DRM playback
  cannot be automated and needed a manual Play.
- **Login state.** The second pass was meant to be logged in on both apps, but
  Litespot's numbers were almost identical to the logged-out pass, so I could not
  confirm it was logged in. Treat that row as unverified.
- **Widevine / audio.** Whether the installed Litespot plays protected audio was
  not confirmed. `web/drm-test.html` exists for this check.
- **Media keys.** The `data-testid` selectors in `src-tauri/src/media.rs` are from
  memory and were not checked against the logged-in site.
- **Startup metric.** The window is shown on the first `PageLoadEvent::Finished`,
  which may fire before the page paints. 229 ms may therefore understate the time
  to useful content.
- **Third-party login** (Google, Facebook, Apple) inside the embedded webview was
  not tried.
- **Linux and macOS.** CI builds `.deb`, `.rpm` and `.AppImage`, but the app was
  never run there, and WebView2/Widevine behaviour does not carry over.
- **Optimization experiments.** None were run (see below).

## Optimization ideas that were never tested

Issue #6 holds the full list. In short, ordered by expected value:

1. Hide the webview and set WebView2 `MemoryUsageTargetLevel=Low` when the
   window is minimized (targets CPU and memory for the "left open" case).
2. Reduce process count: disable site isolation, `--renderer-process-limit`,
   `--enable-low-end-device-mode`. Site isolation is a security trade-off.
3. Profile CPU per process first, to see whether the renderer or the GPU is
   responsible for the idle cost.
4. Avoid `--disable-gpu` (likely raises CPU) and flags that disable component
   updates (Widevine is a component).

## Harness bugs found along the way

The original benchmark harness was wrong in ways worth remembering if it is
reused (all fixed in PR #5):

- The process tree stopped at the root process, because CIM returns `UInt32`
  parent ids and PowerShell hashtable keys of different numeric types never
  match. Earlier memory numbers measured one process.
- Cleanup stopped processes by helper name and killed every WebView2 process on
  the machine, including other apps'.
- The root could be another app's `msedgewebview2`.
- Processes that were not the app's (a browser opened from a link) were counted.
- Decimal-comma locales turned `968,2` into 9682 in the report.
- Non-ASCII dashes broke the report under Windows PowerShell 5.1.

## History

- The first design was a custom client (Svelte UI, OAuth PKCE, Web API, Web
  Playback SDK). It is preserved on the `archive/custom-client` branch.
- It was replaced by this webview shell (issue #2, PR #3), removing the need for a
  Spotify app registration and a Client ID.
