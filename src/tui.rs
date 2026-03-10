use ratatui::{
    layout::{HorizontalAlignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
};

pub fn draw_min_area_warning(frame: &mut ratatui::Frame) {
    let terminal_area = frame.area();

    let text_area = Rect {
        x: 0,
        y: terminal_area.height / 2 - 1,
        width: terminal_area.width,
        height: terminal_area.height,
    };

    let content = format!(
        "Min Width=80 Min height=18\nCurrent Width={} Current Height={}",
        terminal_area.width, terminal_area.height
    );

    let warning_widget = Paragraph::new(content).centered();

    frame.render_widget(warning_widget, text_area);
}

pub fn draw_sidebar(
    frame: &mut ratatui::Frame,
    title: &str,
    items: Vec<&str>,
    selected: Option<usize>,
) {
    let terminal_area = frame.area();
    let sidebar_title = format!(" {title} ");

    let sidebar_list: Vec<ListItem> = items.into_iter().map(|i| ListItem::new(i)).collect();

    let sidebar_area = Rect {
        x: 0,
        y: 1,
        width: 18,
        height: terminal_area.height - 7,
    };

    let sidebar_list_area = Rect {
        x: 1,
        y: 2,
        width: 17,
        height: terminal_area.height - 8,
    };

    let sidebar_widget = Block::new()
        .title_top(sidebar_title)
        .title_alignment(HorizontalAlignment::Center)
        .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Yellow));

    let sidebar_list_widget = List::new(sidebar_list)
        .style(Style::default().add_modifier(Modifier::DIM))
        .highlight_symbol(" ")
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Yellow),
        );

    let mut sidebar_list_status = ListState::default();
    sidebar_list_status.select(selected);

    frame.render_stateful_widget(
        sidebar_list_widget,
        sidebar_list_area,
        &mut sidebar_list_status,
    );
    frame.render_widget(sidebar_widget, sidebar_area);
}

pub fn draw_file_panel(frame: &mut ratatui::Frame) {
    let terminal_area = frame.area();

    let file_panel_area = Rect {
        x: 18,
        y: 0,
        width: terminal_area.width,
        height: terminal_area.height - 5,
    };

    let file_panel_widget = Block::new()
        .title_bottom("═╣ 0/0 ╠═─")
        .title_alignment(HorizontalAlignment::Right)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Green));

    let file_panel_corner_top_area = Rect {
        x: 18,
        y: 1,
        width: 1,
        height: 1,
    };

    let file_panel_corner_bottom_area = Rect {
        x: 18,
        y: terminal_area.height - 7,
        width: 1,
        height: 1,
    };

    frame.render_widget(file_panel_widget, file_panel_area);
    frame.render_widget("┤", file_panel_corner_top_area);
    frame.render_widget("┤", file_panel_corner_bottom_area);
}

pub fn draw_panel(frame: &mut ratatui::Frame) {
    let terminal_area = frame.area();

    let width = (terminal_area.width - 18) / 2;
    let y = terminal_area.height - 5;

    let metadata_area = Rect {
        x: 19,
        y,
        width: width - 2,
        height: 5,
    };

    let metadata_widget = Block::new()
        .title_bottom(" Metadata ")
        .title_alignment(HorizontalAlignment::Center)
        .borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Green));

    let metadata_corner_left_area = Rect {
        x: 19,
        y: y - 1,
        width: 1,
        height: 1,
    };

    let metadata_corner_right_area = Rect {
        x: 16 + width,
        y: y - 1,
        width: 1,
        height: 1,
    };

    let clipboard_area = Rect {
        x: 19 + width,
        y,
        width: (terminal_area.width - 1) - (18 + width) - 1,
        height: 6,
    };

    let clipboard_widget = Block::new()
        .title_bottom(" Clipboard ")
        .title_alignment(HorizontalAlignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    frame.render_widget(metadata_widget, metadata_area);
    frame.render_widget(clipboard_widget, clipboard_area);
    frame.render_widget("┬", metadata_corner_left_area);
    frame.render_widget("┬", metadata_corner_right_area);
}
