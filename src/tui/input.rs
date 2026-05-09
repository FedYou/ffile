use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::app::App;

pub struct InputStyles {
    pub cursor: Color,
    pub fg: Color,
    pub modifier: Modifier,
}

pub fn draw_input(
    frame: &mut ratatui::Frame,
    app: &mut App,
    area: Rect,
    input_styles: InputStyles,
) {
    let width = area.width as usize;
    let scroll = if app.input_index >= width {
        app.input_index - width + 1
    } else {
        0
    };

    let cursor = app.input_index.saturating_sub(scroll);
    let visible = app.input.iter().skip(scroll).take(width);

    let mut lines: Vec<Span> = visible
        .enumerate()
        .map(|(i, c)| {
            if i == cursor {
                Span::styled(c.to_string(), Style::default().bg(input_styles.cursor))
            } else {
                Span::styled(
                    c.to_string(),
                    Style::default()
                        .fg(input_styles.fg)
                        .add_modifier(input_styles.modifier),
                )
            }
        })
        .collect();

    if app.input_index == app.input.len() {
        lines.push(Span::styled(" ", Style::default().bg(input_styles.cursor)))
    }

    let widget = Paragraph::new(Line::from(lines));

    frame.render_widget(widget, area);
}
