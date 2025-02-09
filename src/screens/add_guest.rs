use std::sync::{Arc, Mutex};

use iced::{
    widget::{button, column, scrollable, text, text_input},
    Alignment::Center,
    Element,
    Length::Fill,
    Task,
};
use iced_aw::date_picker::Date;

use crate::{
    app::{AppMessage, GlobalState, Screen},
    components::{
        checkbox::Checkbox,
        date_input::DateInput,
        text_box::{
            id_card_number_text_box::IdCardNumberTextBox,
            phone_number_text_box::PhoneNumberTextBox,
            text_box::{TextBox, TextElement},
            ucn_text_box::UcnTextBox,
        },
    },
    model::id_card::IdCard,
    services::add_guest::AddGuestInput,
    styles::{ERROR_COLOR, FORM_PADDING, FORM_SPACING, TEXT_BOX_WIDTH, TITLE_FONT_SIZE},
};

#[derive(Debug, Clone)]
pub enum AddGuestMessage {
    ChangeFirstName(String),
    ChangeLastName(String),
    ChangeCheckbox(bool),
    ChangePhoneNumber(String),
    ChangeUcn(String),
    ChangeIdCardNumber(String),
    ChangeIdCardIssueAuthority(String),
    ToggleShowIssueDate,
    ChangeIdCardIssueDate(Date),
    ToggleShowValidityDate,
    ChangeIdCardValidityDate(Date),
    ToggleShowDateOfBirth,
    ChangeDateOfBirth(Date),
    AddGuest,
}

pub struct AddGuestScreen {
    error: String,
    first_name_input: TextBox,
    last_name_input: TextBox,
    has_id_card_checkbox: Checkbox,
    phone_number_input: PhoneNumberTextBox,
    date_of_birth_input: DateInput,
    id_card_ucn_input: UcnTextBox,
    id_card_number_input: IdCardNumberTextBox,
    id_card_issue_authority_input: TextBox,
    id_card_issue_date_input: DateInput,
    id_card_validity_input: DateInput,
}
impl AddGuestScreen {
    pub fn new() -> Self {
        Self {
            error: "".to_owned(),
            first_name_input: TextBox::new("", 20),
            last_name_input: TextBox::new("", 20),
            has_id_card_checkbox: Checkbox::new("Id card", false),
            phone_number_input: PhoneNumberTextBox::new(""),
            id_card_ucn_input: UcnTextBox::new(""),
            id_card_number_input: IdCardNumberTextBox::new(""),
            id_card_issue_authority_input: TextBox::new("", 25),
            id_card_issue_date_input: DateInput::new(
                "Issue Date",
                Date::today(),
                AppMessage::AddGuestMessage(AddGuestMessage::ToggleShowIssueDate),
            ),
            id_card_validity_input: DateInput::new(
                "Valid Until",
                Date::today(),
                AppMessage::AddGuestMessage(AddGuestMessage::ToggleShowValidityDate),
            ),
            date_of_birth_input: DateInput::new(
                "Date of birth",
                Date::today(),
                AppMessage::AddGuestMessage(AddGuestMessage::ToggleShowDateOfBirth),
            ),
        }
    }

    fn view_card_input(&self) -> Element<AppMessage> {
        if self.has_id_card_checkbox.is_checked() {
            column![
                text_input("UCN", self.id_card_ucn_input.get_text())
                    .on_input(|x| AppMessage::AddGuestMessage(AddGuestMessage::ChangeUcn(x)))
                    .align_x(Center)
                    .width(TEXT_BOX_WIDTH)
                    .line_height(1.5),
                text_input("Id Card Number", self.id_card_number_input.get_text())
                    .on_input(
                        |x| AppMessage::AddGuestMessage(AddGuestMessage::ChangeIdCardNumber(x))
                    )
                    .align_x(Center)
                    .width(TEXT_BOX_WIDTH)
                    .line_height(1.5),
                text_input(
                    "Id Card Issue Authority",
                    self.id_card_issue_authority_input.get_text()
                )
                .on_input(|x| AppMessage::AddGuestMessage(
                    AddGuestMessage::ChangeIdCardIssueAuthority(x)
                ))
                .align_x(Center)
                .width(TEXT_BOX_WIDTH)
                .line_height(1.5),
                self.id_card_issue_date_input
                    .view(
                        |x| AppMessage::AddGuestMessage(AddGuestMessage::ChangeIdCardIssueDate(x))
                    ),
                self.id_card_validity_input
                    .view(|x| AppMessage::AddGuestMessage(
                        AddGuestMessage::ChangeIdCardValidityDate(x)
                    ))
            ]
            .spacing(FORM_SPACING)
            .align_x(Center)
        } else {
            column![]
        }
        .into()
    }

