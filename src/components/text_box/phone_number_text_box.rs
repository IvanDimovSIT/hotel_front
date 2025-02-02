use regex::Regex;

use super::regex_text_box::RegexTextBox;

pub struct PhoneNumberTextBox {
    text_box: RegexTextBox,
}
impl PhoneNumberTextBox {
    pub fn new<T>(initial_text: T) -> Self
    where
        T: Into<String>,
    {
        let regex =
            Regex::new("^(\\+[0-9]{0,15})?$").expect("Error creating phone number text box");
        Self {
            text_box: RegexTextBox::new(initial_text, 15, regex),
        }
    }

    pub fn get_text(&self) -> &str {
        &self.text_box.get_text()
    }

    pub fn update<T>(&mut self, new_text: T)
    where
        T: Into<String>,
    {
        self.text_box.update(new_text);
    }
}
