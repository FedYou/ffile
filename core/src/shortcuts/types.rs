#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    Char(char),

    Tab,
    Enter,
    Esc,
    Backspace,
    Delete,
    Insert,

    Left,
    Right,
    Up,
    Down,

    Home,
    End,
    PageUp,
    PageDown,

    F(u8),
}

impl KeyCode {
    pub fn as_char(&self) -> Option<char> {
        match self {
            KeyCode::Char(ch) => Some(ch.clone()),
            KeyCode::Enter => Some('\n'),
            _ => None,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}
