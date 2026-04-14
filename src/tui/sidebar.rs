use ratatui::{
    layout::{Constraint, Direction, HorizontalAlignment, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
};

use crate::{app::App, controller::Mode};

pub fn draw(frame: &mut ratatui::Frame, area: Rect, app: &mut App) {
    // Areas ----------
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(18),
            Constraint::Min(6),
        ])
        .split(area);

    let sidebar_area = layout[1];
    let mut sidebar_list_area = layout[1].inner(Margin {
        horizontal: 0,
        vertical: 1,
    });

    sidebar_list_area.width = sidebar_list_area.width - 1;
    sidebar_list_area.x = sidebar_list_area.x + 1;

    // ----------

    let sidebar_title = format!(" {} ", app.title);

    let mut sidebar_list_status = ListState::default();
    sidebar_list_status.select(app.controller.index_list.sidebar);

    let modifier: Modifier = if app.controller.mode == Mode::SideBar {
        Modifier::BOLD
    } else {
        Modifier::DIM
    };

    let sidebar_widget = Block::new()
        .title_top(sidebar_title)
        .title_alignment(HorizontalAlignment::Center)
        .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Yellow).add_modifier(modifier));

    let sidebar_list: Vec<ListItem> = app
        .controller
        .user_dir
        .clone()
        .into_iter()
        .map(|i| ListItem::new(i.name))
        .collect();

    let sidebar_list_widget = List::new(sidebar_list)
        .style(
            Style::default()
                .add_modifier(Modifier::DIM)
                .fg(Color::Yellow),
        )
        .highlight_symbol(" ")
        .highlight_style(Style::default().add_modifier(modifier).fg(Color::Yellow));

    // Renderizado
    frame.render_stateful_widget(
        sidebar_list_widget,
        sidebar_list_area,
        &mut sidebar_list_status,
    );
    frame.render_widget(sidebar_widget, sidebar_area);
}
