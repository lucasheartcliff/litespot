//! Litespot Tauri host library: a thin native shell around open.spotify.com.
//! `run()` is the shared entrypoint used by the desktop binary (`main.rs`).

mod media;

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

use media::MediaAction;
use tauri::webview::PageLoadEvent;
use tauri::{AppHandle, Manager};

static READY_REPORTED: AtomicBool = AtomicBool::new(false);

/// Build and run the application.
///
/// Side effects: opens the main window, registers global media-key shortcuts
/// and prints `LITESPOT_READY_MS=<ms>` to stdout once the first page loads
/// (scraped by the startup benchmark).
pub fn run() {
    let started_at = Instant::now();

    tauri::Builder::default()
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
            register_media_keys(app.handle())?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Litespot");
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
