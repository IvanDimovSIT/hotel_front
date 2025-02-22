use iced::widget::{pick_list, PickList};

use crate::model::bed::BedSize;

pub struct BedSizeComboBox {
    selected: BedSize,
    options: [BedSize; 4],
}
impl BedSizeComboBox {
    pub fn new() -> Self {
        let options = [
            BedSize::Single,
            BedSize::SmallDouble,
            BedSize::Double,
            BedSize::KingSize,
        ];
        Self {
            selected: BedSize::Single,
            options,
        }
    }

    pub fn update(&mut self, bed_size: BedSize) {
        self.selected = bed_size;
    }

    pub fn view<'a, 'b, F, Message>(
        &'b self,
        on_selected: F,
    ) -> PickList<BedSize, &[BedSize], BedSize, Message>
    where
        Message: Clone,
        F: Fn(BedSize) -> Message + 'a + 'b,
    {
        pick_list(&self.options, Some(self.selected), on_selected)
    }

    pub fn get_selected(&self) -> BedSize {
        self.selected
    }
}
