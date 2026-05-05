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

fn draw_screen(frame: &mut ratatui::Frame, widget: Paragraph) {
    let term_area = frame.area();

    let rect = Rect {
        x: 0,
        y: term_area.height / 2 - 1,
        width: term_area.width,
        height: term_area.height,
    };

    frame.render_widget(widget.centered(), rect);
}

pub fn draw_warning_small(frame: &mut ratatui::Frame) {
    let term_area = frame.area();

    let mut current_width_color: Color = Color::Red;
    let mut current_height_color: Color = Color::Red;

    if MIN_WIDTH_APP <= term_area.width {
        current_width_color = Color::Green
    }
    if MIN_HEIGHT_APP <= term_area.height {
        current_height_color = Color::Green
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
                term_area.width.to_string(),
                Style::default().fg(current_width_color),
            ),
            Span::raw(" "),
            Span::styled("Current Height", Style::default().fg(Color::Yellow)),
            Span::raw("="),
            Span::styled(
                term_area.height.to_string(),
                Style::default().fg(current_height_color),
            ),
        ]),
    ];
    let widget = Paragraph::new(lines);

    draw_screen(frame, widget);
}

pub fn draw_opening_editor(frame: &mut ratatui::Frame) {
    let config = get_config().lock().unwrap();

    let widget = Paragraph::new(format!("Opening '{}' editor...", config.editor.clone()))
        .style(Style::default().fg(Color::Green));

    draw_screen(frame, widget);
}
