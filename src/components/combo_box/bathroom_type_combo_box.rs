use iced::widget::{pick_list, PickList};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BathroomType {
    Private,
    Shared,
}
impl ToString for BathroomType {
    fn to_string(&self) -> String {
        match self {
            BathroomType::Private => "Private".to_owned(),
            BathroomType::Shared => "Shared".to_owned(),
        }
    }
}

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
}
