use iced_aw::date_picker::Date;
use serde::{Deserialize, Serialize};

use crate::model::id_card::{self, IdCard, IdCardDto};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddGuestInput {
    pub first_name: String,
    pub last_name: String,
    #[serde(skip)]
    pub date_of_birth_model: Date,
    pub date_of_birth: String,
    pub phone_number: Option<String>,
    #[serde(skip)]
    id_card_model: Option<IdCard>,
    id_card: Option<IdCardDto>,
}
impl AddGuestInput {
    pub fn new(
        first_name: String,
        last_name: String,
        date_of_birth: Date,
        phone_number: Option<String>,
        id_card: Option<IdCard>,
    ) -> Self {
        Self {
            id_card_model: id_card.clone(),
            id_card: id_card.map(|x| x.into()),
            first_name,
            last_name,
            date_of_birth: date_of_birth.to_string(),
            date_of_birth_model: date_of_birth,
            phone_number,
        }
    }
}
