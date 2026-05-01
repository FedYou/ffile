use ratatui::{
    layout::{HorizontalAlignment, Margin, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, TitlePosition},
};

use crate::{
    app::{App, Mode},
    tui::utils::string_to_elipse,
};

pub fn draw(frame: &mut ratatui::Frame, metadata_area: Rect, app: &mut App) {
    // Areas ---------
    let mut metadata_area = metadata_area;

    metadata_area.width = metadata_area.width - 1;
    metadata_area.x = metadata_area.x + 1;

    let mut metadata_content_area = metadata_area.inner(Margin {
        horizontal: 2,
        vertical: 0,
    });

    metadata_content_area.height = metadata_content_area.height - 1;

    let mut index_position_area = metadata_area.inner(Margin {
        horizontal: 2,
        vertical: 0,
    });

    index_position_area.width = 9;
    index_position_area.y = index_position_area.y + 4;
    index_position_area.height = 1;
    index_position_area.x =
        index_position_area.x + metadata_content_area.width - index_position_area.width;

    // ----------

    let mut index_position: String = "".to_string();

    let modifier: Modifier = if app.mode == Mode::Metadata {
        Modifier::BOLD
    } else {
        Modifier::DIM
    };

    // ---------

    let metadata_widget = Block::new()
        .borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan).add_modifier(modifier))
        .title("Metadata")
        .title_position(TitlePosition::Bottom)
        .title_alignment(HorizontalAlignment::Center);

    frame.render_widget(metadata_widget, metadata_area);

    if let Some(metadata) = &app.current_metadata {
        let metadata_list: Vec<ListItem> = metadata
            .into_iter()
            .map(|m| {
                ListItem::new(string_to_elipse(
                    metadata_content_area.width as usize,
                    format!("{}: {}", m.name, m.value),
                ))
            })
            .collect();

        let mut metadata_list_state = ListState::default();
        metadata_list_state.select(app.index_list.metadata);

        if let Some(index) = app.index_list.metadata {
            index_position = format!("═╣ {}/{} ╠═", index + 1, metadata.len());
        } else {
            index_position = format!("═╣ 1/{} ╠═", metadata.len());
        }

        let metadata_list_widget = List::new(metadata_list)
            .style(Style::default().add_modifier(Modifier::DIM).fg(Color::Cyan))
            .highlight_style(Style::default().bold().fg(Color::Cyan));

        frame.render_stateful_widget(
            metadata_list_widget,
            metadata_content_area,
            &mut metadata_list_state,
        );
    } else {
        let text_widget = Paragraph::new("Without metadata")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::DIM));

        frame.render_widget(text_widget, metadata_content_area);
    }

    let index_position_widget = Paragraph::new(index_position);

    frame.render_widget(index_position_widget, index_position_area);
}
