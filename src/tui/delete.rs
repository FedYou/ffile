use ratatui::{
    layout::{HorizontalAlignment, Margin},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};

use crate::app::{App, DeleteState, Panel};

pub fn render_delete_popup(frame: &mut ratatui::Frame, app: &mut App) {
    if app.active_panel != Panel::Delete {
        return;
    }

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

    let mut message_area =
        popup_inner_area.centered_horizontally(ratatui::layout::Constraint::Percentage(70));
    message_area.height = 2;
    message_area.y = popup_inner_area.bottom().saturating_sub(4);

    let mut hints_area = popup_inner_area;
    hints_area.height = 1;

    hints_area.y = popup_inner_area.bottom().saturating_sub(1);

    let popup_block = Block::new()
        .title_alignment(HorizontalAlignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));

    if let DeleteState::Error(err) = &app.delete_state {
        let error_message = Paragraph::new(format!("Error while deleting\n{}", err.to_string()))
            .style(Style::default().fg(Color::Red))
            .alignment(HorizontalAlignment::Center);

        let hints_paragraph = Paragraph::new(Line::from(vec![Span::styled(
            "[ESC] Exit",
            Style::default().fg(Color::Red),
        )]))
        .alignment(HorizontalAlignment::Center);

        frame.render_widget(Clear, popup_area);
        frame.render_widget(popup_block, popup_area);
        frame.render_widget(hints_paragraph, hints_area);

        frame.render_widget(error_message, message_area);

        return;
    }

    let message = Paragraph::new("Delete selected item? This cannot be undone.")
        .wrap(Wrap { trim: true })
        .alignment(HorizontalAlignment::Center)
        .style(Style::default().fg(Color::Red).dim());

    let hints_paragraph = Paragraph::new(Line::from(vec![
        Span::styled("[ESC] Cancel", Style::default().fg(Color::Red)),
        Span::raw("   "),
        Span::styled("[ENTER] Delete", Style::default().fg(Color::Red)),
    ]))
    .alignment(HorizontalAlignment::Center);

    frame.render_widget(Clear, popup_area);
    frame.render_widget(popup_block, popup_area);
    frame.render_widget(message, message_area);
    frame.render_widget(hints_paragraph, hints_area);
}
