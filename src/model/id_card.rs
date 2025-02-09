use iced_aw::date_picker::Date;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct IdCard {
    pub ucn: String,
    pub number: String,
    pub issue_authority: String,
    pub issue_date: Date,
    pub validity_date: Date,
}
impl Into<IdCardDto> for IdCard {
    fn into(self) -> IdCardDto {
        IdCardDto {
            ucn: self.ucn,
            number: self.number,
            issue_authority: self.issue_authority,
            issue_date: self.issue_date.to_string(),
            validity_date: self.validity_date.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdCardDto {
    ucn: String,
    number: String,
    issue_authority: String,
    issue_date: String,
    validity_date: String,
}
impl Into<IdCard> for IdCardDto {
    fn into(self) -> IdCard {
        IdCard {
            ucn: self.ucn,
            number: self.number,
            issue_authority: self.issue_authority,
            issue_date: string_to_date(&self.issue_date),
            validity_date: string_to_date(&self.validity_date),
        }
    }
}

fn string_to_date(date_string: &str) -> Date {
    let parts: Vec<_> = date_string.split("-").collect();
    if parts.len() != 3 {
        return Date::default();
    }

    let year = parts[0].parse().unwrap_or_default();

    let month = parts[1].parse().unwrap_or_default();

    let day = parts[2].parse().unwrap_or_default();

    Date::from_ymd(year, month, day)
}
