use std::sync::{Arc, Mutex};

use iced::{
    widget::{button, column, container, row, scrollable, text, text_input},
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
        list_input::{guest_list_input::GuestListInput, room_list_input::RoomListInput},
        notification::NotificationType,
        text_box::{
            number_text_box::{NumberTextBox, NumberType},
            phone_number_text_box::PhoneNumberTextBox,
            text_box::{TextBox, TextElement},
            ucn_text_box::UcnTextBox,
        },
    },
    model::{guest::Guest, room::Room},
    services::{
        find_guest::{find_guest, FindGuestInput, FindGuestResult},
        find_unoccupied_rooms::{
            find_unoccupied_rooms, FindUnoccupiedRoomsInput, FindUnoccupiedRoomsResult,
        },
        get_guest::GetGuestResult,
        get_room::GetRoomResult,
    },
    styles::{ERROR_COLOR, FORM_PADDING, FORM_SPACING, TEXT_BOX_WIDTH, TITLE_FONT_SIZE},
    utils::show_notification,
};

#[derive(Debug, Clone, Copy)]
enum BookRoomStep {
    DateAndRoom,
    AddGuests,
}
impl BookRoomStep {
    fn get_focus_chain(self) -> FocusChain {
        match self {
            BookRoomStep::DateAndRoom => FocusChain::new(DATE_AND_ROOM_IDS.into()),
            BookRoomStep::AddGuests => FocusChain::new(ADD_GUESTS_IDS.into()),
        }
    }
}

const MIN_CAPACITY_ID: &str = "Book Room Min Capacity";
const MAX_CAPACITY_ID: &str = "Book Room Min Capacity";
const DATE_AND_ROOM_IDS: [&str; 2] = [MIN_CAPACITY_ID, MAX_CAPACITY_ID];

const FIEST_NAME_ID: &str = "Book Room First Name";
const LAST_NAME_ID: &str = "Book Room Last Name";
const PHONE_NUMBER_ID: &str = "Book Room Phone";
const UCN_ID: &str = "Book Room UCN";
const ADD_GUESTS_IDS: [&str; 4] = [FIEST_NAME_ID, LAST_NAME_ID, UCN_ID, PHONE_NUMBER_ID];

#[derive(Debug, Clone)]
pub enum BookRoomMessage {
    SetError(String),
    SetStep(BookRoomStep),
    ChangeMinimumCapacity(String),
    ChangeMaximumCapacity(String),
    ChangeStartDate(Date),
    ToggleShowStartDate,
    ChangeEndDate(Date),
    ToggleShowEndDate,
    FindFreeRooms,
    FoundFreeRooms(Vec<Uuid>),
    RoomLoaded(Box<Room>),
    ScrollRooms(f32),
    SelectRoom(Uuid),
    ChangeFirstName(String),
    ChangeLastName(String),
    ChangeUCN(String),
    ChangeDateOfBirth(Date),
    ChangeSearchByDateOfBirth(bool),
    ToggleShowDateOfBirth,
    ChangePhoneNumber(String),
    FindGuests,
    FoundGuests(Vec<Uuid>),
    GuestLoaded(Box<Guest>),
    ScrollGuests(f32),
    AddGuest(Uuid),
    SetSelectedMainGuest(Uuid),
    RemoveAddedGuest(Uuid),
}

