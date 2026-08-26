use ratatui::{
    layout::{HorizontalAlignment, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, TitlePosition},
};
use ratatui_braille_bar::BrailleBar;

use crate::{
    app::{App, Panel},
    file_system::utils::bytes_to_size_string,
    process::{Process, ProcessKind, ProcessStatus},
    tui::utils::{milliseconds_to_string, truncate_with_ellipsis},
};

fn build_transfer_stats_span(process: &Process) -> Span<'_> {
    let bytes_progress: String = if let Some(bytes_total) = process.bytes_total
        && let Some(bytes_done) = process.bytes_done
    {
        format!(
            "{}/{}",
            bytes_to_size_string(bytes_done),
            bytes_to_size_string(bytes_total)
        )
    } else {
        String::new()
    };

    let speed = if let Some(speed) = process.speed {
        format!("{}/s", bytes_to_size_string(speed as u64))
    } else {
        String::new()
    };

    let eta = if let Some(eta) = process.eta {
        milliseconds_to_string(eta.as_millis())
    } else {
        String::new()
    };

    Span::styled(
        vec![bytes_progress, speed, eta].join(" "),
        Style::default().italic(),
    )
}

fn build_status_body_message_span(process: &Process, len: usize) -> Span<'_> {
    let duration = if process.status != ProcessStatus::Running
        && let Some(duration) = process.duration
    {
        milliseconds_to_string(duration.as_millis())
    } else {
        String::new()
    };

    let mut message = match process.status {
        ProcessStatus::Running => String::new(),
        ProcessStatus::Finished => format!(
            "Finished in {duration}, copied {} from {} to {}",
            bytes_to_size_string(process.bytes_total.unwrap_or(0)),
            format!(
                "{}/{}",
                process.entries_done.unwrap_or(0),
                process.entries_total.unwrap_or(0)
            ),
            process.pwd.display()
        ),
        ProcessStatus::Failed => format!("There was an error copying to {}", process.pwd.display()),
        ProcessStatus::Cancelled => {
            format!("Copying was canceled: {}", process.pwd.display())
        }
    };

    message = truncate_with_ellipsis(len, message);

    Span::styled(message, Style::default().italic())
}

fn build_copy_process_item(
    inner_area: Rect,
    process: &Process,
    status_color: Color,
    is_selected: bool,
) -> ListItem<'_> {
    let process_icon: &str = "";
    let progress = process.progress.unwrap_or(0.0);

    let selection_indicator_top: Span = if is_selected {
        Span::styled("  ", Style::default().fg(Color::Yellow))
    } else {
        Span::raw("   ")
    };

    let selection_indicator_center: Span = if is_selected {
        Span::styled("❯ ", Style::default().fg(Color::Yellow))
    } else {
        Span::raw("   ")
    };

    let selection_indicator_bottom: Span = if is_selected {
        Span::styled("  ", Style::default().fg(Color::Yellow))
    } else {
        Span::raw("   ")
    };

    let message_len: usize = if is_selected {
        (inner_area.width - 2 - 6 - 6) as usize
    } else {
        (inner_area.width - 3 - 6 - 6) as usize
    };

    // --

    let header_spans: Vec<Span> = vec![
        selection_indicator_top,
        Span::styled("╭───╮ ", Style::default().fg(status_color)),
        // Message
        Span::styled(
            truncate_with_ellipsis(
                message_len,
                process.message.clone().unwrap_or(String::new()),
            ),
            Style::default().italic(),
        ),
    ];

    let mut body_spans: Vec<Span> = vec![
        selection_indicator_center,
        Span::styled(
            format!("│ {process_icon} │ "),
            Style::default().fg(status_color),
        ),
        Span::raw(if progress > 0.0 {
            format!("{:.0}% ", progress)
        } else {
            String::new()
        }),
    ];

    let mut footer_spans: Vec<Span> = vec![
        selection_indicator_bottom,
        Span::styled("╰───╯", Style::default().fg(status_color)),
    ];

    if process.status == ProcessStatus::Running {
        let mut progress_bar = BrailleBar::new(progress, 100.0)
            .empty_color(Color::White)
            .fill_color(Color::Yellow)
            .into_line(30);
        body_spans.append(&mut progress_bar.spans);
        footer_spans.push(build_transfer_stats_span(process));
    } else {
        body_spans.push(build_status_body_message_span(process, message_len));
    }

    // --

    let header_line = Line::from(header_spans);
    let body_line = Line::from(body_spans);
    let footer_line = Line::from(footer_spans).style(Style::default().italic());

    ListItem::new(vec![header_line, body_line, footer_line])
        .style(Style::default().fg(Color::Gray).dim())
}

///
/// Render
///

pub fn render_process_panel(frame: &mut ratatui::Frame, process_area: Rect, app: &mut App) {
    let inner_area = process_area.inner(Margin {
        horizontal: 1,
        vertical: 1,
    });

    let panel_block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Yellow))
        .title(" Process ")
        .title_position(TitlePosition::Top)
        .title_alignment(HorizontalAlignment::Center);

    let items: Vec<ListItem> = app
        .processes
        .processes
        .iter()
        .enumerate()
        .map(|(index, (_, process))| {
            let status_color: Color = match process.status {
                ProcessStatus::Running | ProcessStatus::Finished => Color::Green,
                ProcessStatus::Failed | ProcessStatus::Cancelled => Color::Red,
            };

            let is_selected: bool = app.panels[Panel::Process].selected == index;

            match process.kind {
                ProcessKind::Copy => {
                    build_copy_process_item(inner_area, process, status_color, is_selected)
                }
            }
        })
        .collect();

    let list = List::new(items)
        .style(
            Style::default()
                .add_modifier(Modifier::DIM)
                .fg(Color::Green),
        )
        .highlight_style(Style::default().bold());

    let mut list_state = ListState::default();

    frame.render_widget(panel_block, process_area);

    if app.panels[Panel::Process].valid {
        list_state.select(Some(app.panels[Panel::Process].selected));
        frame.render_stateful_widget(list, inner_area, &mut list_state);
    } else {
        let empty_message =
            Paragraph::new("Without processes").style(Style::default().fg(Color::Yellow).dim());
        frame.render_widget(empty_message, inner_area);
    }
}
