use regex::Regex;

use super::regex_text_box::RegexTextBox;

pub enum NumberType {
    PositiveInteger,
    Price,
}

pub struct NumberTextBox {
    text_box: RegexTextBox,
}
impl NumberTextBox {
    pub fn new(initial_text: String, max_length: usize, number_type: NumberType) -> Self {
        let regex = match number_type {
            NumberType::PositiveInteger => {
                Regex::new("^\\d*$").expect("Error creating PositiveInteger regex")
            }
            NumberType::Price => {
                Regex::new("^\\d*(\\.\\d{0,2})?$").expect("Error creating Price regex")
            }
        };
        Self {
            text_box: RegexTextBox::new(initial_text, max_length, regex),
        }
    }

    pub fn get_text(&self) -> &str {
        &self.text_box.get_text()
    }

    pub fn update(&mut self, new_text: String) {
        self.text_box.update(new_text);
    }
}