pub struct BookRoomScreen {
    current_step: BookRoomStep,
    focus_chain: FocusChain,
    minimum_capacity_input: NumberTextBox,
    maximum_capacity_input: NumberTextBox,
    start_date_input: DateInput,
    end_date_input: DateInput,
    select_room_input: RoomListInput,
    select_guest_input: GuestListInput,
    selected_guests: Vec<Guest>,
    main_guest_id: Option<Uuid>,
    first_name_input: TextBox,
    last_name_input: TextBox,
    ucn_input: UcnTextBox,
    phone_number_input: PhoneNumberTextBox,
    date_of_birth_input: DateInput,
    search_by_date_of_birth_checkbox: Checkbox,
    error: String,
}
impl BookRoomScreen {
    pub fn new() -> Self {
        Self {
            current_step: BookRoomStep::DateAndRoom,
            error: "".to_owned(),
            minimum_capacity_input: NumberTextBox::new("", 2, NumberType::PositiveInteger),
            maximum_capacity_input: NumberTextBox::new("", 2, NumberType::PositiveInteger),
            start_date_input: DateInput::new(
                "Start",
                Date::today(),
                AppMessage::BookRoomMessage(BookRoomMessage::ToggleShowStartDate),
            ),
            end_date_input: DateInput::new(
                "End",
                Date::today(),
                AppMessage::BookRoomMessage(BookRoomMessage::ToggleShowEndDate),
            ),
            focus_chain: BookRoomStep::DateAndRoom.get_focus_chain(),
            select_room_input: RoomListInput::new(),
            select_guest_input: GuestListInput::new(),
            selected_guests: vec![],
            first_name_input: TextBox::new("", 20),
            last_name_input: TextBox::new("", 20),
            ucn_input: UcnTextBox::new(""),
            phone_number_input: PhoneNumberTextBox::new(""),
            date_of_birth_input: DateInput::new(
                "Date of birth",
                Date::today(),
                AppMessage::BookRoomMessage(BookRoomMessage::ToggleShowDateOfBirth),
            ),
            search_by_date_of_birth_checkbox: Checkbox::new("Search by date of birth", false),
            main_guest_id: None,
        }
    }

    fn view_date_and_room(&self) -> Element<AppMessage> {
        column![
            text!("Book Room")
                .align_x(Center)
                .size(TITLE_FONT_SIZE)
                .width(Fill),
            row![
                text_input("Min capacity", self.minimum_capacity_input.get_text())
                    .id(MIN_CAPACITY_ID)
                    .on_input(|x| AppMessage::BookRoomMessage(
                        BookRoomMessage::ChangeMinimumCapacity(x)
                    ))
                    .align_x(Center)
                    .width(120)
                    .line_height(1.5),
                text_input("Max capacity", self.maximum_capacity_input.get_text())
                    .id(MIN_CAPACITY_ID)
                    .on_input(|x| AppMessage::BookRoomMessage(
                        BookRoomMessage::ChangeMaximumCapacity(x)
                    ))
                    .align_x(Center)
                    .width(120)
                    .line_height(1.5),
            ]
            .spacing(10),
            row![
                self.start_date_input
                    .view(|x| AppMessage::BookRoomMessage(BookRoomMessage::ChangeStartDate(x))),
                self.end_date_input
                    .view(|x| AppMessage::BookRoomMessage(BookRoomMessage::ChangeEndDate(x))),
            ]
            .spacing(10),
            button("Find")
                .on_press(AppMessage::BookRoomMessage(BookRoomMessage::FindFreeRooms))
                .height(30)
                .width(80),
            text!("{}", self.error)
                .color(ERROR_COLOR)
                .size(18)
                .align_x(Center)
                .width(Fill),
            self.select_room_input.view(
                |id| AppMessage::BookRoomMessage(BookRoomMessage::SelectRoom(id)),
                |x| AppMessage::BookRoomMessage(BookRoomMessage::ScrollRooms(
                    x.relative_offset().y
                ))
            ),
            button("Next")
                .on_press(AppMessage::BookRoomMessage(BookRoomMessage::SetStep(
                    BookRoomStep::AddGuests
                )))
                .height(30)
                .width(80)
        ]
        .spacing(FORM_SPACING)
        .align_x(Center)
        .padding(FORM_PADDING)
        .into()
    }

