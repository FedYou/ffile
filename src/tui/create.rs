use ratatui::{
    layout::{HorizontalAlignment, Margin},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

use crate::{
    app::{App, Mode},
    tui::{
        input::{draw_input, InputStyles},
        utils::string_to_elipse_inverse,
    },
};

pub fn draw(frame: &mut ratatui::Frame, app: &mut App) {
    if app.mode != Mode::Create {
        return;
    }

    // Areas ---------

    let mut popup_area = frame.area().centered(
        ratatui::layout::Constraint::Length(50),
        ratatui::layout::Constraint::Length(7),
    );

    popup_area.width = 50;
    popup_area.height = 7;

    let inner = popup_area.inner(Margin {
        horizontal: 1,
        vertical: 1,
    });

    let mut header_area = inner.centered_horizontally(ratatui::layout::Constraint::Percentage(90));
    header_area.height = 1;
    header_area.y = inner.bottom().saturating_sub(5);

    let mut input_area = inner.centered_horizontally(ratatui::layout::Constraint::Percentage(70));
    input_area.height = 1;
    input_area.y = inner.bottom().saturating_sub(3);

    let mut panel_area = inner;
    panel_area.height = 1;

    panel_area.y = inner.bottom().saturating_sub(1);

    // ---------

    let popup_widget = Block::new()
        .title_alignment(HorizontalAlignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        );

    if app.create_error {
        let error_widget = Paragraph::new("Error while creating")
            .style(Style::default().fg(Color::Green))
            .alignment(HorizontalAlignment::Center);

        let panel_widget = Paragraph::new(Line::from(vec![Span::styled(
            "[ESC] Exit",
            Style::default().fg(Color::Red),
        )]))
        .alignment(HorizontalAlignment::Center);

        frame.render_widget(Clear, popup_area);
        frame.render_widget(popup_widget, popup_area);
        frame.render_widget(panel_widget, panel_area);

        frame.render_widget(error_widget, input_area);

        return;
    }

    let header_path = format!(
        "{}/{}",
        app.current_dir.clone().to_str().unwrap(),
        app.get_input_to_string()
    );

    let header_icon = if header_path.ends_with("/") {
        '\u{f07b}' // 
    } else {
        '\u{f15b}' // 
    };

    let header_content = format!(
        "{} {}",
        header_icon,
        string_to_elipse_inverse(header_area.width as usize - 2, header_path)
    );

    let header_widget = Paragraph::new(header_content).style(Style::default().fg(Color::Blue));

    let panel_widget = Paragraph::new(Line::from(vec![
        Span::styled("[ESC] Cancel", Style::default().fg(Color::Red)),
        Span::raw("   "),
        Span::styled("[ENTER] Create", Style::default().fg(Color::Green)),
    ]))
    .alignment(HorizontalAlignment::Center);

    frame.render_widget(Clear, popup_area);
    frame.render_widget(popup_widget, popup_area);
    frame.render_widget(header_widget, header_area);
    frame.render_widget(panel_widget, panel_area);

    draw_input(
        frame,
        app,
        input_area,
        InputStyles {
            cursor: Color::Yellow,
            fg: Color::White,
            modifier: Modifier::DIM,
        },
    );
}
