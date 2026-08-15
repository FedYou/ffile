use crate::app::Panel;
use crossterm::event::{KeyCode, KeyEvent as CrosstermKeyEvent, KeyModifiers};
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;

mod actions;
pub use actions::Action;

// ─── Tipos y estructuras de apoyo ────────────────────────────────────────

/// Estructura "cruda" del TOML: cada campo es un mapa `atajo -> nombre_accion`
/// (ej. `"Ctrl-a" = "open_entry"`), separado por panel.
#[derive(Debug, Deserialize)]
struct ShortcutsToml {
    #[serde(default)]
    global: HashMap<String, String>,
    #[serde(default)]
    file_panel: HashMap<String, String>,
    #[serde(default)]
    sidebar: HashMap<String, String>,
    #[serde(default)]
    metadata: HashMap<String, String>,
    #[serde(default)]
    create: HashMap<String, String>,
    #[serde(default)]
    file_clipboard: HashMap<String, String>,
}

/// Representación normalizada de una combinación de teclas.
struct KeyEvent {
    code: KeyCode,
    ctrl: bool,
    alt: bool,
    shift: bool,
}

impl KeyEvent {
    /// Compara este `KeyEvent` contra un evento real de crossterm: la tecla
    /// y los tres modificadores deben coincidir exactamente.
    fn matches(&self, ev: &CrosstermKeyEvent) -> bool {
        self.code == ev.code
            && self.ctrl == ev.modifiers.contains(KeyModifiers::CONTROL)
            && self.alt == ev.modifiers.contains(KeyModifiers::ALT)
            && self.shift == ev.modifiers.contains(KeyModifiers::SHIFT)
    }
}

/// Un atajo ya parseado: qué tecla dispara qué `Action`.
struct Shortcut {
    action: Action,
    key_event: KeyEvent,
}

/// Todos los atajos parseados, agrupados por panel (más un grupo `global`
/// que aplica sin importar el panel activo).
struct Shortcuts {
    global: Vec<Shortcut>,
    file_panel: Vec<Shortcut>,
    sidebar: Vec<Shortcut>,
    metadata: Vec<Shortcut>,
    create: Vec<Shortcut>,
    file_clipboard: Vec<Shortcut>,
}

// ─── Parseo de teclas ─────────────────────────────────────────────────────

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

