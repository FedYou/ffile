use crossterm::event::KeyCode;
mod fs;
mod tui;
mod utils;

struct App {
    title: String,
    exit: bool,
    is_small: bool,
}

impl App {
    fn render(&mut self, frame: &mut ratatui::Frame) {
        let term_area = frame.area();

        if term_area.width < 80 || term_area.height < 18 {
            self.is_small = true;
        } else {
            self.is_small = false
        }

        tui::app::draw(frame, self.is_small);
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut terminal = ratatui::init();

    let mut app = App {
        title: "FFile".to_string(),
        exit: false,
        is_small: false,
    };

    loop {
        if app.exit {
            break;
        }

        let _ = terminal.draw(|frame| app.render(frame));

        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            if key.code == KeyCode::Char('q') {
                app.exit = true
            }

            if app.is_small {
                continue;
            }
        }
    }

    ratatui::restore();
    return Ok(());
}
