mod app;
mod config;
mod controller;
mod fs;
mod global;
mod keyboard;
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

    // title: "FFile".to_string(),
    // exit: false,
    // is_small: false,
    // controller: Controller::new(watcher),
    // };
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
                let action = keyboard::get_action(key.code, None);

                app.execute_action(action);
            }
        };

        let _ = terminal.draw(|frame| app.render(frame));
    }

    ratatui::restore();
    return Ok(());
}
