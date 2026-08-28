use ratatui::{
    layout::{Constraint, Direction, HorizontalAlignment, Layout, Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, HighlightSpacing, List, ListItem, ListState, Paragraph},
};

use crate::{
    app::{App, Panel},
    tui::utils::{get_file_icon, get_file_icon_color, truncate_with_leading_ellipsis},
};

fn render_file_header(frame: &mut ratatui::Frame, area: Rect, app: &mut App, modifier: Modifier) {
    let path_area = area.inner(Margin {
        horizontal: 1,
        vertical: 1,
    });

    let path = app.explorer.pwd.to_str().unwrap();

    let file_header = Block::new()
        .title_alignment(HorizontalAlignment::Right)
        .borders(Borders::all())
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue).add_modifier(modifier));

    let path_widget = Paragraph::new(format!(
        "   {}",
        truncate_with_leading_ellipsis((path_area.width - 5).into(), path.to_string())
    ))
    .fg(Color::Blue)
    .add_modifier(modifier);

    frame.render_widget(path_widget, path_area);
    frame.render_widget(file_header, area);
}

pub fn render_file_panel(frame: &mut ratatui::Frame, area: Rect, app: &mut App) {
    let panel_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    let panel_body_area = panel_layout[1];

    let mut content_area = panel_layout[1].inner(Margin {
        horizontal: 1,
        vertical: 0,
    });

    content_area.height = content_area.height - 1;

    let file_header_area = panel_layout[0];

    let entry_count = app.explorer.entries.len();
    let mut list_state = ListState::default();

    let has_valid_selection = app.panels[Panel::FilePanel].valid;

    if has_valid_selection {
        list_state.select(Some(app.panels[Panel::FilePanel].selected));
    } else {
        list_state.select(None);
    }

    let mut selection_indicator: String = "".to_string();

    if has_valid_selection {
        let selected_index = app.panels[Panel::FilePanel].selected;
        if entry_count > 0 {
            selection_indicator = format!("═╣ {}/{} ╠═─", selected_index + 1, entry_count);
        }
    }

    let focus_modifier: Modifier = if app.active_panel == Panel::FilePanel {
        Modifier::BOLD
    } else {
        Modifier::DIM
    };

    let file_panel_block = Block::new()
        .title_bottom(selection_indicator)
        .title_alignment(HorizontalAlignment::Right)
        .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
        .border_type(BorderType::Rounded)
        .border_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(focus_modifier),
        );

    let entry_items: Vec<ListItem> = app
        .explorer
        .entries
        .iter()
        .map(|entry| {
            let entry_icon_is_selected = if app.file_clipboard.exist(&entry.path) {
                " "
            } else {
                ""
            };

            let entry_icon = get_file_icon(
                &entry.metadata.name,
                &entry.metadata.ext.clone().unwrap_or("".to_string()),
                &entry.is_file,
            );
            let icon_color = get_file_icon_color(
                &entry.metadata.name,
                &entry.metadata.ext.clone().unwrap_or("".to_string()),
                &entry.is_file,
            );

            let entry_line = Line::from(vec![
                Span::raw(entry_icon_is_selected),
                Span::styled(
                    entry_icon.to_string(),
                    Style::default().fg(icon_color).add_modifier(Modifier::DIM),
                ),
                Span::raw(" "),
                Span::raw(entry.metadata.name.clone()),
            ]);

            return ListItem::new(entry_line);
        })
        .collect();

    let entry_list = List::new(entry_items)
        .style(
            Style::default()
                .add_modifier(Modifier::DIM)
                .fg(Color::Green),
        )
        .highlight_symbol(" ")
        .highlight_style(Style::default().add_modifier(focus_modifier))
        .highlight_spacing(HighlightSpacing::Always);

    let empty_folder_message = Paragraph::new("Folder is empty")
        .style(Style::new().fg(Color::Green).add_modifier(Modifier::DIM));

    render_file_header(frame, file_header_area, app, focus_modifier);

    frame.render_widget(file_panel_block, panel_body_area);

    if entry_count > 0 {
        frame.render_stateful_widget(entry_list, content_area, &mut list_state);
        return;
    }
    frame.render_widget(empty_folder_message, content_area);
}
