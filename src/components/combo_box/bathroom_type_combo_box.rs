use iced::widget::{pick_list, PickList};

use crate::model::bathroom_type::BathroomType;

pub struct BathroomTypeComboBox {
    selected: BathroomType,
    options: [BathroomType; 2],
}
impl BathroomTypeComboBox {
    pub fn new() -> Self {
        let options = [BathroomType::Private, BathroomType::Shared];
        Self {
            selected: BathroomType::Private,
            options,
        }
    }

    pub fn update(&mut self, bathroom_type: BathroomType) {
        self.selected = bathroom_type;
    }

    pub fn view<'a, 'b, F, Message>(
        &'b self,
        on_selected: F,
    ) -> PickList<BathroomType, &[BathroomType], BathroomType, Message>
    where
        Message: Clone,
        F: Fn(BathroomType) -> Message + 'a + 'b,
    {
        pick_list(&self.options, Some(self.selected), on_selected)
    }

    pub fn get_selected(&self) -> BathroomType {
        self.selected
    }
}
