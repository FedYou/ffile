use ratatui::{
    layout::{HorizontalAlignment, Margin, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
};

use crate::{
    app::{App, Panel},
    tui::utils::get_sidebar_icon,
};

pub fn render_sidebar(frame: &mut ratatui::Frame, sidebar_area: Rect, app: &mut App) {
    // Areas ----------
    let mut bookmarks_list_area = sidebar_area.inner(Margin {
        horizontal: 0,
        vertical: 1,
    });

    bookmarks_list_area.width = bookmarks_list_area.width - 1;
    bookmarks_list_area.x = bookmarks_list_area.x + 1;

    // ----------

    let sidebar_title = format!(" {} ", app.title);

    let mut bookmarks_list_state = ListState::default();
    bookmarks_list_state.select(Some(app.panels[Panel::SideBar].selected));

    let focus_modifier: Modifier = if app.active_panel == Panel::SideBar {
        Modifier::BOLD
    } else {
        Modifier::DIM
    };

    let sidebar_block = Block::new()
        .title_top(sidebar_title)
        .title_alignment(HorizontalAlignment::Center)
        .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
        .border_type(BorderType::Rounded)
        .border_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(focus_modifier),
        );

    let bookmark_items: Vec<ListItem> = app
        .explorer
        .bookmarks
        .clone()
        .into_iter()
        .map(|(name, _)| ListItem::new(format!("{} {}", get_sidebar_icon(&name), name)))
        .collect();

    let bookmarks_list = List::new(bookmark_items)
        .style(
            Style::default()
                .add_modifier(Modifier::DIM)
                .fg(Color::Yellow),
        )
        .highlight_symbol(" ")
        .highlight_style(
            Style::default()
                .add_modifier(focus_modifier)
                .fg(Color::Yellow),
        );

    // Renderizado
    frame.render_stateful_widget(
        bookmarks_list,
        bookmarks_list_area,
        &mut bookmarks_list_state,
    );
    frame.render_widget(sidebar_block, sidebar_area);
}
