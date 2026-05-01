use ratatui::{
    layout::{HorizontalAlignment, Margin, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
};

use crate::{
    app::{App, Mode},
    tui::utils::get_icon_sidebar,
};

pub fn draw(frame: &mut ratatui::Frame, sidebar_area: Rect, app: &mut App) {
    // Areas ----------
    let mut sidebar_list_area = sidebar_area.inner(Margin {
        horizontal: 0,
        vertical: 1,
    });

    sidebar_list_area.width = sidebar_list_area.width - 1;
    sidebar_list_area.x = sidebar_list_area.x + 1;

    // ----------

    let sidebar_title = format!(" {} ", app.title);

    let mut sidebar_list_status = ListState::default();
    sidebar_list_status.select(app.index_list.sidebar);

    let modifier: Modifier = if app.mode == Mode::SideBar {
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
        .user_dirs
        .clone()
        .into_iter()
        .map(|i| ListItem::new(format!("{} {}", get_icon_sidebar(i.name.as_str()), i.name)))
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
