use regex::Regex;

use super::regex_text_box::RegexTextBox;

pub struct RoomNumberTextBox {
    text_box: RegexTextBox,
}
impl RoomNumberTextBox {
    pub fn new(initial_text: String) -> Self {
        let regex = Regex::new("^(\\d*[A-Za-z]?)?$").expect("Error creating room number text box");
        Self {
            text_box: RegexTextBox::new(initial_text, 6, regex),
        }
    }

    pub fn get_text(&self) -> &str {
        &self.text_box.get_text()
    }

    pub fn update(&mut self, new_text: String) {
        self.text_box.update(new_text.to_uppercase());
    }
}
