use ratatui::{
    layout::{HorizontalAlignment, Margin},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

use crate::{
    app::{App, CreateState, Panel},
    tui::{
        input::{InputStyle, render_input},
        utils::truncate_with_leading_ellipsis,
    },
};

pub fn render_create_popup(frame: &mut ratatui::Frame, app: &mut App) {
    if app.active_panel != Panel::Create {
        return;
    }

    // Areas ---------

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

    let mut input_area =
        popup_inner_area.centered_horizontally(ratatui::layout::Constraint::Percentage(70));
    input_area.height = 2;
    input_area.y = popup_inner_area.bottom().saturating_sub(3);

    let mut hints_area = popup_inner_area;
    hints_area.height = 1;

    hints_area.y = popup_inner_area.bottom().saturating_sub(1);

    // ---------

    let popup_block = Block::new()
        .title_alignment(HorizontalAlignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        );

    if let CreateState::Error(err) = &app.create_state {
        let error_message = Paragraph::new(format!("Error while creating\n{}", err.to_string()))
            .style(Style::default().fg(Color::Green))
            .alignment(HorizontalAlignment::Center);

        let hints_paragraph = Paragraph::new(Line::from(vec![Span::styled(
            "[ESC] Exit",
            Style::default().fg(Color::Red),
        )]))
        .alignment(HorizontalAlignment::Center);

        frame.render_widget(Clear, popup_area);
        frame.render_widget(popup_block, popup_area);
        frame.render_widget(hints_paragraph, hints_area);

        frame.render_widget(error_message, input_area);

        return;
    }

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

    let header_text = format!(
        "{} {}",
        target_icon,
        truncate_with_leading_ellipsis(header_area.width as usize - 2, target_path)
    );

    let header_paragraph = Paragraph::new(header_text).style(Style::default().fg(Color::Blue));

    let hints_paragraph = Paragraph::new(Line::from(vec![
        Span::styled("[ESC] Cancel", Style::default().fg(Color::Red)),
        Span::raw("   "),
        Span::styled("[ENTER] Create", Style::default().fg(Color::Green)),
    ]))
    .alignment(HorizontalAlignment::Center);

    frame.render_widget(Clear, popup_area);
    frame.render_widget(popup_block, popup_area);
    frame.render_widget(header_paragraph, header_area);
    frame.render_widget(hints_paragraph, hints_area);

    render_input(
        frame,
        app,
        input_area,
        InputStyle {
            cursor_color: Color::Yellow,
            foreground_color: Color::White,
            text_modifier: Modifier::DIM,
        },
    );
}
