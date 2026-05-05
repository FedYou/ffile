mod app;
mod config;
mod controller;
mod fs;
mod global;
mod keyboard;
mod open_file;
mod tui;

use app::App;
use notify::{Config, RecommendedWatcher, Watcher};
use std::sync::mpsc::channel;

fn main() -> Result<(), std::io::Error> {
    let mut terminal = ratatui::init();
    let (tx, rx) = channel();

    let watcher = loop {
        // Retorna el watcher, reintenta hasta que se pueda iniciar
        match RecommendedWatcher::new(tx.clone(), Config::default()) {
            Ok(w) => break w,
            _ => {}
        }
    };

    let mut app = App::new(watcher);
    let mut dirty = true;

    loop {
        if app.exit {
            break;
        }

        while let Ok(res) = rx.try_recv() {
            match res {
                Ok(event) => match event.kind {
                    notify::EventKind::Any => {}
                    notify::EventKind::Other => {}
                    notify::EventKind::Access(_) => {}
                    _ => dirty = true,
                },
                _ => {}
            }
        }

        if dirty {
            app.update();
            dirty = false;
        }

        // Espera 50 milisegundo si no pasa nada sigue el loop sin bloquer la app
        if crossterm::event::poll(std::time::Duration::from_millis(50))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                let action = keyboard::get_action(&app, key.code, None);

                app.execute_action(action);
            }
        };

        if let Some(path) = app.pending_open.take() {
            let mime = mime_guess::from_path(&path).first_or_text_plain();

            if mime.type_() == "text" {
                app.pending_open_editor = Some(path)
            } else {
                open_file::file(path);
            }

            dirty = true;
        }

        let _ = terminal.draw(|frame| app.render(frame));

        //Si hay una ruta pendiente el loop no continua y renderiza el editor,
        // cuando el editor se cierra el loop continua,donde vuelve renderiza el tui normalmente
        if let Some(path) = app.pending_open_editor.take() {
            run_external_app(&mut terminal, || {
                open_file::editor(path.to_str().unwrap());
            });
            dirty = true
        }
    }

    ratatui::restore();
    return Ok(());
}

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
