use std::sync::{Arc, Mutex};

use iced::{
    widget::{button, column, row, scrollable, text, text_input},
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
        date_input::DateInput,
        focus_chain::FocusChain,
        notification::NotificationType,
        text_box::{
            number_text_box::{NumberTextBox, NumberType},
            text_box::TextElement,
        },
    },
    services::find_unoccupied_rooms::{
        find_unoccupied_rooms, FindUnoccupiedRoomsInput, FindUnoccupiedRoomsResult,
    },
    styles::{ERROR_COLOR, FORM_PADDING, FORM_SPACING, TEXT_BOX_WIDTH, TITLE_FONT_SIZE},
    utils::show_notification,
};

#[derive(Debug, Clone, Copy)]
enum BookRoomStep {
    DateAndRoom,
}
impl BookRoomStep {
    fn get_focus_chain(self) -> FocusChain {
        match self {
            BookRoomStep::DateAndRoom => FocusChain::new(DATE_AND_ROOM_IDS.into()),
        }
    }
}

const MIN_CAPACITY_ID: &str = "Book Room Min Capacity";
const MAX_CAPACITY_ID: &str = "Book Room Min Capacity";
const DATE_AND_ROOM_IDS: [&str; 2] = [MIN_CAPACITY_ID, MAX_CAPACITY_ID];

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
}

pub struct BookRoomScreen {
    current_step: BookRoomStep,
    focus_chain: FocusChain,
    minimum_capacity_input: NumberTextBox,
    maximum_capacity_input: NumberTextBox,
    start_date_input: DateInput,
    end_date_input: DateInput,
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
            button("Next")
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
                    println!("TODO: set ids: '{ids:?}'");
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
        };

        scrollable(current_view).into()
    }
}