    fn view_add_guests(&self) -> Element<AppMessage> {
        column![
            text!("Add guests")
                .align_x(Center)
                .size(TITLE_FONT_SIZE)
                .width(Fill),
            row![
                text_input("First name", self.first_name_input.get_text())
                    .id(FIEST_NAME_ID)
                    .on_input(|x| AppMessage::BookRoomMessage(BookRoomMessage::ChangeFirstName(x)))
                    .align_x(Center)
                    .width(150)
                    .line_height(1.5),
                text_input("Last name", self.last_name_input.get_text())
                    .id(LAST_NAME_ID)
                    .on_input(|x| AppMessage::BookRoomMessage(BookRoomMessage::ChangeLastName(x)))
                    .align_x(Center)
                    .width(150)
                    .line_height(1.5)
            ]
            .spacing(10),
            row![
                text_input("UCN", self.ucn_input.get_text())
                    .id(UCN_ID)
                    .on_input(|x| AppMessage::BookRoomMessage(BookRoomMessage::ChangeUCN(x)))
                    .align_x(Center)
                    .width(150)
                    .line_height(1.5),
                text_input("Phone number", self.phone_number_input.get_text())
                    .id(PHONE_NUMBER_ID)
                    .on_input(
                        |x| AppMessage::BookRoomMessage(BookRoomMessage::ChangePhoneNumber(x))
                    )
                    .align_x(Center)
                    .width(150)
                    .line_height(1.5),
            ]
            .spacing(10),
            self.search_by_date_of_birth_checkbox
                .view(
                    |x| AppMessage::BookRoomMessage(BookRoomMessage::ChangeSearchByDateOfBirth(x))
                ),
            self.date_of_birth_input
                .view(|x| AppMessage::BookRoomMessage(BookRoomMessage::ChangeDateOfBirth(x))),
            button("Find")
                .on_press(AppMessage::BookRoomMessage(BookRoomMessage::FindGuests))
                .height(30)
                .width(80),
            text!("{}", self.error)
                .color(ERROR_COLOR)
                .size(18)
                .align_x(Center)
                .width(Fill),
            self.select_guest_input.view(
                |id| AppMessage::BookRoomMessage(BookRoomMessage::AddGuest(id)),
                |x| AppMessage::BookRoomMessage(BookRoomMessage::ScrollGuests(
                    x.relative_offset().y
                ))
            ),
            self.view_added_guests_list(),
            button("Previous")
                .on_press(AppMessage::BookRoomMessage(BookRoomMessage::SetStep(
                    BookRoomStep::DateAndRoom
                )))
                .height(30)
                .width(80)
        ]
        .spacing(FORM_SPACING)
        .align_x(Center)
        .padding(FORM_PADDING)
        .into()
    }

    fn get_optional_number(number_str: &str) -> Option<i16> {
        if number_str.is_empty() {
            return None;
        }

        number_str.parse::<i16>().map_or(None, |x| Some(x))
    }

    fn find_free_rooms(&mut self, global_state: Arc<Mutex<GlobalState>>) -> Task<AppMessage> {
        let input = FindUnoccupiedRoomsInput {
            start_date: self.start_date_input.get_date(),
            end_date: self.end_date_input.get_date(),
            minimum_capacity: Self::get_optional_number(self.minimum_capacity_input.get_text()),
            maximum_capacity: Self::get_optional_number(self.maximum_capacity_input.get_text()),
        };

        Task::perform(
            find_unoccupied_rooms(global_state, input),
            |res| match res {
                Ok(FindUnoccupiedRoomsResult::Found(ids)) => {
                    AppMessage::BookRoomMessage(BookRoomMessage::FoundFreeRooms(ids))
                }
                Ok(FindUnoccupiedRoomsResult::Forbidden) => AppMessage::TokenExpired,
                Ok(FindUnoccupiedRoomsResult::BadRequest(err)) => {
                    AppMessage::BookRoomMessage(BookRoomMessage::SetError(err))
                }
                Err(err) => {
                    println!("Error finding free rooms: '{err}'");
                    show_notification("Unexpected Error", NotificationType::Error)
                }
            },
        )
    }

    fn map_get_room_result(result: Result<GetRoomResult, String>) -> AppMessage {
        match result {
            Ok(GetRoomResult::Found(room)) => {
                AppMessage::BookRoomMessage(BookRoomMessage::RoomLoaded(Box::new(room)))
            }
            Ok(GetRoomResult::Forbidden) => AppMessage::TokenExpired,
            Ok(GetRoomResult::BadRequest(err)) => {
                AppMessage::BookRoomMessage(BookRoomMessage::SetError(err))
            }
            Err(err) => {
                println!("Error fetching rooms: {err}");
                show_notification("Unexpected_message", NotificationType::Error)
            }
        }
    }

    fn map_get_guest_result(result: Result<GetGuestResult, String>) -> AppMessage {
        match result {
            Ok(GetGuestResult::Found(guest)) => {
                AppMessage::BookRoomMessage(BookRoomMessage::GuestLoaded(guest))
            }
            Ok(GetGuestResult::Forbidden) => AppMessage::TokenExpired,
            Ok(GetGuestResult::BadRequest(err)) => {
                AppMessage::BookRoomMessage(BookRoomMessage::SetError(err))
            }
            Err(err) => {
                println!("Error fetching guest: {err}");
                show_notification("Unexpected_message", NotificationType::Error)
            }
        }
    }

