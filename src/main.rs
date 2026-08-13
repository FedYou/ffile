use ffile::config;
use ffile::keymap;
mod open_file;

use crossterm::{
    event::{DisableBracketedPaste, EnableBracketedPaste},
    execute,
};
use ffile::app::App;
use notify::{Config, RecommendedWatcher, Watcher};
use std::io::stdout;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;

// ─── Punto de entrada ────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // ──  Setup de terminal y watcher ───────────────────────────────────

    let mut terminal = ratatui::init();
    execute!(stdout(), EnableBracketedPaste)?;

    //  Canales tokio: separan las fuentes de eventos del loop de la TUI
    let (tx_fs, mut rx_fs) = tokio::sync::mpsc::channel::<notify::Event>(100);
    let (tx_keys, mut rx_keys) = tokio::sync::mpsc::channel::<crossterm::event::Event>(100);

    //  Señal de que el editor externo terminó, y flag para pausar el teclado
    //  (y el render) mientras el editor esté abierto.
    let (tx_editor_done, mut rx_editor_done) = tokio::sync::mpsc::channel::<()>(1);
    let editor_open = Arc::new(AtomicBool::new(false));

    //  Evita lanzar dos veces el editor en el hueco entre el spawn y que el
    //  hilo del editor active `editor_open`.
    let editor_launching = Arc::new(AtomicBool::new(false));

    let (tx, rx) = channel();

    //  Reintenta hasta poder iniciar el watcher del filesystem
    let watcher = loop {
        match RecommendedWatcher::new(tx.clone(), Config::default()) {
            Ok(w) => break w,
            _ => {}
        }
    };

    //  Puentea los eventos del watcher (canal síncrono) al canal tokio
    tokio::task::spawn_blocking(move || {
        while let Ok(res) = rx.recv() {
            if let Ok(event) = res {
                let _ = tx_fs.blocking_send(event);
            }
        }
    });

    //  Lee el teclado en un hilo aparte y lo reenvía al canal tokio.
    //  Mientras el editor externo esté abierto no lee stdin, para no
    //  robarle las teclas al editor.
    let pause_flag = editor_open.clone();
    std::thread::spawn(move || {
        loop {
            if pause_flag.load(Ordering::Relaxed) {
                std::thread::sleep(std::time::Duration::from_millis(50));
                continue;
            }
            if let Ok(ev) = crossterm::event::read() {
                let _ = tx_keys.blocking_send(ev);
            }
        }
    });

    let mut app = App::new(watcher);

    //  Fuerza un primer `app.update()` en la primera vuelta del loop
    let mut update_app = true;

    // ── Loop principal ─────────────────────────────────────────────────

    loop {
        if app.exit {
            break;
        }

        //  1 y 2. Eventos del watcher y del teclado por canales tokio.
        //  El timeout de 50ms hace que el loop siga sin bloquearse si no
        //  llegan eventos, igual que el `poll` original.
        tokio::select! {
            Some(event) = rx_fs.recv() => {
                match event.kind {
                    notify::EventKind::Any => {}
                    notify::EventKind::Other => {}
                    notify::EventKind::Access(_) => {}
                    _ => update_app = true,
                }
            }
            Some(ev) = rx_keys.recv() => {
                match ev {
                    crossterm::event::Event::Paste(string) => {
                        if !editor_open.load(Ordering::Relaxed) {
                            app.input.set_string(string);
                        }
                    }
                    crossterm::event::Event::Key(ev) => {
                        if !editor_open.load(Ordering::Relaxed) {
                            let action = keymap::get_action(ev, &app.active_panel);
                            app.execute_input_key(ev.code);
                            app.execute_action(action);
                        }
                    }
                    _ => {}
                }
            }
            Some(_) = rx_editor_done.recv() => {
                // El editor externo terminó: restauro la terminal y vuelvo a
                // aceptar teclado y render.
                editor_open.store(false, Ordering::Relaxed);
                editor_launching.store(false, Ordering::Relaxed);
                let _ = crossterm::execute!(
                    stdout(),
                    crossterm::terminal::EnterAlternateScreen,
                    crossterm::cursor::Hide,
                );
                let _ = terminal.clear();
                execute!(stdout(), EnableBracketedPaste)?;
                app.pending_external_actions.open_with_editor = None;
                update_app = true;
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(50)) => {}
        }

        if update_app {
            app.update();
            update_app = false;
        }

        // 3. Apertura de archivos con app externa (o editor, si es texto)
        // Si el archivo es texto, se abre en el editor externo; si no,
        // se abre con la app por defecto del sistema (open_file::file).
        if let Some(path) = app.pending_external_actions.open_with_app.take() {
            let mime = mime_guess::from_path(&path).first_or_text_plain();
            if mime.type_() == "text" {
                app.pending_external_actions.open_with_editor = Some(path)
            } else {
                open_file::file(path);
            }
            update_app = true;
        }

        // 4. Render del TUI (no se dibuja mientras el editor esté abierto,
        // porque el editor es dueño de la pantalla).
        if !editor_open.load(Ordering::Relaxed) {
            let _ = terminal.draw(|frame| app.render(frame));
        }

        // 5. Apertura de archivos en el editor externo
        // Se lanza en un hilo para no bloquear el loop: la app sigue
        // funcionando en segundo plano (watcher, updates) y cuando el editor
        // se cierra, `rx_editor_done` restaura la terminal y refresca.
        // El hilo activa `editor_open` justo antes de lanzar el editor, así
        // el loop sigue dibujando la pantalla "Opening editor..." durante el
        // arranque real y corta el render cuando el editor toma la terminal.
        if !editor_open.load(Ordering::Relaxed)
            && !editor_launching.load(Ordering::Relaxed)
            && let Some(path) = app.pending_external_actions.open_with_editor.clone()
        {
            editor_launching.store(true, Ordering::Relaxed);
            let done = tx_editor_done.clone();
            let flag = editor_open.clone();
            std::thread::spawn(move || {
                flag.store(true, Ordering::Relaxed);
                let _ = execute!(stdout(), DisableBracketedPaste);
                open_file::editor(path.to_str().unwrap());
                let _ = done.blocking_send(());
            });
        }
    }

    // ── Cierre ─────────────────────────────────────────────────────────

    execute!(stdout(), DisableBracketedPaste)?;
    ratatui::restore();
    return Ok(());
}
