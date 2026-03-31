use crossterm::event::KeyCode;
mod shortcuts;

use shortcuts::SHORTCUTS;

pub use shortcuts::{Action, Shortcut};

pub fn get_action(key: KeyCode, custom_shortcuts: Option<&[Shortcut]>) -> Action {
    if let Some(custom) = custom_shortcuts {
        if let Some(found) = custom.iter().find(|c| c.key == key) {
            return found.action.clone();
        }
    }

    if let Some(found) = SHORTCUTS.iter().find(|d| d.key == key) {
        return found.action.clone();
    }

    Action::None
}
