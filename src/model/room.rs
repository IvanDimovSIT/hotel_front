use ::serde::{Deserialize, Serialize};
use uuid::{serde, Uuid};

use super::{bathroom_type::BathroomType, bed::Bed};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    pub id: Uuid,
    pub price: i64,
    pub floor: i16,
    pub room_number: String,
    pub bathroom_type: BathroomType,
    pub beds: Vec<Bed>,
}
