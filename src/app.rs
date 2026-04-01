use crate::controller::{Controller, IndexAction};
use crate::keyboard::Action;
use crate::tui;

pub struct App {
    pub title: String,
    pub exit: bool,
    pub is_small: bool,
    pub controller: Controller,
}

impl App {
    pub fn render(&mut self, frame: &mut ratatui::Frame) {
        let term_area = frame.area();

        if term_area.width < 80 || term_area.height < 18 {
            self.is_small = true;
        } else {
            self.is_small = false
        }

        tui::draw(frame, self);
    }

    pub fn execute_action(&mut self, action: Action) {
        if action == Action::Exit {
            self.exit = true
        }

        if self.is_small {
            return;
        }

        match action {
            Action::Up => {
                self.controller.change_index_vertical(IndexAction::Decrease);
            }
            Action::Down => {
                self.controller.change_index_vertical(IndexAction::Increase);
            }
            _ => {}
        }
    }
}
