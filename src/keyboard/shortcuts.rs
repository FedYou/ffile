use crossterm::event::KeyCode;

#[derive(PartialEq, Clone)]
pub enum Action {
    Up,
    Down,
    Exit,
    ChangeModeToggle,
    Open,
    ToParent,
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
    Shortcut {
        key: KeyCode::Char('f'),
        action: Action::ChangeModeToggle,
    },
    Shortcut {
        key: KeyCode::Right,
        action: Action::Open,
    },
    Shortcut {
        key: KeyCode::Left,
        action: Action::ToParent,
    },
    Shortcut {
        key: KeyCode::Enter,
        action: Action::Open,
    },
];
