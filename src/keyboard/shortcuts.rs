use crate::app::Mode;
use crossterm::event::KeyCode;

#[derive(PartialEq, Clone)]
pub enum Action {
    MoveUp,
    MoveDown,
    Exit,
    ChangeModeFromSideBar,
    ChangeModeFromFilePanel,
    ChangeModeFromMetadata,
    ChangeModeMetadata,
    CopyMetadata,
    OpenFromSideBar,
    OpenFromFilePanel,
    ToParentDirectory,
    None,
}

pub struct Shortcut {
    pub key: KeyCode,
    pub action: Action,
    pub mode: Mode,
}

pub static SHORTCUTS: &[Shortcut] = &[
    // -------------------------
    // ----- FilePanel Mode
    // -------------------------
    Shortcut {
        key: KeyCode::Char('q'),
        action: Action::Exit,
        mode: Mode::FilePanel,
    },
    Shortcut {
        key: KeyCode::Up,
        action: Action::MoveUp,
        mode: Mode::FilePanel,
    },
    Shortcut {
        key: KeyCode::Down,
        action: Action::MoveDown,
        mode: Mode::FilePanel,
    },
    Shortcut {
        key: KeyCode::Char('f'),
        action: Action::ChangeModeFromFilePanel,
        mode: Mode::FilePanel,
    },
    Shortcut {
        key: KeyCode::Right,
        action: Action::OpenFromFilePanel,
        mode: Mode::FilePanel,
    },
    Shortcut {
        key: KeyCode::Enter,
        action: Action::OpenFromFilePanel,
        mode: Mode::FilePanel,
    },
    Shortcut {
        key: KeyCode::Left,
        action: Action::ToParentDirectory,
        mode: Mode::FilePanel,
    },
    Shortcut {
        key: KeyCode::Char('m'),
        action: Action::ChangeModeMetadata,
        mode: Mode::FilePanel,
    },
    // -------------------------
    // ----- SideBar Mode
    // -------------------------
    Shortcut {
        key: KeyCode::Char('q'),
        action: Action::Exit,
        mode: Mode::SideBar,
    },
    Shortcut {
        key: KeyCode::Up,
        action: Action::MoveUp,
        mode: Mode::SideBar,
    },
    Shortcut {
        key: KeyCode::Down,
        action: Action::MoveDown,
        mode: Mode::SideBar,
    },
    Shortcut {
        key: KeyCode::Char('f'),
        action: Action::ChangeModeFromSideBar,
        mode: Mode::SideBar,
    },
    Shortcut {
        key: KeyCode::Right,
        action: Action::OpenFromSideBar,
        mode: Mode::SideBar,
    },
    Shortcut {
        key: KeyCode::Enter,
        action: Action::OpenFromSideBar,
        mode: Mode::SideBar,
    },
    // -------------------------
    // ----- Metadata Mode
    // -------------------------
    Shortcut {
        key: KeyCode::Char('q'),
        action: Action::Exit,
        mode: Mode::Metadata,
    },
    Shortcut {
        key: KeyCode::Up,
        action: Action::MoveUp,
        mode: Mode::Metadata,
    },
    Shortcut {
        key: KeyCode::Down,
        action: Action::MoveDown,
        mode: Mode::Metadata,
    },
    Shortcut {
        key: KeyCode::Char('f'),
        action: Action::ChangeModeFromMetadata,
        mode: Mode::Metadata,
    },
    Shortcut {
        key: KeyCode::Char(' '),
        action: Action::CopyMetadata,
        mode: Mode::Metadata,
    },
];
