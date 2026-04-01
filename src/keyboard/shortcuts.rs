use crossterm::event::KeyCode;

#[derive(PartialEq, Clone)]
pub enum Action {
    Up,
    Down,
    Exit,
    None,
}

pub struct Shortcut {
    pub key: KeyCode,
    pub action: Action,
}

pub static SHORTCUTS: &[Shortcut] = &[
    Shortcut {
        key: KeyCode::Char('q'),
        action: Action::Exit,
    },
    Shortcut {
        key: KeyCode::Up,
        action: Action::Up,
    },
    Shortcut {
        key: KeyCode::Down,
        action: Action::Down,
    },
];
