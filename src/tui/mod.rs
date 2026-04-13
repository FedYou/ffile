mod corners;
mod file_panel;
mod metadata;
mod sidebar;
mod warning;

use super::app::App;
use ratatui::layout::{Constraint, Direction, Layout};

pub fn draw(frame: &mut ratatui::Frame, app: &mut App) {
    if app.is_small {
        warning::draw(frame);
        return;
    }

    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(18), Constraint::Min(0)])
        .split(frame.area());

    let content_layout = Layout::default()
        .constraints([Constraint::Min(0), Constraint::Length(5)])
        .direction(Direction::Vertical)
        .split(main_layout[1]);

    let footer_layout = Layout::default()
        .constraints([Constraint::Min(0), Constraint::Min(0)])
        .direction(Direction::Horizontal)
        .split(content_layout[1]);

    let sidebar_area = main_layout[0];
    let file_panel_area = content_layout[0];
    let metadata_area = footer_layout[0];

    sidebar::draw(frame, sidebar_area, app);

    file_panel::draw(frame, file_panel_area, app);

    metadata::draw(frame, metadata_area, app);

    corners::draw(frame, sidebar_area, file_panel_area, metadata_area);
}
