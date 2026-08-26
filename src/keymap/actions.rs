/// Acción que dispara un atajo de teclado. Cada variante es un comando que
/// luego `App::execute_action` interpreta y ejecuta.
#[derive(PartialEq, Clone)]
pub enum Action {
    /// Subir/bajar la selección dentro del panel activo.
    MoveUp,
    MoveDown,

    /// Salir de la app.
    Exit,

    /// Cambiar el foco a un panel.
    FocusFilePanel,
    FocusSideBar,

    /// Panel de metadata: entrar, salir y copiar un valor al portapapeles.
    FocusMetadata,
    ExitMetadata,
    CopyMetadata,

    /// Abrir elementos: bookmarks, archivos/directorios y editor.
    OpenBookmark,
    OpenEntry,
    OpenEditor,
    GoToParentDir,

    /// Panel de creación de archivos/directorios.
    OpenCreatePanel,
    CancelCreate,
    ConfirmCreate,

    /// Movimiento del cursor dentro del input.
    MoveInputCursorLeft,
    MoveInputCursorRight,
    RemoveInputChar,

    /// Clipboard
    AddEntryFileClipboard,
    RemoveEntryFileClipboard,
    ClearFileClipboard,
    FocusFileClipboard,
    ExitFileClipboard,

    // Process Panel
    OpenProcessPanel,
    ExitProcessPanel,
    CancelProcess,
    AddCopyProcess,

    /// Acción vacía: no se hace nada.
    None,
}
