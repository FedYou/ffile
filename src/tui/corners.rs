use ratatui::{layout::Rect, text::ToText};

struct Corner {
    symbol: char,
    area: Rect,
}

pub fn draw(frame: &mut ratatui::Frame) {
    let term_area = frame.area();

    let corners: Vec<Corner> = vec![
        // Izquierda arriba del file_panel para unirse con el sideabr
        Corner {
            symbol: '┤',
            area: Rect {
                x: 18,
                y: 1,
                width: 1,
                height: 1,
            },
        },
        // Izquierda abajo del file_panel para unirse con el sideabr
        Corner {
            symbol: '┤',
            area: Rect {
                x: 18,
                y: term_area.height - 7,
                width: 1,
                height: 1,
            },
        },
        // Izquierda del header del file_panel
        Corner {
            symbol: '├',
            area: Rect {
                x: 18,
                y: 2,
                width: 1,
                height: 1,
            },
        },
        // Derecha del header del file_panel
        Corner {
            symbol: '┤',
            area: Rect {
                x: term_area.width - 1,
                y: 2,
                width: 1,
                height: 1,
            },
        },
    ];

    for c in corners {
        frame.render_widget(c.symbol.to_text(), c.area);
    }
}
