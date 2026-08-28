use ratatui::{
    layout::{HorizontalAlignment, Margin, Rect},
    style::{Color, Modifier, Style},
    widgets::{
        Block, BorderType, Borders, HighlightSpacing, List, ListItem, ListState, Paragraph,
        TitlePosition,
    },
};

use crate::{
    app::{App, Panel},
    tui::utils::truncate_with_leading_ellipsis,
};

pub fn render_file_clipboard(frame: &mut ratatui::Frame, clipboard_area: Rect, app: &mut App) {
    let mut panel_area = clipboard_area;

    panel_area.width = panel_area.width - 1;
    panel_area.x = panel_area.x + 1;

    let clipboard_list_area = panel_area.inner(Margin {
        horizontal: 2,
        vertical: 1,
    });

    let mut selection_indicator_area = panel_area.inner(Margin {
        horizontal: 0,
        vertical: 0,
    });

    selection_indicator_area.width = 9;
    selection_indicator_area.y = selection_indicator_area.y + 4;
    selection_indicator_area.height = 1;
    selection_indicator_area.x =
        selection_indicator_area.x - selection_indicator_area.width / 2 + panel_area.width / 2;

    let mut selection_indicator: String = "".to_string();

    let focus_modifier: Modifier = if app.active_panel == Panel::FileClipboard {
        Modifier::BOLD
    } else {
        Modifier::DIM
    };

    let clipboard_block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(focus_modifier),
        )
        .title("Clipboard")
        .title_position(TitlePosition::Top)
        .title_alignment(HorizontalAlignment::Center);

    frame.render_widget(clipboard_block, panel_area);

    if !app.file_clipboard.list.is_empty() {
        let clipboard_items: Vec<ListItem> = app
            .file_clipboard
            .list
            .iter()
            .map(|i| {
                let item_color = if i.exist { Color::Magenta } else { Color::Red };
                let item_modifier = if i.exist {
                    Modifier::ITALIC
                } else {
                    Modifier::DIM
                };

                ListItem::new(truncate_with_leading_ellipsis(
                    clipboard_list_area.width as usize,
                    i.path.to_string_lossy().to_string(),
                ))
                .style(Style::default().fg(item_color).add_modifier(item_modifier))
            })
            .collect();

        let mut list_state = ListState::default();

        if app.panels[Panel::FileClipboard].count > 0 {
            list_state.select(Some(app.panels[Panel::FileClipboard].selected));
        } else {
            list_state.select(None);
        }

        if app.panels[Panel::FileClipboard].count > 0 {
            let selected_index = app.panels[Panel::FileClipboard].selected;

            selection_indicator =
                format!("═╣ {}/{} ╠═", selected_index + 1, app.file_clipboard.len());
        }

        let clipboard_list = List::new(clipboard_items)
            .style(
                Style::default()
                    .add_modifier(Modifier::DIM)
                    .fg(Color::Magenta),
            )
            .highlight_style(
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(focus_modifier),
            )
            .highlight_spacing(HighlightSpacing::Always);

        frame.render_stateful_widget(clipboard_list, clipboard_list_area, &mut list_state);
    } else {
        let void_clipboard_message = Paragraph::new("Void file clipboard").style(
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::DIM),
        );

        frame.render_widget(void_clipboard_message, clipboard_list_area);
    }

    let selection_indicator_paragraph = Paragraph::new(selection_indicator);

    frame.render_widget(selection_indicator_paragraph, selection_indicator_area);
}
