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
use std::sync::mpsc::channel;

// ─── Punto de entrada ────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // ──  Setup de terminal y watcher ───────────────────────────────────

    let mut terminal = ratatui::init();
    execute!(stdout(), EnableBracketedPaste)?;

    let (tx, rx) = channel();

    //  Reintenta hasta poder iniciar el watcher del filesystem
    let watcher = loop {
        match RecommendedWatcher::new(tx.clone(), Config::default()) {
            Ok(w) => break w,
            _ => {}
        }
    };

    let mut app = App::new(watcher);

    //  Fuerza un primer `app.update()` en la primera vuelta del loop
    let mut update_app = true;

    // ── Loop principal ─────────────────────────────────────────────────

    loop {
        if app.exit {
            break;
        }

        //  1. Cambios en el filesystem (eventos del watcher)
        while let Ok(res) = rx.try_recv() {
            match res {
                Ok(event) => match event.kind {
                    notify::EventKind::Any => {}
                    notify::EventKind::Other => {}
                    notify::EventKind::Access(_) => {}
                    _ => update_app = true,
                },
                _ => {}
            }
        }

        if update_app {
            app.update();
            update_app = false;
        }

        // 2. Eventos de teclado / paste
        // Espera 50ms; si no pasa nada, sigue el loop sin bloquear la app
        if crossterm::event::poll(std::time::Duration::from_millis(50))? {
            match crossterm::event::read()? {
                crossterm::event::Event::Paste(string) => app.input.set_string(string),
                crossterm::event::Event::Key(ev) => {
                    let action = keymap::get_action(ev, &app.active_panel);
                    app.execute_input_key(ev.code);
                    app.execute_action(action);
                }
                _ => {}
            }
        };

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

        // 4. Render del TUI
        let _ = terminal.draw(|frame| app.render(frame));

        // 5. Apertura de archivos en el editor externo
        // Si hay una ruta pendiente el loop no continúa y se renderiza el
        // editor tui; cuando el editor se cierra, el loop sigue y vuelve a
        // renderizar el TUI normalmente.
        if let Some(path) = app.pending_external_actions.open_with_editor.take() {
            execute!(stdout(), DisableBracketedPaste)?;
            run_external_app(&mut terminal, || {
                open_file::editor(path.to_str().unwrap());
            });
            execute!(stdout(), EnableBracketedPaste)?;
            update_app = true
        }
    }

    // ── Cierre ─────────────────────────────────────────────────────────

    execute!(stdout(), DisableBracketedPaste)?;
    ratatui::restore();
    return Ok(());
}

// ─── Helpers ──────────────────────────────────────────────────────────────

/// Suspende el TUI para correr una app externa `f` (bloqueante) y, al
/// terminar, restaura la pantalla alternativa y el cursor, y limpia el
/// terminal para volver a dibujar el TUI desde cero.
fn run_external_app<F: FnOnce()>(
    terminal: &mut ratatui::Terminal<impl ratatui::backend::Backend>,
    f: F,
) {
    f();

    // Volver al TUI
    let _ = crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::EnterAlternateScreen,
        crossterm::cursor::Hide,
    );
    let _ = terminal.clear();
}
