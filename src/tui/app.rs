use super::warning;

pub fn draw(frame: &mut ratatui::Frame, is_small: bool) {
    if is_small {
        warning::draw(frame);
        return;
    }
}
