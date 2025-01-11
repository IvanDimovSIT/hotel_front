use regex::Regex;

pub struct RegexTextBox {
    text: String,
    max_length: usize,
    regex: Regex,
}
impl RegexTextBox {
    pub fn new(initial_text: String, max_length: usize, regex: Regex) -> Self {
        Self {
            text: initial_text,
            max_length,
            regex,
        }
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn update(&mut self, new_text: String) {
        if new_text.len() > self.max_length {
            return;
        }
        if self.regex.is_match(&new_text) {
            self.text = new_text;
        }
    }
}
