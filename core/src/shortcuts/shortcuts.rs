use super::{KeyCode, KeyEvent};
use crate::actions::Action;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::LazyLock};

// ---------- Macros ----------
macro_rules! shortcuts {
    ($($key:literal => $action:expr),* $(,)?) => {
        HashMap::from([
            $(
                (
                    ShortcutKey(
                        parse_key_event($key)
                            .expect(concat!("Invalid shortcut: ", $key))
                    ),
                    $action
                ),
            )*
        ])
    };
}

// ----------------------------

const REGEX_PARSE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^(?:Ctrl\-)?(?:Alt\-)?(?:Shift\-)?(?:Tab|Enter|Esc|Space|Backspace|Left|Right|Up|Down|Home|End|Insert|Delete|PageUp|PageDown|F(?:[1-9]|1[0-2])|[A-Za-z0-9]|[!@#$%^&*()_+\-=\[\]{};:'",.<>/?\\|`~])$"#,
    )
    .unwrap()
});

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Shortcuts {
    pub global: HashMap<ShortcutKey, Action>,
    pub files: HashMap<ShortcutKey, Action>,
    pub sidebar: HashMap<ShortcutKey, Action>,
    pub metadata: HashMap<ShortcutKey, Action>,
    pub create: HashMap<ShortcutKey, Action>,
    pub delete: HashMap<ShortcutKey, Action>,
    pub clipboard: HashMap<ShortcutKey, Action>,
    pub processes: HashMap<ShortcutKey, Action>,
}

impl Default for Shortcuts {
    fn default() -> Self {
        Self {
            global: shortcuts! {
                "q"       => Action::Quit,
                "Up"      => Action::Up,
                "Down"    => Action::Down,
                "Left"    => Action::Left,
                "Right"   => Action::Right,
                "Tab"     => Action::Focus,
                "Enter"   => Action::Confirm,
                "Esc"     => Action::Cancel,
                "Ctrl-p"  => Action::Processes,
            },

            files: shortcuts! {
                "." => Action::ToggleShowFiles,
                "-" => Action::ToggleSort,
                "," => Action::ToggleInvertSort,
                "m" => Action::Metadata,
                "n" => Action::New,
                "d" => Action::Delete,
                "y" => Action::AddClipboard,
                "Y" => Action::RemoveClipboard,
                "c" => Action::FocusClipboard,
                "p" => Action::Copy,
                "e" => Action::Editor,
            },

            sidebar: HashMap::new(),

            metadata: shortcuts! {
                "y"     => Action::CopyMetadata,
                "Space" => Action::CopyMetadata,
            },

            create: shortcuts! {
                "Backspace" => Action::RemoveInputChar,
            },

            delete: HashMap::new(),

            clipboard: shortcuts! {
                "d" => Action::Delete,
                "c" => Action::ClearFileClipboard,
            },

            processes: shortcuts! {
                "Ctrl-c" => Action::CancelProcess,
                "c"      => Action::CancelProcess,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShortcutKey(pub KeyEvent);

impl From<KeyEvent> for ShortcutKey {
    fn from(event: KeyEvent) -> Self {
        Self(event)
    }
}

// Añade implementaciones para serializar y deserializar con serde el toml del config

impl Serialize for ShortcutKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&serialize_str(&self.0))
    }
}

impl<'de> Deserialize<'de> for ShortcutKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        let key = parse_key_event(&value)
            .ok_or_else(|| serde::de::Error::custom(format!("Invalid shortcut key: {value}")))?;

        Ok(Self(key))
    }
}

