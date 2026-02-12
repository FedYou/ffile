use crossterm::event::KeyCode;

struct App {
    exit: bool,
}

impl App {
    fn render(&mut self, frame: &mut ratatui::Frame) {}
}

fn main() -> Result<(), std::io::Error> {
    let mut terminal = ratatui::init();
    let mut app = App { exit: false };

    loop {
        if app.exit {
            break;
        }

        let _ = terminal.draw(|frame| app.render(frame));

        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            if key.code == KeyCode::Char('q') {
                app.exit = true
            }
        }
    }

    ratatui::restore();
    return Ok(());
}
