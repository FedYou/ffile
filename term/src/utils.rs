use crossterm::{
    event::{
        KeyCode as CrosstermKeyCode, KeyEvent as CrosstermKeyEvent, KeyEventKind, KeyModifiers,
        KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
    },
    execute,
};
use ffile_core::shortcuts::{KeyCode, KeyEvent};
use std::io::{Result as IoResult, stdout};

pub fn convert_key_code(code: CrosstermKeyCode) -> Option<KeyCode> {
    match code {
        CrosstermKeyCode::Char(c) => Some(KeyCode::Char(c)),
        CrosstermKeyCode::Enter => Some(KeyCode::Enter),
        CrosstermKeyCode::Esc => Some(KeyCode::Esc),
        CrosstermKeyCode::Tab | CrosstermKeyCode::BackTab => Some(KeyCode::Tab),
        CrosstermKeyCode::Backspace => Some(KeyCode::Backspace),
        CrosstermKeyCode::Up => Some(KeyCode::Up),
        CrosstermKeyCode::Down => Some(KeyCode::Down),
        CrosstermKeyCode::Left => Some(KeyCode::Left),
        CrosstermKeyCode::Right => Some(KeyCode::Right),
        CrosstermKeyCode::Home => Some(KeyCode::Home),
        CrosstermKeyCode::End => Some(KeyCode::End),
        CrosstermKeyCode::Insert => Some(KeyCode::Insert),
        CrosstermKeyCode::Delete => Some(KeyCode::Delete),
        CrosstermKeyCode::PageUp => Some(KeyCode::PageUp),
        CrosstermKeyCode::PageDown => Some(KeyCode::PageDown),
        CrosstermKeyCode::F(n) => Some(KeyCode::F(n)),
        _ => None,
    }
}

/// Convierte un evento de crossterm para que se pueden usar en el core.

pub fn convert_key_event(event: CrosstermKeyEvent) -> Option<KeyEvent> {
    if event.kind == KeyEventKind::Release {
        return None;
    }

    let is_back_tab = matches!(event.code, CrosstermKeyCode::BackTab);
    let code = convert_key_code(event.code)?;

    let mut shift = event.modifiers.contains(KeyModifiers::SHIFT);
    // Algunos terminales mandan BackTab sin marcar SHIFT.
    if is_back_tab {
        shift = true;
    }

    Some(KeyEvent {
        code,
        ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
        alt: event.modifiers.contains(KeyModifiers::ALT),
        shift,
    })
}

fn enhancement_flags() -> KeyboardEnhancementFlags {
    KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
        | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
}

/// Protocolo de teclado extendido (kitty).
///
/// Es lo único que permite distinguir `Ctrl+Shift+letra`, `Shift+digit`,
/// `Shift+Enter`, etc. En terminales sin soporte no hace nada y queda el modo
/// legacy.
pub struct KeyboardEnhancement {
    supported: bool,
}

impl KeyboardEnhancement {
    /// Consulta la terminal y activa el protocolo.
    ///
    /// IMPORTANTE: llamarlo antes de arrancar el hilo que usa `event::read`,
    /// porque la consulta lee de la terminal y compite por el input.
    pub fn enable() -> Self {
        let supported = crossterm::terminal::supports_keyboard_enhancement().unwrap_or(false);

        if supported {
            let _ = execute!(stdout(), PushKeyboardEnhancementFlags(enhancement_flags()));
        }

        Self { supported }
    }

    pub fn push(&self) -> IoResult<()> {
        if !self.supported {
            return Ok(());
        }
        execute!(stdout(), PushKeyboardEnhancementFlags(enhancement_flags()))
    }

    pub fn pop(&self) -> IoResult<()> {
        if !self.supported {
            return Ok(());
        }
        execute!(stdout(), PopKeyboardEnhancementFlags)
    }
}

impl Drop for KeyboardEnhancement {
    fn drop(&mut self) {
        let _ = self.pop();
    }
}
