//! Media-key handling: maps OS media keys to the player controls rendered by
//! open.spotify.com.

/// A transport action triggered by an OS media key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaAction {
    PlayPause,
    Next,
    Previous,
}

impl MediaAction {
    /// The `data-testid` of the matching button on open.spotify.com.
    ///
    /// Spotify can rename these without notice; this is the single place to
    /// update when that happens.
    fn button_test_id(self) -> &'static str {
        match self {
            MediaAction::PlayPause => "control-button-playpause",
            MediaAction::Next => "control-button-skip-forward",
            MediaAction::Previous => "control-button-skip-back",
        }
    }

    /// JavaScript that clicks the player button for this action, if present.
    pub fn click_script(self) -> String {
        format!(
            "document.querySelector('[data-testid=\"{}\"]')?.click();",
            self.button_test_id()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_pause_script_targets_playpause_button() {
        assert_eq!(
            MediaAction::PlayPause.click_script(),
            "document.querySelector('[data-testid=\"control-button-playpause\"]')?.click();"
        );
    }

    #[test]
    fn next_and_previous_target_distinct_buttons() {
        let next = MediaAction::Next.click_script();
        let prev = MediaAction::Previous.click_script();
        assert!(next.contains("skip-forward"));
        assert!(prev.contains("skip-back"));
        assert_ne!(next, prev);
    }

    #[test]
    fn script_is_noop_safe_when_button_missing() {
        // Optional chaining means a missing button never throws in the page.
        assert!(MediaAction::Next.click_script().contains("?.click()"));
    }
}
