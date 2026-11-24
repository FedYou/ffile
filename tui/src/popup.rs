use ffile_core::app::{App, panels::Panel};
use ratatui::{
    layout::{HorizontalAlignment, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};

use crate::{
    input::{InputStyle, render_input},
    utils::truncate_with_leading_ellipsis,
};

pub fn get_create_header(app: &mut App, header_area: Rect) -> String {
    let target_path = format!(
        "{}/{}",
        app.explorer.pwd.clone().to_str().unwrap(),
        app.input.to_string()
    );

    let target_icon = if target_path.ends_with("/") {
        '\u{f07b}' // 
    } else {
        '\u{f15b}' // 
    };

    format!(
        "{} {}",
        target_icon,
        truncate_with_leading_ellipsis(header_area.width as usize - 2, target_path)
    )
}

pub fn render_popup(frame: &mut ratatui::Frame, app: &mut App) {
    if app.active_panel != Panel::New && app.active_panel != Panel::Delete {
        return;
    }

    let message = match app.active_panel {
        Panel::Delete => "Delete selected item? This cannot be undone.",
        _ => "",
    };

    let popup_color = match app.active_panel {
        Panel::Delete => Color::Red,
        _ => Color::Green,
    };

    let mut popup_area = frame.area().centered(
        ratatui::layout::Constraint::Length(50),
        ratatui::layout::Constraint::Length(7),
    );

    popup_area.width = 50;
    popup_area.height = 7;

    let popup_inner_area = popup_area.inner(Margin {
        horizontal: 1,
        vertical: 1,
    });

    let mut header_area =
        popup_inner_area.centered_horizontally(ratatui::layout::Constraint::Percentage(90));
    header_area.height = 1;
    header_area.y = popup_inner_area.bottom().saturating_sub(5);

    let mut center_area =
        popup_inner_area.centered_horizontally(ratatui::layout::Constraint::Percentage(70));
    center_area.height = 2;
    center_area.y = popup_inner_area
        .bottom()
        .saturating_sub(if message.len() > 36 { 4 } else { 3 });
    // 50 - 14 para añadir el margin del contenido del centre

    let mut hints_area = popup_inner_area;
    hints_area.height = 1;

    hints_area.y = popup_inner_area.bottom().saturating_sub(1);

    let popup_block = Block::new()
        .title_alignment(HorizontalAlignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(
            Style::default()
                .fg(popup_color)
                .add_modifier(Modifier::BOLD),
        );

    let hints_paragraph = Paragraph::new(Line::from(vec![
        Span::styled("[ESC] Cancel", Style::default().fg(popup_color)),
        Span::raw("   "),
        Span::styled("[ENTER] Accept", Style::default().fg(popup_color)),
    ]))
    .alignment(HorizontalAlignment::Center);

    frame.render_widget(Clear, popup_area);
    frame.render_widget(popup_block, popup_area);
    frame.render_widget(hints_paragraph, hints_area);

    if app.active_panel == Panel::New {
        let header_paragraph = Paragraph::new(get_create_header(app, header_area))
            .style(Style::default().fg(Color::Blue));
        frame.render_widget(header_paragraph, header_area);
    }

    if app.active_panel == Panel::New {
        render_input(
            frame,
            app,
            center_area,
            InputStyle {
                cursor_color: Color::Yellow,
                foreground_color: Color::White,
                text_modifier: Modifier::DIM,
            },
        );
    } else {
        let message_widget = Paragraph::new(message)
            .wrap(Wrap { trim: true })
            .alignment(HorizontalAlignment::Center)
            .style(Style::default().fg(popup_color).dim());

        frame.render_widget(message_widget, center_area);
    }
}
