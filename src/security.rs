use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum Role {
    User,
    Admin,
}

#[derive(Debug, Deserialize)]
pub struct JwtToken {
    #[serde(skip)]
    pub token_string: String,
    pub role: Role,
}
impl JwtToken {
    pub fn new(token: String) -> Option<Self> {
        let key = DecodingKey::from_secret(&[]);
        let mut validation = Validation::new(Algorithm::HS256);
        validation.insecure_disable_signature_validation();
        validation.validate_exp = false;
        let jwt_token = decode(&token, &key, &validation);

        match jwt_token {
            Ok(ok) => {
                let jwt = Self {
                    token_string: token,
                    ..ok.claims
                };

                Some(jwt)
            }
            Err(err) => {
                println!("Error parsing token:{err}");
                None
            }
        }
    }
}
