use ratatui::{
    layout::{HorizontalAlignment, Margin, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, TitlePosition},
};

use crate::{
    app::{App, Panel},
    tui::utils::truncate_with_ellipsis,
};

pub fn render_metadata(frame: &mut ratatui::Frame, metadata_area: Rect, app: &mut App) {
    let mut panel_area = metadata_area;

    panel_area.width = panel_area.width - 1;
    panel_area.x = panel_area.x + 1;

    let mut metadata_list_area = panel_area.inner(Margin {
        horizontal: 2,
        vertical: 0,
    });

    metadata_list_area.height = metadata_list_area.height - 1;

    let mut selection_indicator_area = panel_area.inner(Margin {
        horizontal: 2,
        vertical: 0,
    });

    selection_indicator_area.width = 9;
    selection_indicator_area.y = selection_indicator_area.y + 4;
    selection_indicator_area.height = 1;
    selection_indicator_area.x =
        selection_indicator_area.x + metadata_list_area.width - selection_indicator_area.width;

    let mut selection_indicator: String = "".to_string();

    let focus_modifier: Modifier = if app.active_panel == Panel::Metadata {
        Modifier::BOLD
    } else {
        Modifier::DIM
    };

    let metadata_block = Block::new()
        .borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT)
        .border_type(BorderType::Rounded)
        .border_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(focus_modifier),
        )
        .title("Metadata")
        .title_position(TitlePosition::Bottom)
        .title_alignment(HorizontalAlignment::Center);

    frame.render_widget(metadata_block, panel_area);

    if let Some(metadata_entries) = &app.explorer.metadata {
        let metadata_items: Vec<ListItem> = metadata_entries
            .into_iter()
            .map(|(name, value)| {
                ListItem::new(truncate_with_ellipsis(
                    metadata_list_area.width as usize,
                    format!("{}: {}", name, value),
                ))
            })
            .collect();

        let mut list_state = ListState::default();
        if app.panels[Panel::Metadata].valid {
            list_state.select(Some(app.panels[Panel::Metadata].selected));
        } else {
            list_state.select(None);
        }

        if app.panels[Panel::Metadata].valid {
            let selected_index = app.panels[Panel::Metadata].selected;

            selection_indicator =
                format!("═╣ {}/{} ╠═", selected_index + 1, metadata_entries.len());
        } else {
            selection_indicator = format!("═╣ 1/{} ╠═", metadata_entries.len());
        }

        let metadata_list = List::new(metadata_items)
            .style(Style::default().add_modifier(Modifier::DIM).fg(Color::Cyan))
            .highlight_style(Style::default().bold().fg(Color::Cyan));

        frame.render_stateful_widget(metadata_list, metadata_list_area, &mut list_state);
    } else {
        let no_metadata_message = Paragraph::new("Without metadata")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::DIM));

        frame.render_widget(no_metadata_message, metadata_list_area);
    }

    let selection_indicator_paragraph = Paragraph::new(selection_indicator);

    frame.render_widget(selection_indicator_paragraph, selection_indicator_area);
}
