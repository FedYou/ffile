mod shortcuts;
mod types;

use std::collections::HashMap;

pub use shortcuts::Shortcuts;
pub use types::{KeyCode, KeyEvent};

use crate::{
    actions::ACTIONS_ENABLED, actions::Action, app::panels::Panel, get_config,
    shortcuts::shortcuts::ShortcutKey,
};

fn get_active_action(list: &HashMap<ShortcutKey, Action>, event: &KeyEvent) -> Option<Action> {
    let shortcut = event.clone().into();

    if let Some(action) = list.get(&shortcut) {
        return Some(action.clone());
    }

    return None;
}

fn is_plain_text_key(event: &KeyEvent) -> bool {
    matches!(event.code, KeyCode::Char(_)) && !event.ctrl && !event.alt
}

pub fn get_action(event: &KeyEvent, active_panel: &Panel) -> Action {
    let shortcuts = &get_config().shortcuts;

    match active_panel {
        Panel::Files => {
            if let Some(a) = get_active_action(&shortcuts.files, event) {
                if ACTIONS_ENABLED.files.contains(&a) {
                    return a;
                }
            }
        }
        Panel::SideBar => {
            if let Some(a) = get_active_action(&shortcuts.sidebar, event) {
                if ACTIONS_ENABLED.sidebar.contains(&a) {
                    return a;
                }
            }
        }
        Panel::Metadata => {
            if let Some(a) = get_active_action(&shortcuts.metadata, event) {
                if ACTIONS_ENABLED.metadata.contains(&a) {
                    return a;
                }
            }
        }
        Panel::New => {
            if let Some(a) = get_active_action(&shortcuts.create, event) {
                if ACTIONS_ENABLED.create.contains(&a) {
                    return a;
                }
            }
        }
        Panel::Delete => {
            if let Some(a) = get_active_action(&shortcuts.delete, event) {
                if ACTIONS_ENABLED.delete.contains(&a) {
                    return a;
                }
            }
        }
        Panel::Clipboard => {
            if let Some(a) = get_active_action(&shortcuts.clipboard, event) {
                if ACTIONS_ENABLED.clipboard.contains(&a) {
                    return a;
                }
            }
        }
        Panel::Processes => {
            if let Some(a) = get_active_action(&shortcuts.processes, event) {
                if ACTIONS_ENABLED.processes.contains(&a) {
                    return a;
                }
            }
        }
        _ => {}
    }

    if *active_panel == Panel::New && is_plain_text_key(event) {
        return Action::None;
    }

    if let Some(a) = get_active_action(&shortcuts.global, event) {
        if ACTIONS_ENABLED.global.contains(&a) {
            return a;
        }
    }

    return Action::None;
}
