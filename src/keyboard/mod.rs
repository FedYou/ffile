use crossterm::event::KeyCode;
mod shortcuts;

use shortcuts::SHORTCUTS;

pub use shortcuts::{Action, Shortcut};

use crate::app::App;

pub fn get_action(app: &App, key: KeyCode, custom_shortcuts: Option<&[Shortcut]>) -> Action {
    if let Some(custom) = custom_shortcuts {
        if let Some(found) = custom.iter().find(|c| c.key == key && c.mode == app.mode) {
            return found.action.clone();
        }
    }

    if let Some(found) = SHORTCUTS
        .iter()
        .find(|d| d.key == key && d.mode == app.mode)
    {
        return found.action.clone();
    }

    Action::None
}
