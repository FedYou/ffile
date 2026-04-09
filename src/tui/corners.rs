use ratatui::{layout::Rect, text::ToText};

struct Corner {
    symbol: char,
    x: u16,
    y: u16,
}

pub fn draw(frame: &mut ratatui::Frame) {
    let term_area = frame.area();

    let corners: Vec<Corner> = vec![
        // Izquierda arriba del file_panel para unirse con el sideabr
        Corner {
            symbol: '┤',
            x: 18,
            y: 1,
        },
        // Izquierda abajo del file_panel para unirse con el sideabr
        Corner {
            symbol: '┤',
            x: 18,
            y: term_area.height - 7,
        },
        // Izquierda del header del file_panel
        Corner {
            symbol: '├',
            x: 18,
            y: 2,
        },
        // Derecha del header del file_panel
        Corner {
            symbol: '┤',
            x: term_area.width - 1,
            y: 2,
        },
    ];

    for c in corners {
        frame.render_widget(
            c.symbol.to_text(),
            Rect {
                x: c.x,
                y: c.y,
                width: 1,
                height: 1,
            },
        );
    }
}
