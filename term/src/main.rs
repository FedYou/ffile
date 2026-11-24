use crossterm::{
    event::{DisableBracketedPaste, EnableBracketedPaste},
    execute,
};
use ffile_core::{actions::Action, app::App, open_file, shortcuts, watcher};
use std::{io::stdout, sync::Arc};
use std::{
    sync::atomic::{AtomicBool, AtomicU8, Ordering},
    time::Duration,
};

use crate::utils::{KeyboardEnhancement, convert_key_code, convert_key_event};

mod utils;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State {
    Close = 0,
    Launching = 1,
    Open = 2,
}

impl State {
    fn from_u8(v: u8) -> State {
        match v {
            1 => State::Launching,
            2 => State::Open,
            _ => State::Close,
        }
    }
}

pub struct EditorState(AtomicU8);

impl Default for EditorState {
    fn default() -> Self {
        Self(AtomicU8::new(State::Close as u8))
    }
}

impl EditorState {
    pub fn get(&self) -> State {
        State::from_u8(self.0.load(Ordering::SeqCst))
    }

    fn set(&self, s: State) {
        self.0.store(s as u8, Ordering::SeqCst);
    }

    pub fn open(&self) {
        self.set(State::Open)
    }
    pub fn launching(&self) {
        self.set(State::Launching)
    }
    pub fn close(&self) {
        self.set(State::Close)
    }

    pub fn is_open(&self) -> bool {
        self.get() == State::Open
    }
    pub fn is_launching(&self) -> bool {
        self.get() == State::Launching
    }
    pub fn is_close(&self) -> bool {
        self.get() == State::Close
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    ffile_core::init();

    let mut terminal = ratatui::init();
    let keyboard = KeyboardEnhancement::enable();

    execute!(stdout(), EnableBracketedPaste)?;

    let mut app = App::new();

    let (crossterm_tx, mut crossterm_rx) =
        tokio::sync::mpsc::channel::<crossterm::event::Event>(100);

    //  Señal de que el editor externo terminó, y flag para pausar el teclado
    //  (y el render) mientras el editor esté abierto.
    let (tx_editor_done, mut rx_editor_done) = tokio::sync::mpsc::channel::<()>(1);

    let editor_state = Arc::new(EditorState::default());
    let reader_parked = Arc::new(AtomicBool::new(false));

    let mut watcher_rx = watcher::get_rx().lock().await;

    let mut update_app = true;

    let editor_state_clone = editor_state.clone();
    let parked = reader_parked.clone();

    // Dibuja la tui al iniciar la app
    let _ = terminal.draw(|frame| ffile_tui::render(frame, &mut app));

    std::thread::spawn(move || {
        loop {
            if !editor_state_clone.is_close() {
                parked.store(true, Ordering::SeqCst); // ya no leo stdin
                std::thread::sleep(Duration::from_millis(5));
                continue;
            }
            parked.store(false, Ordering::SeqCst);
            if let Ok(true) = crossterm::event::poll(Duration::from_millis(50)) {
                if let Ok(ev) = crossterm::event::read() {
                    let _ = crossterm_tx.blocking_send(ev);
                }
            }
        }
    });

    loop {
        if app.quit {
            break;
        }

        tokio::select! {
            Some(event) = watcher_rx.recv() => {
                match event.kind {
                    watcher::EventKind::Any => {}
                    watcher::EventKind::Other => {}
                    watcher::EventKind::Access(_) => {}
                    _ => update_app = true,
                }
            }
            Some(ev) = crossterm_rx.recv() => {
                match ev {
                    crossterm::event::Event::Paste(string) => {
                        if editor_state.is_close() {
                            app.input.set_string(string);
                        }
                    }
                    crossterm::event::Event::Key(ev) => {
                        if editor_state.is_close() {
                            // Combierte los eventos de crossterm a una manera compatible para el core
                            if let Some(event) = convert_key_event(ev) {
                                let action = shortcuts::get_action(&event, &app.active_panel);
                                if action == Action::None && let Some(code) = convert_key_code(ev.code){
                                     app.execute_input_key(code);
                                } else {
                                     app.execute_action(action);
                                }
                            }

                        }
                    }
                    _ => {}
                }
                // Evita que se aumule la misma tecla
                let crossterm_rx_len = crossterm_rx.len().saturating_sub(1);
                for _ in 0..crossterm_rx_len { let _ = crossterm_rx.try_recv().is_ok(); }
            }
            Some(_) = rx_editor_done.recv() => {
                // El editor externo terminó: restauro la terminal y vuelvo a
                // aceptar teclado y render.
                editor_state.close();

                let _ = crossterm::execute!(
                    stdout(),
                    crossterm::terminal::EnterAlternateScreen,
                    crossterm::cursor::Hide,
                );
                let _ = terminal.clear();
                execute!(stdout(), EnableBracketedPaste)?;
                app.pending_external_actions.open_with_editor = None;
                let _ = keyboard.push();
                let _ = terminal.draw(|frame| ffile_tui::render(frame, &mut app));
                update_app = true;
            }
            Some(event) = app.processes.rx.recv() => {
                app.processes.handle_event(event);
        }
            _ = tokio::time::sleep(std::time::Duration::from_millis(50)) => {
                continue;
            }

        };

        if update_app {
            app.update();
            update_app = false;
        }

        // Abre el archivo con la app externa (o en el editor si es texto).
        if editor_state.is_close()
            && let Some(entry) = app.pending_external_actions.open_with_app.take()
        {
            let is_executable = entry.metadata.clone().is_executable().unwrap_or(true);

            if !is_executable {
                let is_text = entry
                    .metadata
                    .clone()
                    .get_mime_type()
                    .is_some_and(|mime| mime.contains("text"));

                if is_text {
                    app.pending_external_actions.open_with_editor = Some(entry.path);
                } else {
                    open_file::file(entry.path);
                }
            }

            update_app = true;
        }

        if editor_state.is_close() || editor_state.is_launching() {
            // Renderiza el tui si el editor no se abrio aun o no esta abierto
            let _ = terminal.draw(|frame| ffile_tui::render(frame, &mut app));
        }

        // Abre el archivo en el editor externo, en un hilo para no bloquear
        // el loop; `rx_editor_done` restaura la terminal al cerrarse.
        if editor_state.is_close()
            && let Some(path) = app.pending_external_actions.open_with_editor.clone()
        {
            reader_parked.store(false, Ordering::SeqCst);
            editor_state.launching();

            // Espera (~20 ms como mucho) a que el lector deje de tocar stdin
            while !reader_parked.load(Ordering::SeqCst) {
                tokio::time::sleep(Duration::from_millis(1)).await;
            }

            let _ = keyboard.pop();
            let _ = execute!(stdout(), DisableBracketedPaste);
            editor_state.open();

            let tx_done = tx_editor_done.clone();
            std::thread::spawn(move || {
                open_file::terminal_editor(&path.to_str().unwrap());
                let _ = tx_done.blocking_send(());
            });
        }
    }

    execute!(stdout(), DisableBracketedPaste)?;
    ratatui::restore();

    Ok(())
}
