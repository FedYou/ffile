use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{
    config::get_config,
    global::{MIN_HEIGHT_APP, MIN_WIDTH_APP},
};

fn render_centered_message(frame: &mut ratatui::Frame, message: Paragraph) {
    let terminal_area = frame.area();

    let message_rect = Rect {
        x: 0,
        y: terminal_area.height / 2 - 1,
        width: terminal_area.width,
        height: terminal_area.height,
    };

    frame.render_widget(message.centered(), message_rect);
}

pub fn render_small_screen_warning(frame: &mut ratatui::Frame) {
    let terminal_area = frame.area();

    let mut width_status_color: Color = Color::Red;
    let mut height_status_color: Color = Color::Red;

    if MIN_WIDTH_APP <= terminal_area.width {
        width_status_color = Color::Green
    }
    if MIN_HEIGHT_APP <= terminal_area.height {
        height_status_color = Color::Green
    }

    let lines = vec![
        Line::from(vec![
            Span::styled("Min Width", Style::default().fg(Color::Yellow)),
            Span::raw("="),
            Span::styled(
                MIN_WIDTH_APP.clone().to_string(),
                Style::default().fg(Color::Green),
            ),
            Span::raw(" "),
            Span::styled("Min Height", Style::default().fg(Color::Yellow)),
            Span::raw("="),
            Span::styled(
                MIN_HEIGHT_APP.clone().to_string(),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(vec![
            Span::styled("Current Width", Style::default().fg(Color::Yellow)),
            Span::raw("="),
            Span::styled(
                terminal_area.width.to_string(),
                Style::default().fg(width_status_color),
            ),
            Span::raw(" "),
            Span::styled("Current Height", Style::default().fg(Color::Yellow)),
            Span::raw("="),
            Span::styled(
                terminal_area.height.to_string(),
                Style::default().fg(height_status_color),
            ),
        ]),
    ];
    let message = Paragraph::new(lines);

    render_centered_message(frame, message);
}

pub fn render_opening_editor(frame: &mut ratatui::Frame) {
    let config = get_config().lock().unwrap();

    let message = Paragraph::new(format!("Opening '{}' editor...", config.editor.clone()))
        .style(Style::default().fg(Color::Green));

    render_centered_message(frame, message);
}
