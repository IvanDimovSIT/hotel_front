pub const REFRESH_TOKEN_PERIOD: u64 = 30;

pub const MAX_PASSWORD_LENGTH: usize = 24;
pub const MAX_EMAIL_LENGTH: usize = 40;

pub const BASE_URL: &str = "http://localhost:8080/";
pub const LOGIN_PATH: &str = "auth/login";
pub const ADD_ROOM_PATH: &str = "room";
pub const ADD_GUEST_PATH: &str = "guest";
pub const REFRESH_TOKEN_PATH: &str = "auth/refresh";
pub const LOGOUT_PATH: &str = "auth/logout";
pub const REGISTER_PATH: &str = "auth/register";
pub const SEND_OTP_PATH: &str = "auth/send-otp";
