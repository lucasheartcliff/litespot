//! Litespot Tauri host library: a thin native shell around open.spotify.com.
//! `run()` is the shared entrypoint used by the desktop binary (`main.rs`).

mod media;
mod navigation;

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

use media::MediaAction;
use tauri::webview::PageLoadEvent;
use tauri::{AppHandle, Manager, WebviewWindowBuilder};
use tauri_plugin_opener::OpenerExt;

static READY_REPORTED: AtomicBool = AtomicBool::new(false);

/// Build and run the application.
///
/// Side effects: opens the main window, registers global media-key shortcuts
/// and prints `LITESPOT_READY_MS=<ms>` to stdout once the first page loads
/// (scraped by the startup benchmark).
pub fn run() {
    let started_at = Instant::now();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .on_page_load(move |webview, payload| {
            if payload.event() == PageLoadEvent::Finished
                && !READY_REPORTED.swap(true, Ordering::SeqCst)
            {
                println!("LITESPOT_READY_MS={}", started_at.elapsed().as_millis());
                // Reveal the window only once content is ready, avoiding a white flash.
                let window = webview.window();
                let _ = window.show();
                let _ = window.set_focus();
            }
        })
        .setup(|app| {
            create_main_window(app.handle())?;
            register_media_keys(app.handle())?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Litespot");
}

/// Create the main window from `tauri.conf.json` with a navigation guard:
/// non-Spotify URLs are opened in the system browser instead of in-app.
fn create_main_window(app: &AppHandle) -> tauri::Result<()> {
    let config = app.config().app.windows[0].clone();
    let opener = app.clone();
    WebviewWindowBuilder::from_config(app, &config)?
        .on_navigation(move |url| {
            if navigation::is_allowed(url.scheme(), url.host_str()) {
                return true;
            }
            if matches!(url.scheme(), "http" | "https") {
                let _ = opener.opener().open_url(url.as_str(), None::<&str>);
            }
            false
        })
        .build()?;
    Ok(())
}

/// Register OS media keys and forward each press to the page as a button click.
fn register_media_keys(app: &AppHandle) -> tauri::Result<()> {
    use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Shortcut, ShortcutState};

    let bindings = [
        (Code::MediaPlayPause, MediaAction::PlayPause),
        (Code::MediaTrackNext, MediaAction::Next),
        (Code::MediaTrackPrevious, MediaAction::Previous),
    ];

    for (code, action) in bindings {
        let handle = app.clone();
        let result = app.global_shortcut().on_shortcut(
            Shortcut::new(None, code),
            move |_app, _sc, event| {
                if event.state() == ShortcutState::Pressed {
                    if let Some(window) = handle.get_webview_window("main") {
                        let _ = window.eval(action.click_script());
                    }
                }
            },
        );
        // Another app may own the key; that must not prevent startup.
        if let Err(err) = result {
            eprintln!("media key {action:?} not registered: {err}");
        }
    }
    Ok(())
}
