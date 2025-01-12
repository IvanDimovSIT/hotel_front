use regex::Regex;

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