    fn validate_date_before(date: Date, max_date: Date, message: &str) -> Result<Date, String> {
        if date.year > max_date.year
            || (date.year == max_date.year && date.month > max_date.month)
            || (date.year == max_date.year
                && date.month == max_date.month
                && date.day > max_date.day)
        {
            return Err(message.to_owned());
        }
        Ok(date)
    }

    fn retrieve_and_validate_card(&self) -> Result<IdCard, String> {
        let ucn = if self.id_card_ucn_input.get_text().len() < 10 {
            return Err("Invalid UCN".to_owned());
        } else {
            self.id_card_ucn_input.get_text().to_owned()
        };
        let id_card_number = if self.id_card_number_input.get_text().len() < 10 {
            return Err("Invalid card number".to_owned());
        } else {
            self.id_card_number_input.get_text().to_owned()
        };
        let issue_authority = if self.id_card_issue_authority_input.get_text().is_empty() {
            return Err("Invalid issue authority".to_owned());
        } else {
            self.id_card_issue_authority_input.get_text().to_owned()
        };
        let today = Date::today();
        let issue_date = Self::validate_date_before(
            self.id_card_issue_date_input.get_date(),
            today,
            "Invalid card issue date",
        )?;
        Self::validate_date_before(
            today,
            self.id_card_validity_input.get_date(),
            "Invalid card validity date",
        )?;
        let validity_date = self.id_card_validity_input.get_date();

        Ok(IdCard {
            ucn,
            number: id_card_number,
            issue_authority,
            issue_date,
            validity_date,
        })
    }

    fn retrieve_and_validate_phone(&self) -> Result<Option<String>, String> {
        let phone = self.phone_number_input.get_text();
        if phone.is_empty() {
            Ok(None)
        } else if phone.len() < 12 {
            Err("Invalid phone number".to_owned())
        } else {
            Ok(Some(phone.to_owned()))
        }
    }

    fn retrieve_and_validate_input(&self) -> Result<AddGuestInput, String> {
        let today = Date::today();
        let first_name = {
            if self.first_name_input.get_text().is_empty() {
                return Err("Enter first name".to_owned());
            } else {
                self.first_name_input.get_text()
            }
        };
        let last_name = {
            if self.last_name_input.get_text().is_empty() {
                return Err("Enter last name".to_owned());
            } else {
                self.last_name_input.get_text()
            }
        };
        let date_of_birth = Self::validate_date_before(
            self.date_of_birth_input.get_date(),
            today,
            "Invalid date of birth",
        )?;
        let id_card = if self.has_id_card_checkbox.is_checked() {
            Some(self.retrieve_and_validate_card()?)
        } else {
            None
        };
        let phone_number = self.retrieve_and_validate_phone()?;

        Ok(AddGuestInput::new(
            first_name.to_owned(),
            last_name.to_owned(),
            date_of_birth,
            phone_number,
            id_card,
        ))
    }

