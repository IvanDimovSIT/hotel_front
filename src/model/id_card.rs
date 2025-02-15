use iced_aw::date_picker::Date;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct IdCard {
    pub ucn: String,
    pub id_card_number: String,
    pub issue_authority: String,
    pub issue_date: Date,
    pub validity_date: Date,
}
impl Into<IdCardDto> for IdCard {
    fn into(self) -> IdCardDto {
        IdCardDto {
            ucn: self.ucn,
            id_card_number: self.id_card_number,
            issue_authority: self.issue_authority,
            issue_date: self.issue_date.to_string(),
            validity: self.validity_date.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdCardDto {
    ucn: String,
    id_card_number: String,
    issue_authority: String,
    issue_date: String,
    validity: String,
}
impl Into<IdCard> for IdCardDto {
    fn into(self) -> IdCard {
        IdCard {
            ucn: self.ucn,
            id_card_number: self.id_card_number,
            issue_authority: self.issue_authority,
            issue_date: string_to_date(&self.issue_date),
            validity_date: string_to_date(&self.validity),
        }
    }
}

fn string_to_date(date_string: &str) -> Date {
    const DATE_COMPONENTS: usize = 3;
    const YEAR_INDEX: usize = 0;
    const MONTH_INDEX: usize = 1;
    const DAY_INDEX: usize = 2;
    let parts: Vec<_> = date_string.split("-").collect();
    
    if parts.len() != DATE_COMPONENTS {
        println!("Invalid date: '{date_string}'");
        return Date::default();
    }

    let year = parts[YEAR_INDEX].parse().unwrap_or_default();

    let month = parts[MONTH_INDEX].parse().unwrap_or_default();

    let day = parts[DAY_INDEX].parse().unwrap_or_default();

    Date::from_ymd(year, month, day)
}
