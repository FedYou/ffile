use crossterm::event::KeyCode;

#[derive(PartialEq, Clone)]
pub enum Action {
    Exit,
    None,
}

pub struct Shortcut {
    pub key: KeyCode,
    pub action: Action,
}

pub static SHORTCUTS: &[Shortcut] = &[Shortcut {
    key: KeyCode::Char('q'),
    action: Action::Exit,
}];
