use regex::Regex;

use super::regex_text_box::RegexTextBox;

pub struct RoomNumberTextBox {
    text_box: RegexTextBox,
}
impl RoomNumberTextBox {
    pub fn new<T>(initial_text: T) -> Self
    where
        T: Into<String>,
    {
        let regex = Regex::new("^(\\d*[A-Za-z]?)?$").expect("Error creating room number text box");
        Self {
            text_box: RegexTextBox::new(initial_text, 6, regex),
        }
    }

    pub fn get_text(&self) -> &str {
        &self.text_box.get_text()
    }

    pub fn update<T>(&mut self, new_text: T)
    where
        T: Into<String>,
    {
        self.text_box.update(new_text.into().to_uppercase());
    }
}
