use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BathroomType {
    Private,
    Shared,
}
impl ToString for BathroomType {
    fn to_string(&self) -> String {
        match self {
            BathroomType::Private => "Private",
            BathroomType::Shared => "Shared",
        }
        .to_owned()
    }
}
