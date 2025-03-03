use std::sync::{Arc, Mutex};

use iced::{
    widget::{button, column, scrollable, text, text_input},
    Alignment::Center,
    Element,
    Length::Fill,
    Task,
};
use iced_aw::date_picker::Date;
use uuid::Uuid;

use crate::{
    app::{AppMessage, GlobalState, Screen},
    components::{
        checkbox::Checkbox,
        date_input::DateInput,
        focus_chain::FocusChain,
        notification::NotificationType,
        text_box::{
            id_card_number_text_box::IdCardNumberTextBox,
            phone_number_text_box::PhoneNumberTextBox,
            text_box::{TextBox, TextElement},
            ucn_text_box::UcnTextBox,
        },
    },
    model::id_card::IdCard,
    services::{
        self,
        add_guest::{AddGuestInput, AddGuestResult},
    },
    styles::{ERROR_COLOR, FORM_PADDING, FORM_SPACING, TEXT_BOX_WIDTH, TITLE_FONT_SIZE},
    utils::show_notification,
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
    ShowError(String),
    AddGuest,
    GuestAdded(Uuid),
}

const FIRST_NAME_ID: &str = "Register First Name";
const LAST_NAME_ID: &str = "Register Last Name";
const PHONE_NUMBER_ID: &str = "Register Phone Number";
const ID_CARD_UCN_ID: &str = "Register ID Card UCN";
const ID_CARD_NUMBER_ID: &str = "Register ID Card Number";
const ID_CARD_ISSUE_AUTHORITY_ID: &str = "Register ID Card Issue Authority";
const FOCUS_IDS_WITHOUT_CARD: [&str; 3] = [FIRST_NAME_ID, LAST_NAME_ID, PHONE_NUMBER_ID];
const FOCUS_IDS_WITH_CARD: [&str; 6] = [
    FIRST_NAME_ID,
    LAST_NAME_ID,
    PHONE_NUMBER_ID,
    ID_CARD_UCN_ID,
    ID_CARD_NUMBER_ID,
    ID_CARD_ISSUE_AUTHORITY_ID,
];

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
    focus_chain: FocusChain,
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
            focus_chain: FocusChain::new(FOCUS_IDS_WITHOUT_CARD.into()),
        }
    }

    fn view_card_input(&self) -> Element<AppMessage> {
        if self.has_id_card_checkbox.is_checked() {
            column![
                text_input("UCN", self.id_card_ucn_input.get_text())
                    .id(ID_CARD_UCN_ID)
                    .on_input(|x| AppMessage::AddGuestMessage(AddGuestMessage::ChangeUcn(x)))
                    .on_submit(AppMessage::AddGuestMessage(AddGuestMessage::AddGuest))
                    .align_x(Center)
                    .width(TEXT_BOX_WIDTH)
                    .line_height(1.5),
                text_input("Id Card Number", self.id_card_number_input.get_text())
                    .id(ID_CARD_NUMBER_ID)
                    .on_input(
                        |x| AppMessage::AddGuestMessage(AddGuestMessage::ChangeIdCardNumber(x))
                    )
                    .on_submit(AppMessage::AddGuestMessage(AddGuestMessage::AddGuest))
                    .align_x(Center)
                    .width(TEXT_BOX_WIDTH)
                    .line_height(1.5),
                text_input(
                    "Id Card Issue Authority",
                    self.id_card_issue_authority_input.get_text()
                )
                .id(ID_CARD_ISSUE_AUTHORITY_ID)
                .on_input(|x| AppMessage::AddGuestMessage(
                    AddGuestMessage::ChangeIdCardIssueAuthority(x)
                ))
                .on_submit(AppMessage::AddGuestMessage(AddGuestMessage::AddGuest))
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
        let id_card_number = if self.id_card_number_input.get_text().len() < 9 {
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
            id_card_number,
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

    fn add_guest(&mut self, global_state: Arc<Mutex<GlobalState>>) -> Task<AppMessage> {
        let raw_input = self.retrieve_and_validate_input();
        let input = if let Ok(ok) = raw_input {
            ok
        } else {
            self.error = raw_input.unwrap_err();
            return Task::none();
        };

        Task::perform(
            services::add_guest::add_guest(global_state, input),
            move |res| match res {
                Ok(AddGuestResult::GuestAdded(uuid)) => {
                    AppMessage::AddGuestMessage(AddGuestMessage::GuestAdded(uuid))
                }
                Ok(AddGuestResult::Forbidden) => AppMessage::TokenExpired,
                Ok(AddGuestResult::BadRequest(bad_request)) => {
                    AppMessage::AddGuestMessage(AddGuestMessage::ShowError(bad_request))
                }
                Err(err) => AppMessage::AddGuestMessage(AddGuestMessage::ShowError(err)),
            },
        )
    }

    fn clear_inputs(&mut self) {
        let today = Date::today();

        self.first_name_input.update("");
        self.last_name_input.update("");
        self.id_card_issue_authority_input.update("");
        self.id_card_number_input.update("");
        self.id_card_ucn_input.update("");
        self.phone_number_input.update("");
        self.date_of_birth_input.update_date(today);
        self.id_card_issue_date_input.update_date(today);
        self.id_card_validity_input.update_date(today);
        self.error = "".to_owned();
    }
}
impl Screen for AddGuestScreen {
    fn update(
        &mut self,
        message: AppMessage,
        global_state: Arc<Mutex<GlobalState>>,
    ) -> Task<AppMessage> {
        match message {
            AppMessage::AddGuestMessage(m) => match m {
                AddGuestMessage::ChangeFirstName(x) => {
                    self.focus_chain.set_focus(Some(FIRST_NAME_ID));
                    self.first_name_input.update(x);
                    Task::none()
                }
                AddGuestMessage::ChangeLastName(x) => {
                    self.focus_chain.set_focus(Some(LAST_NAME_ID));
                    self.last_name_input.update(x);
                    Task::none()
                }
                AddGuestMessage::ChangeCheckbox(x) => {
                    let selected = self.focus_chain.get_selected();
                    self.focus_chain = if x {
                        FocusChain::new(FOCUS_IDS_WITH_CARD.into())
                    } else {
                        FocusChain::new(FOCUS_IDS_WITHOUT_CARD.into())
                    };
                    self.focus_chain.set_focus(selected);

                    self.has_id_card_checkbox.update(x);
                    Task::none()
                }
                AddGuestMessage::ChangePhoneNumber(x) => {
                    self.focus_chain.set_focus(Some(PHONE_NUMBER_ID));
                    self.phone_number_input.update(x);
                    Task::none()
                }
                AddGuestMessage::ChangeUcn(x) => {
                    self.focus_chain.set_focus(Some(ID_CARD_UCN_ID));
                    self.id_card_ucn_input.update(x);
                    Task::none()
                }
                AddGuestMessage::ChangeIdCardNumber(x) => {
                    self.focus_chain.set_focus(Some(ID_CARD_NUMBER_ID));
                    self.id_card_number_input.update(x);
                    Task::none()
                }
                AddGuestMessage::ChangeIdCardIssueAuthority(x) => {
                    self.focus_chain.set_focus(Some(ID_CARD_ISSUE_AUTHORITY_ID));
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
                AddGuestMessage::AddGuest => self.add_guest(global_state),
                AddGuestMessage::ShowError(err) => {
                    self.error = err;
                    Task::none()
                }
                AddGuestMessage::GuestAdded(_uuid) => {
                    self.clear_inputs();
                    Task::done(show_notification("Guest added", NotificationType::Success))
                }
            },
            AppMessage::SelectNext => {
                self.focus_chain.set_next();
                self.focus_chain.apply_focus()
            }
            AppMessage::SelectPrev => {
                self.focus_chain.set_prev();
                self.focus_chain.apply_focus()
            }
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
                    .id(FIRST_NAME_ID)
                    .on_input(|x| AppMessage::AddGuestMessage(AddGuestMessage::ChangeFirstName(x)))
                    .on_submit(AppMessage::AddGuestMessage(AddGuestMessage::AddGuest))
                    .align_x(Center)
                    .width(TEXT_BOX_WIDTH)
                    .line_height(1.5),
                text_input("Last Name", self.last_name_input.get_text())
                    .id(LAST_NAME_ID)
                    .on_input(|x| AppMessage::AddGuestMessage(AddGuestMessage::ChangeLastName(x)))
                    .on_submit(AppMessage::AddGuestMessage(AddGuestMessage::AddGuest))
                    .align_x(Center)
                    .width(TEXT_BOX_WIDTH)
                    .line_height(1.5),
                self.date_of_birth_input
                    .view(|x| AppMessage::AddGuestMessage(AddGuestMessage::ChangeDateOfBirth(x))),
                text_input("Phone number (with +)", self.phone_number_input.get_text())
                    .id(PHONE_NUMBER_ID)
                    .on_input(
                        |x| AppMessage::AddGuestMessage(AddGuestMessage::ChangePhoneNumber(x))
                    )
                    .on_submit(AppMessage::AddGuestMessage(AddGuestMessage::AddGuest))
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
