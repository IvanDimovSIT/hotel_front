use regex::Regex;

pub struct RegexTextBox {
    text: String,
    max_length: usize,
    regex: Regex,
}
impl RegexTextBox {
    pub fn new<T>(initial_text: T, max_length: usize, regex: Regex) -> Self
    where
        T: Into<String>,
    {
        Self {
            text: initial_text.into(),
            max_length,
            regex,
        }
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn update<T>(&mut self, new_text: T)
    where
        T: Into<String>,
    {
        let text = new_text.into();
        if text.len() > self.max_length {
            return;
        }
        if self.regex.is_match(&text) {
            self.text = text;
        }
    }
}
