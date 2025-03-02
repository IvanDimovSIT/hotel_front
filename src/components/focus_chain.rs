use iced::{widget::text_input, Task};

use crate::app::AppMessage;

pub struct FocusChain {
    chain: Vec<&'static str>,
    selected: Option<&'static str>,
}
impl FocusChain {
    pub fn new(chain: Vec<&'static str>) -> Self {
        Self {
            chain,
            selected: None,
        }
    }

    pub fn set_focus(&mut self, select: Option<&'static str>) {
        if let Some(id) = select {
            if select.iter().any(|x| *x == id) {
                self.selected = Some(id);
            } else {
                self.selected = None;
            }
        } else {
            self.selected = None;
        }
    }

    pub fn set_next(&mut self) {
        let ind = self.find_selected_index();
        if let Some(index) = ind {
            self.selected = Some(self.chain[(index + 1) % self.chain.len()]);
        }
    }

    pub fn set_prev(&mut self) {
        let ind = self.find_selected_index();
        if let Some(index) = ind {
            self.selected = Some(self.chain[(index + self.chain.len() - 1) % self.chain.len()]);
        }
    }

    pub fn apply_focus(&self) -> Task<AppMessage> {
        if let Some(id) = self.selected {
            text_input::focus(id)
        } else {
            Task::none()
        }
    }

    fn find_selected_index(&self) -> Option<usize> {
        if let Some(id) = self.selected {
            let index = self
                .chain
                .iter()
                .enumerate()
                .find(|(_ind, x)| **x == id)
                .map(|(ind, _x)| ind);

            index
        } else {
            None
        }
    }
}
