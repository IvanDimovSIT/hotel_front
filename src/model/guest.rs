use iced_aw::date_picker::Date;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::string_to_date;

use super::id_card::{IdCard, IdCardDto};

#[derive(Debug, Clone)]
pub struct Guest {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Date,
    pub id_card: Option<IdCard>,
    pub phone_number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuestDto {
    first_name: String,
    last_name: String,
    date_of_birth: String,
    id_card: Option<IdCardDto>,
    phone_number: Option<String>,
}
impl GuestDto {
    pub fn convert_with_id(self, id: Uuid) -> Guest {
        Guest {
            id,
            first_name: self.first_name,
            last_name: self.last_name,
            date_of_birth: string_to_date(&self.date_of_birth),
            id_card: self.id_card.map(|card| card.into()),
            phone_number: self.phone_number,
        }
    }
}
