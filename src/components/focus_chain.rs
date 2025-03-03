use iced::{widget::text_input, Task};

use crate::app::AppMessage;

pub struct FocusChain {
    chain: Vec<&'static str>,
    selected: Option<usize>,
}
impl FocusChain {
    pub fn new(chain: Vec<&'static str>) -> Self {
        assert!(!chain.is_empty());
        Self {
            chain,
            selected: None,
        }
    }

    pub fn set_focus(&mut self, select: Option<&'static str>) {
        if let Some(id) = select {
            let found = self
                .chain
                .iter()
                .enumerate()
                .find(|(_index, id_element)| id == **id_element)
                .map(|(index, _id)| index);
            self.selected = found;
        } else {
            self.selected = None;
        }
    }

    pub fn get_selected(&self) -> Option<&'static str> {
        self.selected.map(|index| self.chain[index])
    }

    pub fn set_next(&mut self) {
        if let Some(index) = self.selected {
            self.selected = Some((index + 1) % self.chain.len());
            println!("Selecting: {}", self.chain[self.selected.unwrap()])
        }
    }

    pub fn set_prev(&mut self) {
        if let Some(index) = self.selected {
            self.selected = Some((index + self.chain.len() - 1) % self.chain.len());
            println!("Selecting: {}", self.chain[self.selected.unwrap()])
        }
    }

    pub fn apply_focus(&self) -> Task<AppMessage> {
        if let Some(index) = self.selected {
            text_input::focus(self.chain[index])
        } else {
            Task::none()
        }
    }
}
