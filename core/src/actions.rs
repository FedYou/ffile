use std::{collections::HashSet, sync::LazyLock};

use serde::{Deserialize, Serialize};

pub struct ActionsEnabled {
    pub global: HashSet<Action>,
    pub files: HashSet<Action>,
    pub sidebar: HashSet<Action>,
    pub metadata: HashSet<Action>,
    pub create: HashSet<Action>,
    pub delete: HashSet<Action>,
    pub clipboard: HashSet<Action>,
    pub processes: HashSet<Action>,
}

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Quit,

    Up,
    Down,
    Left,
    Right,
    Cancel,
    Confirm,

    Focus,

    /// Panel de metadata: entrar, salir y copiar un valor al portapapeles.
    Metadata,
    CopyMetadata,

    Editor,
    ToggleShowFiles,
    ToggleSort,
    ToggleInvertSort,

    /// Panel de creación de archivos/directorios.
    New,
    Delete,
    RemoveInputChar,

    /// Clipboard
    AddClipboard,
    RemoveClipboard,
    ClearFileClipboard,
    FocusClipboard,
    ExitFileClipboard,

    // Process Panel
    Processes,
    CancelProcess,
    Copy,

    /// Acción vacía: no se hace nada.
    None,
}

pub static ACTIONS_ENABLED: LazyLock<ActionsEnabled> = LazyLock::new(|| ActionsEnabled {
    global: [
        Action::Quit,
        Action::Up,
        Action::Down,
        Action::Left,
        Action::Right,
        Action::Focus,
        Action::Confirm,
        Action::Cancel,
        Action::Processes,
    ]
    .into(),
    files: [
        Action::Up,
        Action::Down,
        Action::Left,
        Action::Right,
        Action::ToggleShowFiles,
        Action::ToggleSort,
        Action::ToggleInvertSort,
        Action::Metadata,
        Action::New,
        Action::Delete,
        Action::AddClipboard,
        Action::RemoveClipboard,
        Action::FocusClipboard,
        Action::Copy,
        Action::Confirm,
        Action::Editor,
    ]
    .into(),
    sidebar: [
        Action::Up,
        Action::Down,
        Action::Left,
        Action::Right,
        Action::Confirm,
    ]
    .into(),
    metadata: [
        Action::Up,
        Action::Down,
        Action::Left,
        Action::Right,
        Action::CopyMetadata,
        Action::Confirm,
    ]
    .into(),
    create: [
        Action::Up,
        Action::Down,
        Action::Left,
        Action::Right,
        Action::RemoveInputChar,
        Action::Confirm,
    ]
    .into(),
    delete: [
        Action::Up,
        Action::Down,
        Action::Left,
        Action::Right,
        Action::Confirm,
    ]
    .into(),
    clipboard: [
        Action::Up,
        Action::Down,
        Action::Left,
        Action::Right,
        Action::Delete,
        Action::ClearFileClipboard,
        Action::Confirm,
    ]
    .into(),
    processes: [
        Action::Up,
        Action::Down,
        Action::Left,
        Action::Right,
        Action::CancelProcess,
        Action::Confirm,
    ]
    .into(),
});
