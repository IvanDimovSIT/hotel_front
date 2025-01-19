use regex::Regex;

use crate::model::bed::Bed;

const MIN_FLOOR: i16 = 1;
const MAX_FLOOR: i16 = 100;

const MIN_PRICE: i64 = 1;
const MAX_PRICE: i64 = 100_000_00;

const MIN_BED_COUNT: i16 = 1;
const MAX_BED_COUNT: i16 = 9;

#[derive(Debug)]
pub struct Validator {
    email_regex: Regex,
    password_regex: Regex,
}
impl Validator {
    pub fn validate_email(&self, email: &str) -> Result<(), String> {
        if self.email_regex.is_match(email) {
            Ok(())
        } else {
            Err("Invalid email".to_owned())
        }
    }

    pub fn validate_password(&self, password: &str) -> Result<(), String> {
        if self.password_regex.is_match(password) {
            Ok(())
        } else {
            Err("Invalid password".to_owned())
        }
    }

    pub fn validate_price(price: i64) -> Result<(), String> {
        if (MIN_PRICE..=MAX_PRICE).contains(&price) {
            Ok(())
        } else {
            Err("Invalid price".to_owned())
        }
    }

    pub fn validate_floor(floor: i16) -> Result<(), String> {
        if (MIN_FLOOR..=MAX_FLOOR).contains(&floor) {
            Ok(())
        } else {
            Err("Invalid floor".to_owned())
        }
    }

    pub fn validate_bed(bed: &Bed) -> Result<(), String> {
        if (MIN_BED_COUNT..=MAX_BED_COUNT).contains(&bed.count) {
            Ok(())
        } else {
            Err("Invalid bed count".to_owned())
        }
    }
}
impl Default for Validator {
    fn default() -> Self {
        Self {
            email_regex: Regex::new("^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$")
                .expect("Error creating email regex"),
            password_regex: Regex::new("^[a-zA-Z0-9!@#$%^&*(){}]{8,20}$")
                .expect("Error creating password regex"),
        }
    }
}
