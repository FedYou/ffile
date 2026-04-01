mod app;
mod controller;
mod fs;
mod keyboard;
mod tui;
mod utils;

use app::App;
use controller::Controller;

fn main() -> Result<(), std::io::Error> {
    let mut terminal = ratatui::init();

    let mut app = App {
        title: "FFile".to_string(),
        exit: false,
        is_small: false,
        controller: Controller::new(),
    };

    loop {
        if app.exit {
            break;
        }

        let _ = terminal.draw(|frame| app.render(frame));

        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            let action = keyboard::get_action(key.code, None);

            app.execute_action(action);
        }
    }

    ratatui::restore();
    return Ok(());
}
