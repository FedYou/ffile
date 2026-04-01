mod warning;

use super::app::App;

pub fn draw(frame: &mut ratatui::Frame, app: &mut App) {
    if app.is_small {
        warning::draw(frame);
        return;
    }
}
