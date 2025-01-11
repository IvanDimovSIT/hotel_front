pub struct TextBox {
    text: String,
    max_length: usize,
}
impl TextBox {
    pub fn new(initial_text: String, max_length: usize) -> Self {
        Self {
            text: initial_text,
            max_length,
        }
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn update(&mut self, new_text: String) {
        if new_text.len() > self.max_length {
            return;
        }
        self.text = new_text;
    }
}
