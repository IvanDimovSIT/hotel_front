use regex::Regex;

use super::{regex_text_box::RegexTextBox, text_box::TextElement};

pub struct IdCardNumberTextBox {
    text_box: RegexTextBox,
}
impl IdCardNumberTextBox {
    pub fn new<T>(initial_text: T) -> Self
    where
        T: Into<String>,
    {
        let regex = Regex::new("^[0-9]*$").expect("Error creating id card number text box");
        Self {
            text_box: RegexTextBox::new(initial_text, 10, regex),
        }
    }
}
impl TextElement for IdCardNumberTextBox {
    fn get_text(&self) -> &str {
        &self.text_box.get_text()
    }

    fn update<T>(&mut self, new_text: T)
    where
        T: Into<String>,
    {
        self.text_box.update(new_text.into().to_uppercase());
    }
}
