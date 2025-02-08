use regex::Regex;

use super::{regex_text_box::RegexTextBox, text_box::TextElement};

pub enum NumberType {
    PositiveInteger,
    Price,
}

pub struct NumberTextBox {
    text_box: RegexTextBox,
}
impl NumberTextBox {
    pub fn new<T>(initial_text: T, max_length: usize, number_type: NumberType) -> Self
    where
        T: Into<String>,
    {
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
}
impl TextElement for NumberTextBox {
    fn get_text(&self) -> &str {
        &self.text_box.get_text()
    }

    fn update<T>(&mut self, new_text: T)
    where
        T: Into<String>,
    {
        self.text_box.update(new_text);
    }
}
