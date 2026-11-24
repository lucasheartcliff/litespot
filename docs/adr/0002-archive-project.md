# 0002 - Archive the project

Issues: #6 · Status: accepted

## Context
Litespot set out to be a lighter Spotify desktop client. Measured on one machine
(see [RESULTS.md](../RESULTS.md)), it is far smaller on disk (3 MB vs 436 MB) and
starts about 4.5x faster, but it ties the official client on memory and uses
roughly 10x more CPU while idle. Memory and CPU are dominated by the Spotify web
player running in WebView2, which the shell does not control. Remaining ideas
(background memory level, fewer WebView2 processes) are untested, would need
careful Widevine and playback testing, and the shell depends on a site Spotify
may change or restrict at any time.

## Decision
Stop development and archive the repository. Record the results and the
untested ideas so the work can be resumed or learned from.

## Consequences
- The repository becomes read-only on GitHub; it can be unarchived later.
- Issue #6 (optimization) is closed as not planned and keeps its checklist.
- The custom-client implementation stays on `archive/custom-client`.
- Several claims remain unverified (playback, login persistence, media keys,
  Widevine), as listed in RESULTS.md.

## Alternatives considered
- **Continue with the optimization plan in #6:** possible, but the expected gain
  is uncertain and the dependency on a third-party site remains.
- **Return to the custom client:** stable and sanctioned, but needs an app
  registration and a full UI, and still uses WebView2 for audio.
