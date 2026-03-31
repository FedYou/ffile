use ratatui::{layout::Rect, widgets::Paragraph};

fn wrire_message(w: u16, h: u16) -> String {
    format!(
        "Min Width=80 Min height=18\nCurrent Width={} Current Height={}",
        w, h
    )
}

pub fn draw(frame: &mut ratatui::Frame) {
    let term_area = frame.area();

    let rect = Rect {
        x: 0,
        y: term_area.height / 2 - 1,
        width: term_area.width,
        height: term_area.height,
    };

    let widget = Paragraph::new(wrire_message(term_area.width, term_area.height)).centered();

    frame.render_widget(widget, rect);
}
