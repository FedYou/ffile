use ratatui::{layout::Rect, text::ToText};

struct Corner {
    symbol: char,
    x: u16,
    y: u16,
}

pub fn draw(
    frame: &mut ratatui::Frame,
    sidebar_area: Rect,
    file_panel_area: Rect,
    metadata_area: Rect,
) {
    let corners: Vec<Corner> = vec![
        // Derecha arriba del file_panel para unirse con el sideabr
        Corner {
            symbol: '┤',
            x: sidebar_area.width,
            y: 1,
        },
        // Derecha abajo del file_panel para unirse con el sideabr
        Corner {
            symbol: '┤',
            x: sidebar_area.width,
            y: sidebar_area.height - 7,
        },
        // Izquierda del header del file_panel
        Corner {
            symbol: '├',
            x: sidebar_area.width,
            y: 2,
        },
        // Derecha del header del file_panel
        Corner {
            symbol: '┤',
            x: sidebar_area.width + file_panel_area.width - 1,
            y: 2,
        },
        // Derecha de la metadata para conectar con el file_panel
        Corner {
            symbol: '┬',
            x: metadata_area.x + 1,
            y: metadata_area.y - 1,
        },
        //  Izquierda de la metadata para conectar con el file_panel
        Corner {
            symbol: '┬',
            x: metadata_area.x + metadata_area.width - 1,
            y: metadata_area.y - 1,
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