    fn get_text_if_not_empty(input: &impl TextElement) -> Option<String> {
        if input.get_text().is_empty() {
            None
        } else {
            Some(input.get_text().to_owned())
        }
    }

    fn get_find_guests_input(&self) -> FindGuestInput {
        let first_name = Self::get_text_if_not_empty(&self.first_name_input);
        let last_name = Self::get_text_if_not_empty(&self.last_name_input);
        let phone_number = Self::get_text_if_not_empty(&self.phone_number_input);
        let ucn = Self::get_text_if_not_empty(&self.ucn_input);
        let date_of_birth = if self.search_by_date_of_birth_checkbox.is_checked() {
            Some(self.date_of_birth_input.get_date().to_string())
        } else {
            None
        };

        FindGuestInput {
            first_name,
            last_name,
            date_of_birth,
            ucn,
            phone_number,
        }
    }

    fn find_guests(&mut self, global_state: Arc<Mutex<GlobalState>>) -> Task<AppMessage> {
        self.error = "".to_owned();
        let input = self.get_find_guests_input();

        Task::perform(find_guest(global_state, input), |res| match res {
            Ok(FindGuestResult::Found(ids)) => {
                println!("Found guests count: {}", ids.len());
                AppMessage::BookRoomMessage(BookRoomMessage::FoundGuests(ids))
            }
            Ok(FindGuestResult::Forbidden) => AppMessage::TokenExpired,
            Ok(FindGuestResult::BadRequest(err)) => {
                AppMessage::BookRoomMessage(BookRoomMessage::SetError(err))
            }
            Err(err) => {
                println!("Error finding guests: {err}");
                show_notification("Unexpected error", NotificationType::Error)
            }
        })
    }

    fn add_guest_to_booking(&mut self, guest_id: Uuid) -> Task<AppMessage> {
        let is_added = self
            .selected_guests
            .iter()
            .find(|guest| guest.id == guest_id);

        if is_added.is_none() {
            self.select_guest_input
                .get_loaded(guest_id)
                .map(|guest| self.selected_guests.push(guest));
        }

        Task::none()
    }

    fn view_added_guest(&self, guest: &Guest) -> Element<AppMessage> {
        let select_main_guest_button = if self.main_guest_id == Some(guest.id) {
            button("Selected").width(80)
        } else {
            button("Select")
                .on_press(AppMessage::BookRoomMessage(
                    BookRoomMessage::SetSelectedMainGuest(guest.id),
                ))
                .width(80)
        };

        row![
            text!("{} {}", guest.first_name, guest.last_name).width(220),
            button("Remove")
                .on_press(AppMessage::BookRoomMessage(
                    BookRoomMessage::RemoveAddedGuest(guest.id)
                ))
                .width(80),
            select_main_guest_button
        ]
        .spacing(10)
        .into()
    }

