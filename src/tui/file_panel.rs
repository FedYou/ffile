use ratatui::{
    layout::{Constraint, Direction, HorizontalAlignment, Layout, Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, BorderType, Borders, HighlightSpacing, List, ListItem, ListState, Paragraph},
};

use crate::{app::App, controller::Mode};

fn draw_file_header(frame: &mut ratatui::Frame, area: Rect, app: &mut App, modifier: Modifier) {
    let path_area = area.inner(Margin {
        horizontal: 1,
        vertical: 1,
    });

    let path = app.controller.current_dir.to_str().unwrap();

    let file_header = Block::new()
        .title_alignment(HorizontalAlignment::Right)
        .borders(Borders::all())
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue).add_modifier(modifier));

    let path_widget = Paragraph::new(
        format!(" 󰉋  {}", path)
            .fg(Color::Blue)
            .add_modifier(modifier),
    );

    frame.render_widget(path_widget, path_area);
    frame.render_widget(file_header, area);
}

pub fn draw(frame: &mut ratatui::Frame, area: Rect, app: &mut App) {
    // Areas ---------
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    let file_panel_area = layout[1];

    let mut content_area = layout[1].inner(Margin {
        horizontal: 1,
        vertical: 0,
    });

    content_area.height = content_area.height - 1;

    let file_header_area = layout[0];

    // ---------

    let content_len = app.controller.current_entries.len();
    let mut list_state = ListState::default();
    list_state.select(app.controller.index_list.file_panel);

    let mut index_position: String = "".to_string();

    if let Some(index) = app.controller.index_list.file_panel {
        if content_len > 0 {
            index_position = format!("═╣ {}/{} ╠═─", index + 1, content_len);
        }
    }

    let modifier: Modifier = if app.controller.mode == Mode::FilePanel {
        Modifier::BOLD
    } else {
        Modifier::DIM
    };

    let file_panel_widget = Block::new()
        .title_bottom(index_position)
        .title_alignment(HorizontalAlignment::Right)
        .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Green).add_modifier(modifier));

    let list_items: Vec<ListItem> = app
        .controller
        .current_entries
        .iter()
        .map(|i| ListItem::new(i.metadata.name.clone()))
        .collect();

    let list_widget = List::new(list_items)
        .style(
            Style::default()
                .add_modifier(Modifier::DIM)
                .fg(Color::Green),
        )
        .highlight_symbol(" ")
        .highlight_style(Style::default().add_modifier(modifier).fg(Color::Green))
        .highlight_spacing(HighlightSpacing::Always);

    let forlder_empty_widget = Paragraph::new("Folder is empty")
        .style(Style::new().fg(Color::Green).add_modifier(Modifier::DIM));

    // Renderizado
    draw_file_header(frame, file_header_area, app, modifier);

    frame.render_widget(file_panel_widget, file_panel_area);

    if content_len > 0 {
        frame.render_stateful_widget(list_widget, content_area, &mut list_state);
        return;
    }
    frame.render_widget(forlder_empty_widget, content_area);
}
