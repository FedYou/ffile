mod _fs;
mod app;
mod config;
mod controller;
mod global;
mod keyboard;
mod open_file;
mod tui;

use app::App;
use crossterm::{
    event::{DisableBracketedPaste, EnableBracketedPaste},
    execute,
};
use notify::{Config, RecommendedWatcher, Watcher};
use std::io::stdout;
use std::sync::mpsc::channel;

fn main() -> Result<(), std::io::Error> {
    let mut terminal = ratatui::init();
    execute!(stdout(), EnableBracketedPaste)?;

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
            match crossterm::event::read()? {
                crossterm::event::Event::Paste(text) => app.execute_input_paste(text),
                crossterm::event::Event::Key(key) => {
                    let action = keyboard::get_action(&app, key.code, None);

                    app.execute_input_key(key.code);

                    app.execute_action(action);
                }
                _ => {}
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
            execute!(stdout(), DisableBracketedPaste)?;

            run_external_app(&mut terminal, || {
                open_file::editor(path.to_str().unwrap());
            });

            execute!(stdout(), EnableBracketedPaste)?;

            dirty = true
        }
    }

    execute!(stdout(), DisableBracketedPaste)?;
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