    fn view_added_guests_list(&self) -> Element<AppMessage> {
        let mut added_guests_list = column![];
        if !self.selected_guests.is_empty() {
            added_guests_list = added_guests_list.push(text!("Selected Guests:").center());
        }

        for guest in &self.selected_guests {
            added_guests_list = added_guests_list.push(self.view_added_guest(guest));
        }

        container(added_guests_list.spacing(5)).width(480).into()
    }
}
impl Screen for BookRoomScreen {
    fn update(
        &mut self,
        message: AppMessage,
        global_state: Arc<Mutex<GlobalState>>,
    ) -> Task<AppMessage> {
        match message {
            AppMessage::BookRoomMessage(m) => match m {
                BookRoomMessage::SetError(err) => {
                    self.error = err;
                    Task::none()
                }
                BookRoomMessage::SetStep(book_room_step) => {
                    self.current_step = book_room_step;
                    self.focus_chain = book_room_step.get_focus_chain();
                    Task::none()
                }
                BookRoomMessage::ChangeMinimumCapacity(min_capacity) => {
                    self.focus_chain.set_focus(Some(MIN_CAPACITY_ID));
                    self.minimum_capacity_input.update(min_capacity);
                    Task::none()
                }
                BookRoomMessage::ChangeMaximumCapacity(max_capacity) => {
                    self.focus_chain.set_focus(Some(MAX_CAPACITY_ID));
                    self.maximum_capacity_input.update(max_capacity);
                    Task::none()
                }
                BookRoomMessage::ChangeStartDate(date) => {
                    self.start_date_input.update_date(date);
                    self.start_date_input.toggle_show();
                    Task::none()
                }
                BookRoomMessage::ToggleShowStartDate => {
                    self.start_date_input.toggle_show();
                    Task::none()
                }
                BookRoomMessage::ChangeEndDate(date) => {
                    self.end_date_input.update_date(date);
                    self.end_date_input.toggle_show();
                    Task::none()
                }
                BookRoomMessage::ToggleShowEndDate => {
                    self.end_date_input.toggle_show();
                    Task::none()
                }
                BookRoomMessage::FindFreeRooms => self.find_free_rooms(global_state),
                BookRoomMessage::FoundFreeRooms(ids) => {
                    self.error = "".to_owned();
                    self.select_room_input
                        .update_ids(global_state, ids, Self::map_get_room_result)
                }
                BookRoomMessage::RoomLoaded(room) => {
                    self.select_room_input.update_loaded(*room);
                    Task::none()
                }
                BookRoomMessage::ScrollRooms(amount) => self.select_room_input.load_scrolled(
                    global_state,
                    amount,
                    Self::map_get_room_result,
                ),
                BookRoomMessage::SelectRoom(uuid) => {
                    self.select_room_input.set_selected(Some(uuid));
                    Task::none()
                }
                BookRoomMessage::ChangeFirstName(first_name) => {
                    self.focus_chain.set_focus(Some(FIEST_NAME_ID));
                    self.first_name_input.update(first_name);
                    Task::none()
                }
                BookRoomMessage::ChangeLastName(last_name) => {
                    self.focus_chain.set_focus(Some(LAST_NAME_ID));
                    self.last_name_input.update(last_name);
                    Task::none()
                }
                BookRoomMessage::ChangeUCN(ucn) => {
                    self.focus_chain.set_focus(Some(UCN_ID));
                    self.ucn_input.update(ucn);
                    Task::none()
                }
                BookRoomMessage::ChangeDateOfBirth(date) => {
                    self.date_of_birth_input.toggle_show();
                    self.date_of_birth_input.update_date(date);
                    Task::none()
                }
                BookRoomMessage::ToggleShowDateOfBirth => {
                    self.date_of_birth_input.toggle_show();
                    Task::none()
                }
                BookRoomMessage::ChangePhoneNumber(phone_number) => {
                    self.focus_chain.set_focus(Some(PHONE_NUMBER_ID));
                    self.phone_number_input.update(phone_number);
                    Task::none()
                }
                BookRoomMessage::FindGuests => self.find_guests(global_state),
                BookRoomMessage::ChangeSearchByDateOfBirth(should_search) => {
                    self.search_by_date_of_birth_checkbox.update(should_search);
                    Task::none()
                }
                BookRoomMessage::FoundGuests(uuids) => self.select_guest_input.update_ids(
                    global_state,
                    uuids,
                    Self::map_get_guest_result,
                ),
                BookRoomMessage::GuestLoaded(guest) => {
                    self.select_guest_input.update_loaded(*guest);
                    Task::none()
                }
                BookRoomMessage::ScrollGuests(scroll_amount) => self
                    .select_guest_input
                    .load_scrolled(global_state, scroll_amount, Self::map_get_guest_result),
                BookRoomMessage::AddGuest(uuid) => self.add_guest_to_booking(uuid),
                BookRoomMessage::SetSelectedMainGuest(uuid) => {
                    self.main_guest_id = Some(uuid);
                    Task::none()
                }
                BookRoomMessage::RemoveAddedGuest(uuid) => {
                    if self.main_guest_id == Some(uuid) {
                        self.main_guest_id = None;
                    }
                    self.selected_guests.retain(|guest| guest.id != uuid);

                    Task::none()
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
        let current_view = match self.current_step {
            BookRoomStep::DateAndRoom => self.view_date_and_room(),
            BookRoomStep::AddGuests => self.view_add_guests(),
        };

        scrollable(current_view).into()
    }
}
