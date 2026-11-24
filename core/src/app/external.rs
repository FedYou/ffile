use crate::file_system::directories::Entry;
use std::path::PathBuf;

pub struct PendingExternalActions {
    /// Archivo listo para abrirse con la app externa por defecto.
    pub open_with_app: Option<Entry>,
    /// Archivo listo para abrirse en el editor externo configurado.
    pub open_with_editor: Option<PathBuf>,
}

impl Default for PendingExternalActions {
    fn default() -> Self {
        Self {
            open_with_app: None,
            open_with_editor: None,
        }
    }
}
