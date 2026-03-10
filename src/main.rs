use crossterm::event::KeyCode;
mod fs;
mod tui;
mod utils;

struct SideBar<'a> {
    items: Vec<&'a str>,
    ignore: Vec<u64>,
    selected: Option<usize>,
}

struct App<'a> {
    title: &'a str,
    exit: bool,
    is_small: bool,
    sidebar: SideBar<'a>,
}

impl App<'_> {
    fn render(&mut self, frame: &mut ratatui::Frame) {
        let area = frame.area();

        if area.width < 80 || area.height < 18 {
            self.is_small = true;
        } else {
            self.is_small = false
        }

        if self.is_small {
            tui::draw_min_area_warning(frame);
            return;
        }

        tui::draw_sidebar(
            frame,
            self.title,
            self.sidebar.items.clone(),
            self.sidebar.selected,
        );
        tui::draw_file_panel(frame);
        tui::draw_panel(frame);
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut terminal = ratatui::init();

    let mut sidebar = SideBar {
        items: vec![" Home", "󰇅 Desktop", "󰴘 Videos", " Pictures"],
        ignore: vec![],
        selected: Some(0),
    };

    let mut app = App {
        title: "FFile",
        exit: false,
        is_small: false,
        sidebar,
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
