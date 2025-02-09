use crate::app::AppMessage;
use iced::advanced::graphics::core::Element;
use iced::widget::button;
use iced::{Renderer, Theme};
use iced_aw::date_picker::{Date, Style};
use iced_aw::helpers::date_picker;
use iced_aw::style::Status;

pub struct DateInput {
    date: Date,
    show_picker: bool,
    on_cancel: AppMessage,
    text: String,
    display_text: String,
}
impl DateInput {
    pub fn new<T>(text: T, date: Date, on_cancel: AppMessage) -> Self
    where
        T: Into<String>,
    {
        let text_owned = text.into();
        let display_text = Self::create_display_text(&text_owned, &date);

        Self {
            date,
            show_picker: false,
            on_cancel,
            text: text_owned,
            display_text,
        }
    }

    fn create_display_text(text: &str, date: &Date) -> String {
        format!("{}:{}", text, date)
    }

    pub fn update_date(&mut self, new_date: Date) {
        self.date = new_date;
        self.display_text = Self::create_display_text(&self.text, &self.date);
    }

    pub fn toggle_show(&mut self) {
        self.show_picker = !self.show_picker;
    }

    pub fn view<F>(&self, on_submit: F) -> Element<'_, AppMessage, Theme, Renderer>
    where
        F: 'static + Fn(Date) -> AppMessage,
    {
        date_picker(
            self.show_picker,
            self.date,
            <iced::widget::Button<'_, AppMessage, Theme, Renderer> as Into<
                Element<'_, AppMessage, Theme, iced::Renderer>,
            >>::into(
                button(self.display_text.as_ref()).on_press(self.on_cancel.clone())
            ),
            self.on_cancel.clone(),
            on_submit,
        )
        //.style(style)
        .font_size(14)
        .into()
    }

    pub fn get_date(&self) -> Date {
        self.date.clone()
    }
}