    fn add_guest(&mut self) -> Task<AppMessage> {
        let raw_input = self.retrieve_and_validate_input();
        let input = if let Ok(ok) = raw_input {
            ok
        } else {
            self.error = raw_input.unwrap_err();
            return Task::none();
        };

        todo!("Send request {input:?}")
    }
}
impl Screen for AddGuestScreen {
    fn update(
        &mut self,
        message: AppMessage,
        _global_state: Arc<Mutex<GlobalState>>,
    ) -> Task<AppMessage> {
        match message {
            AppMessage::AddGuestMessage(m) => match m {
                AddGuestMessage::ChangeFirstName(x) => {
                    self.first_name_input.update(x);
                    Task::none()
                }
                AddGuestMessage::ChangeLastName(x) => {
                    self.last_name_input.update(x);
                    Task::none()
                }
                AddGuestMessage::ChangeCheckbox(x) => {
                    self.has_id_card_checkbox.update(x);
                    Task::none()
                }
                AddGuestMessage::ChangePhoneNumber(x) => {
                    self.phone_number_input.update(x);
                    Task::none()
                }
                AddGuestMessage::ChangeUcn(x) => {
                    self.id_card_ucn_input.update(x);
                    Task::none()
                }
                AddGuestMessage::ChangeIdCardNumber(x) => {
                    self.id_card_number_input.update(x);
                    Task::none()
                }
                AddGuestMessage::ChangeIdCardIssueAuthority(x) => {
                    self.id_card_issue_authority_input.update(x);
                    Task::none()
                }
                AddGuestMessage::ToggleShowIssueDate => {
                    self.id_card_issue_date_input.toggle_show();
                    Task::none()
                }
                AddGuestMessage::ChangeIdCardIssueDate(date) => {
                    self.id_card_issue_date_input.update_date(date);
                    self.id_card_issue_date_input.toggle_show();
                    Task::none()
                }
                AddGuestMessage::ToggleShowValidityDate => {
                    self.id_card_validity_input.toggle_show();
                    Task::none()
                }
                AddGuestMessage::ChangeIdCardValidityDate(date) => {
                    self.id_card_validity_input.update_date(date);
                    self.id_card_validity_input.toggle_show();
                    Task::none()
                }
                AddGuestMessage::ToggleShowDateOfBirth => {
                    self.date_of_birth_input.toggle_show();
                    Task::none()
                }
                AddGuestMessage::ChangeDateOfBirth(date) => {
                    self.date_of_birth_input.update_date(date);
                    self.date_of_birth_input.toggle_show();
                    Task::none()
                }
                AddGuestMessage::AddGuest => self.add_guest(),
            },
            _ => Task::none(),
        }
    }

    fn view(&self, _global_state: Arc<Mutex<GlobalState>>) -> Element<AppMessage> {
        scrollable(
            column![
                text!("Add Guest")
                    .align_x(Center)
                    .size(TITLE_FONT_SIZE)
                    .width(Fill),
                text_input("First Name", self.first_name_input.get_text())
                    .on_input(|x| AppMessage::AddGuestMessage(AddGuestMessage::ChangeFirstName(x)))
                    .align_x(Center)
                    .width(TEXT_BOX_WIDTH)
                    .line_height(1.5),
                text_input("Last Name", self.last_name_input.get_text())
                    .on_input(|x| AppMessage::AddGuestMessage(AddGuestMessage::ChangeLastName(x)))
                    .align_x(Center)
                    .width(TEXT_BOX_WIDTH)
                    .line_height(1.5),
                self.date_of_birth_input
                    .view(|x| AppMessage::AddGuestMessage(AddGuestMessage::ChangeDateOfBirth(x))),
                text_input("Phone number (with +)", self.phone_number_input.get_text())
                    .on_input(
                        |x| AppMessage::AddGuestMessage(AddGuestMessage::ChangePhoneNumber(x))
                    )
                    .align_x(Center)
                    .width(TEXT_BOX_WIDTH)
                    .line_height(1.5),
                self.has_id_card_checkbox
                    .view(|x| AppMessage::AddGuestMessage(AddGuestMessage::ChangeCheckbox(x))),
                self.view_card_input(),
                text!("{}", self.error)
                    .color(ERROR_COLOR)
                    .size(18)
                    .align_x(Center)
                    .width(Fill),
                button("Add")
                    .on_press(AppMessage::AddGuestMessage(AddGuestMessage::AddGuest))
                    .height(30)
                    .width(80)
            ]
            .spacing(FORM_SPACING)
            .align_x(Center)
            .padding(FORM_PADDING),
        )
        .into()
    }
}
