use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum BedSize {
    Single,
    SmallDouble,
    Double,
    KingSize,
}
impl ToString for BedSize {
    fn to_string(&self) -> String {
        match self {
            BedSize::Single => "Single",
            BedSize::SmallDouble => "Small double",
            BedSize::Double => "Double",
            BedSize::KingSize => "King size",
        }
        .to_owned()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bed {
    pub bed_size: BedSize,
    pub count: i16,
}
