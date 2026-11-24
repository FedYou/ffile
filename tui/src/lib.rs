mod clipboard;
mod corners;
mod files;
mod input;
mod metadata;
mod popup;
mod processes;
mod screen;
mod sidebar;
mod utils;

use ffile_core::{
    app::{App, panels::Panel},
    global::{MIN_TERM_HEIGHT, MIN_TERM_WIDTH},
};

use ratatui::layout::{Constraint, Direction, Layout};

pub fn render(frame: &mut ratatui::Frame, app: &mut App) {
    let term_area = frame.area();

    if term_area.width < MIN_TERM_WIDTH || term_area.height < MIN_TERM_HEIGHT {
        app.is_small = true;
    } else {
        app.is_small = false
    }

    if app.is_small {
        screen::render_small_screen_warning(frame);
        return;
    }

    if app.pending_external_actions.open_with_editor.is_some() {
        screen::render_opening_editor(frame);
        return;
    }

    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(18), Constraint::Min(0)])
        .split(frame.area());

    let main_left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(18),
            Constraint::Min(6),
        ])
        .split(main_layout[0]);

    let content_layout = Layout::default()
        .constraints([Constraint::Min(0), Constraint::Length(5)])
        .direction(Direction::Vertical)
        .split(main_layout[1]);

    let footer_layout = Layout::default()
        .constraints([Constraint::Min(0), Constraint::Min(0)])
        .direction(Direction::Horizontal)
        .split(content_layout[1]);

    let sidebar_area = main_left_layout[1];
    let file_panel_area = content_layout[0];
    let metadata_area = footer_layout[0];
    let clipboard_area = footer_layout[1];

    if app.active_panel == Panel::Processes || app.active_panel == Panel::ProcessInfo {
        processes::render_process_panel(frame, frame.area(), app);
        return;
    }

    sidebar::render_sidebar(frame, sidebar_area, app);

    files::render_files(frame, file_panel_area, app);

    metadata::render_metadata(frame, metadata_area, app);

    clipboard::render_clipboard(frame, clipboard_area, app);

    corners::render_panel_corners(frame, sidebar_area, file_panel_area, metadata_area);

    popup::render_popup(frame, app);
}