fn serialize_str(key: &KeyEvent) -> String {
    let mut result = String::new();

    if key.ctrl {
        result.push_str("Ctrl-");
    }

    if key.alt {
        result.push_str("Alt-");
    }

    match key.code {
        KeyCode::Char(_) => {
            if (key.ctrl || key.alt) && key.shift {
                result.push_str("Shift-");
            }
        }
        _ => {
            if key.shift {
                result.push_str("Shift-");
            }
        }
    }

    let code = match key.code {
        KeyCode::Tab => "Tab".to_string(),
        KeyCode::Enter => "Enter".to_string(),
        KeyCode::Esc => "Esc".to_string(),
        KeyCode::Char(' ') => "Space".to_string(),
        KeyCode::Backspace => "Backspace".to_string(),
        KeyCode::Left => "Left".to_string(),
        KeyCode::Right => "Right".to_string(),
        KeyCode::Up => "Up".to_string(),
        KeyCode::Down => "Down".to_string(),
        KeyCode::Home => "Home".to_string(),
        KeyCode::End => "End".to_string(),
        KeyCode::Insert => "Insert".to_string(),
        KeyCode::Delete => "Delete".to_string(),
        KeyCode::PageUp => "PageUp".to_string(),
        KeyCode::PageDown => "PageDown".to_string(),
        KeyCode::F(n) => format!("F{n}"),
        KeyCode::Char(c) => c.to_string(),
    };

    result.push_str(&code);

    result
}

/// Busca `modifier` (ej. "Ctrl") dentro de `parts`; si lo encuentra lo
/// remueve del vector y devuelve `true`. Se usa para ir "consumiendo" los
/// prefijos de modificador antes de quedarse con la tecla final.
fn exist_modifier(parts: &mut Vec<&str>, modifier: &str) -> bool {
    if let Some(i) = parts.iter().position(|&x| x == modifier) {
        parts.remove(i);
        return true;
    }
    return false;
}

fn parse_key_event(s: &str) -> Option<KeyEvent> {
    if !REGEX_PARSE.is_match(s) {
        return None;
    }

    // Caso especial: "-" a secas es la tecla guion, no un separador
    if s == "-" {
        return Some(KeyEvent {
            code: KeyCode::Char('-'),
            ctrl: false,
            shift: false,
            alt: false,
        });
    }

    if s.contains("--") {
        return None;
    }

    let mut parts: Vec<&str> = s.split('-').collect();

    // Consume los modificadores presentes;
    let ctrl: bool = exist_modifier(&mut parts, "Ctrl");
    let mut shift: bool = exist_modifier(&mut parts, "Shift");
    let alt: bool = exist_modifier(&mut parts, "Alt");

    if parts.len() > 1 {
        return None;
    }

    let key_str: &str = parts.last().map_or("", |v| v);

    //  Evita ambigüedad: "Shift-a" y "A" representarían la misma tecla,
    //  así que se rechaza Shift + letra sola (sin Ctrl/Alt)
    if shift
        && !ctrl
        && !alt
        && key_str.len() == 1
        && key_str.chars().next().unwrap().is_ascii_alphabetic()
    {
        return None;
    }

    let code = match key_str {
        "Tab" => KeyCode::Tab,
        "Enter" => KeyCode::Enter,
        "Esc" => KeyCode::Esc,
        "Space" => KeyCode::Char(' '),
        "Backspace" => KeyCode::Backspace,
        "Left" => KeyCode::Left,
        "Right" => KeyCode::Right,
        "Up" => KeyCode::Up,
        "Down" => KeyCode::Down,
        "Home" => KeyCode::Home,
        "End" => KeyCode::End,
        "Insert" => KeyCode::Insert,
        "Delete" => KeyCode::Delete,
        "PageUp" => KeyCode::PageUp,
        "PageDown" => KeyCode::PageDown,
        str if str.len() == 1 => {
            let ch = str.chars().next().unwrap();
            if !ch.is_control() {
                if ch.is_ascii_uppercase() {
                    shift = true;
                }
                KeyCode::Char(ch)
            } else {
                return None;
            }
        }
        str if str.starts_with('F') => {
            let n: u8 = str[1..].parse().ok()?;
            KeyCode::F(n)
        }
        _ => return None,
    };

    Some(KeyEvent {
        code,
        ctrl,
        alt,
        shift,
    })
}
