use std::{path::PathBuf, process::Command, thread};

use crate::config::get_config;

/// Abre un archivo con la aplicación externa por defecto del sistema
/// (`open::that`) en un hilo aparte para no bloquear el TUI.
pub fn file(path: PathBuf) {
    thread::spawn(move || open::that(path));
}

/// Abre un archivo en el editor externo configurado (`config.editor`),
/// esperando a que el proceso termine.
pub fn editor(path: &str) {
    let config = get_config().lock().unwrap();
    let mut cmd = Command::new(config.editor.clone());
    cmd.arg(path);

    #[cfg(target_os = "linux")]
    unsafe {
        use std::os::unix::process::CommandExt;
        cmd.pre_exec(|| {
            // Cuando el proceso de ffile muera, el kernel le manda SIGTERM
            // al editor y este se cierra junto con la app.
            libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGTERM);
            // Race: si el padre ya murió entre fork y prctl, salir ya.
            if libc::getppid() == 1 {
                libc::_exit(1);
            }
            Ok(())
        });
    }

    let _ = cmd.status();
}
