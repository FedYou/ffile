mod corners;
mod file_panel;
mod metadata;
mod screen;
mod sidebar;
mod utils;

use super::app::App;
use ratatui::layout::{Constraint, Direction, Layout};

pub fn draw(frame: &mut ratatui::Frame, app: &mut App) {
    if app.is_small {
        screen::draw_warning_small(frame);
        return;
    }

    if app.pending_open_editor.is_some() {
        screen::draw_opening_editor(frame);
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

    sidebar::draw(frame, sidebar_area, app);

    file_panel::draw(frame, file_panel_area, app);

    metadata::draw(frame, metadata_area, app);

    corners::draw(frame, sidebar_area, file_panel_area, metadata_area);
}
