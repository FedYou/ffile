use crossterm::event::KeyCode;

fn main() -> Result<(), std::io::Error> {
    let mut terminal = ratatui::init();

    loop {
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    ratatui::restore();
    return Ok(());
}
