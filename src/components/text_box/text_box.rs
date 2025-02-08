pub trait TextElement {
    fn get_text(&self) -> &str;
    fn update<T>(&mut self, new_text: T)
    where
        T: Into<String>;
}

pub struct TextBox {
    text: String,
    max_length: usize,
}
impl TextBox {
    pub fn new<T>(initial_text: T, max_length: usize) -> Self
    where
        T: Into<String>,
    {
        Self {
            text: initial_text.into(),
            max_length,
        }
    }
}
impl TextElement for TextBox {
    fn get_text(&self) -> &str {
        &self.text
    }

    fn update<T>(&mut self, new_text: T)
    where
        T: Into<String>,
    {
        let text = new_text.into();
        if text.len() > self.max_length {
            return;
        }
        self.text = text;
    }
}