/// Convierte un string de atajo (ej. `"Ctrl-Shift-F5"`, `"a"`, `"-"`) en un
/// `KeyEvent`. Devuelve `None` si el formato no es válido.
fn parse_key_event(s: &str) -> Option<KeyEvent> {
    let re = Regex::new(
        r#"^(?:Ctrl\-)?(?:Alt\-)?(?:Shift\-)?(?:Tab|Enter|Esc|Space|Backspace|Left|Right|Up|Down|Home|End|Insert|Delete|PageUp|PageDown|F(?:[1-9]|1[0-2])|[A-Za-z0-9]|[!@#$%^&*()_+\-=\[\]{};:'",.<>/?\\|`~])$"#,
    )
    .unwrap();

    if !re.is_match(s) {
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
    let shift: bool = exist_modifier(&mut parts, "Shift");
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

// ─── Mapeo texto -> Action (por panel) ───────────────────────────────────

/// Acciones válidas en el grupo `global` (aplican en cualquier panel).
fn parse_action_global(s: &str) -> Action {
    match s {
        "exit" => Action::Exit,
        "move_up" => Action::MoveUp,
        "move_down" => Action::MoveDown,
        _ => Action::None,
    }
}

/// Acciones válidas cuando el panel activo es `FilePanel`.
fn parse_action_file_panel(s: &str) -> Action {
    match s {
        "focus_sidebar" => Action::FocusSideBar,
        "open_entry" => Action::OpenEntry,
        "open_editor" => Action::OpenEditor,
        "go_to_parent_dir" => Action::GoToParentDir,
        "focus_metadata" => Action::FocusMetadata,
        "open_create_panel" => Action::OpenCreatePanel,
        "add_entry_clipboard" => Action::AddEntryFileClipboard,
        "focus_file_clipboard" => Action::FocusFileClipboard,
        _ => Action::None,
    }
}

/// Acciones válidas cuando el panel activo es `SideBar`.
fn parse_action_sidebar(s: &str) -> Action {
    match s {
        "focus_file_panel" => Action::FocusFilePanel,
        "open_bookmark" => Action::OpenBookmark,
        _ => Action::None,
    }
}

/// Acciones válidas cuando el panel activo es `Metadata`.
fn parse_action_metadata(s: &str) -> Action {
    match s {
        "exit_metadata" => Action::ExitMetadata,
        "copy_metadata" => Action::CopyMetadata,
        _ => Action::None,
    }
}

/// Acciones válidas cuando el panel activo es `Create`.
fn parse_action_create(s: &str) -> Action {
    match s {
        "cancel_create" => Action::CancelCreate,
        "confirm_create" => Action::ConfirmCreate,
        "move_input_cursor_left" => Action::MoveInputCursorLeft,
        "move_input_cursor_right" => Action::MoveInputCursorRight,
        "remove_input_char" => Action::RemoveInputChar,
        _ => Action::None,
    }
}

/// Acciones válidas cuando el panel activo es `Create`.
fn parse_action_file_clipboard(s: &str) -> Action {
    match s {
        "exit_file_clipboard" => Action::ExitFileClipboard,
        "remove_entry_file_clipboard" => Action::RemoveEntryFileClipboard,
        "clear_file_clipboard" => Action::ClearFileClipboard,
        _ => Action::None,
    }
}

// ───  Carga y resolución de atajos ───────────────────────────────────────

/// ▸ Parsea el TOML crudo (`str`) a la estructura `Shortcuts`, aplicando el
/// parser de acciones correspondiente a cada panel. Los pares clave/acción
/// que no puedan interpretarse como tecla válida se descartan en silencio
/// (`filter_map`).
fn parse_shortcuts(str: &str) -> Shortcuts {
    let toml_parsed: ShortcutsToml =
        toml::from_str(str).expect("Error sintaxis invalida en el TOML de shortcuts");

    /// Arma un `Vec<Shortcut>` a partir del mapa `tecla -> nombre_accion` de
    /// un panel, usando `parse_action` para resolver el nombre de la acción.
    fn build(map: &HashMap<String, String>, parse_action: fn(&str) -> Action) -> Vec<Shortcut> {
        map.iter()
            .filter_map(|(key, action)| {
                Some(Shortcut {
                    action: parse_action(action),
                    key_event: parse_key_event(key)?,
                })
            })
            .collect()
    }

    Shortcuts {
        global: build(&toml_parsed.global, parse_action_global),
        file_panel: build(&toml_parsed.file_panel, parse_action_file_panel),
        sidebar: build(&toml_parsed.sidebar, parse_action_sidebar),
        metadata: build(&toml_parsed.metadata, parse_action_metadata),
        create: build(&toml_parsed.create, parse_action_create),
        file_clipboard: build(&toml_parsed.file_clipboard, parse_action_file_clipboard),
    }
}

/// ▸ Busca en `list` el primer `Shortcut` cuya tecla coincida con `ev`.
fn get_active_action(list: Vec<Shortcut>, ev: &CrosstermKeyEvent) -> Option<Action> {
    if let Some(found) = list.iter().find(|s| s.key_event.matches(ev)) {
        return Some(found.action.clone());
    }
    return None;
}

// ───  API pública ─────────────────────────────────────────────────────────

/// Resuelve qué `Action` corresponde a un evento de teclado, dado el panel
/// activo. Prioridad: primero los atajos `global`, después los específicos
/// del panel actual. Si nada coincide, devuelve `Action::None`.
pub fn get_action(ev: CrosstermKeyEvent, active_panel: &Panel) -> Action {
    let shortcuts = parse_shortcuts(include_str!("./shortcuts.toml"));

    if let Some(a) = get_active_action(shortcuts.global, &ev) {
        return a;
    }

    match active_panel {
        Panel::FilePanel => {
            if let Some(a) = get_active_action(shortcuts.file_panel, &ev) {
                return a;
            }
        }
        Panel::SideBar => {
            if let Some(a) = get_active_action(shortcuts.sidebar, &ev) {
                return a;
            }
        }
        Panel::Metadata => {
            if let Some(a) = get_active_action(shortcuts.metadata, &ev) {
                return a;
            }
        }
        Panel::Create => {
            if let Some(a) = get_active_action(shortcuts.create, &ev) {
                return a;
            }
        }
        Panel::FileClipboard => {
            if let Some(a) = get_active_action(shortcuts.file_clipboard, &ev) {
                return a;
            }
        }
    }

    return Action::None;
}

// ─── Tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_parse_key() {
        assert!(parse_key_event("Ctrl-a").is_some());
        assert!(parse_key_event("Alt-Shift-Tab").is_some());
        assert!(parse_key_event("Shift-F5").is_some());
        assert!(parse_key_event("Ctrl-Shift-a").is_some());
        assert!(parse_key_event("Ctrl-Shift-A").is_some());
        assert!(parse_key_event("Ctrl-Shift-.").is_some());
        assert!(parse_key_event("-").is_some());
    }

    #[test]
    fn test_invalid_parse_key() {
        assert!(parse_key_event("Shift-Ctrl-a").is_none());
        assert!(parse_key_event("Ctrl-Ctrl").is_none());
        assert!(parse_key_event("Shift-a").is_none());
        assert!(parse_key_event("Shift-z").is_none());
    }
}
