use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::app::App;

pub struct InputStyle {
    pub cursor_color: Color,
    pub foreground_color: Color,
    pub text_modifier: Modifier,
}

pub fn render_input(
    frame: &mut ratatui::Frame,
    app: &mut App,
    area: Rect,
    input_style: InputStyle,
) {
    let visible_width = area.width as usize;
    let scroll_offset = if app.input.position >= visible_width {
        app.input.position - visible_width + 1
    } else {
        0
    };

    let cursor_pos = app.input.position.saturating_sub(scroll_offset);
    let visible_chars = app
        .input
        .content
        .iter()
        .skip(scroll_offset)
        .take(visible_width);

    let mut spans: Vec<Span> = visible_chars
        .enumerate()
        .map(|(i, c)| {
            if i == cursor_pos {
                Span::styled(c.to_string(), Style::default().bg(input_style.cursor_color))
            } else {
                Span::styled(
                    c.to_string(),
                    Style::default()
                        .fg(input_style.foreground_color)
                        .add_modifier(input_style.text_modifier),
                )
            }
        })
        .collect();

    if app.input.position == app.input.content.len() {
        spans.push(Span::styled(
            " ",
            Style::default().bg(input_style.cursor_color),
        ))
    }

    let input_paragraph = Paragraph::new(Line::from(spans));

    frame.render_widget(input_paragraph, area);
}
