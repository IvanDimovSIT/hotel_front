use std::{
    collections::{BTreeMap, HashMap},
    sync::{Arc, Mutex},
};

use iced::{
    widget::{button, column, row, scrollable, text, text_input},
    Alignment::Center,
    Element,
    Length::Fill,
    Task,
};
use uuid::Uuid;

use crate::{
    app::{AppMessage, GlobalState, Screen},
    components::{
        combo_box::{
            bathroom_type_combo_box::BathroomTypeComboBox, bed_size_combo_box::BedSizeComboBox,
        },
        notification::NotificationType,
        text_box::{
            number_text_box::{NumberTextBox, NumberType},
            room_number_text_box::RoomNumberTextBox,
            text_box::TextElement,
        },
    },
    model::{
        bathroom_type::BathroomType,
        bed::{Bed, BedSize},
    },
    services::{
        self,
        add_room::{AddRoomInput, AddRoomResult},
    },
    styles::{ERROR_COLOR, FORM_PADDING, FORM_SPACING, TEXT_BOX_WIDTH, TITLE_FONT_SIZE},
    utils::show_notification,
};

#[derive(Debug, Clone)]
pub enum AddRoomMessage {
    ChangeRoomNumber(String),
    ChangeFloor(String),
    ChagePrice(String),
    ChangeBathroomType(BathroomType),
    ChangeBedSize { bed_size: BedSize, input_id: u64 },
    ChangeBedCount { count: String, input_id: u64 },
    RemoveBedSizeInput(u64),
    AddBedSizeInput,
    AddRoom,
    RoomAdded(Uuid),
    ShowError(String),
}

pub struct AddRoomScreen {
    id_counter: u64,
    price: NumberTextBox,
    floor: NumberTextBox,
    room_number: RoomNumberTextBox,
    bathroom_type_combo_box: BathroomTypeComboBox,
    bed_count_inputs: BTreeMap<u64, BedCountInput>,
    error: String,
}
impl AddRoomScreen {
    pub fn new() -> Self {
        Self {
            id_counter: 0,
            price: NumberTextBox::new("", 9, NumberType::Price),
            floor: NumberTextBox::new("", 6, NumberType::PositiveInteger),
            room_number: RoomNumberTextBox::new(""),
            bathroom_type_combo_box: BathroomTypeComboBox::new(),
            bed_count_inputs: BTreeMap::new(),
            error: "".to_owned(),
        }
    }

    fn view_bed_count_inputs(&self, global_state: Arc<Mutex<GlobalState>>) -> Element<AppMessage> {
        let mut col = column![button("Add Beds")
            .on_press(AppMessage::AddRoomMessage(AddRoomMessage::AddBedSizeInput))];
        for (_, i) in &self.bed_count_inputs {
            col = col.push(i.view(global_state.clone()));
        }

        col.align_x(Center).spacing(10.0).into()
    }

    fn get_beds(&self) -> Vec<Bed> {
        let mut beds: HashMap<BedSize, Bed> = HashMap::new();
        for (_, bed) in &self.bed_count_inputs {
            if beds.contains_key(&bed.bed_size.get_selected()) {
                beds.get_mut(&bed.bed_size.get_selected()).unwrap().count +=
                    bed.count.get_text().parse::<i16>().unwrap_or_default();
            } else {
                let size = bed.bed_size.get_selected();
                let new_bed = Bed {
                    bed_size: size,
                    count: bed.count.get_text().parse().unwrap_or_default(),
                };
                beds.insert(size, new_bed);
            }
        }
        beds.into_iter().map(|(_, bed)| bed).collect()
    }

    fn get_input(&self) -> Result<AddRoomInput, String> {
        if self.price.get_text().is_empty() {
            return Err("Enter price".to_owned());
        }
        let price = match self.price.get_text().parse::<f64>() {
            Ok(ok) => (ok * 100.0) as i64,
            Err(_) => return Err("Invalid price".to_owned()),
        };

        if self.floor.get_text().is_empty() {
            return Err("Enter floor".to_owned());
        }
        let floor = match self.floor.get_text().parse::<i16>() {
            Ok(ok) => ok,
            Err(_) => return Err("Invalid price".to_owned()),
        };
        let room_number = if self.room_number.get_text().is_empty() {
            return Err("Enter room number".to_owned());
        } else {
            self.room_number.get_text().to_owned()
        };

        if self.bed_count_inputs.is_empty() {
            return Err("Add beds".to_owned());
        }

        let input = AddRoomInput {
            beds: self.get_beds(),
            price,
            floor,
            room_number,
            bathroom_type: self.bathroom_type_combo_box.get_selected(),
        };

        Ok(input)
    }

    fn clear_inputs(&mut self) {
        self.bed_count_inputs.clear();
        self.price.update("");
        self.room_number.update("");
        self.floor.update("");
        self.error.clear();
    }
}
impl Screen for AddRoomScreen {
    fn update(
        &mut self,
        message: AppMessage,
        global_state: Arc<Mutex<GlobalState>>,
    ) -> Task<AppMessage> {
        match message {
            AppMessage::AddRoomMessage(add_room_message) => match add_room_message {
                AddRoomMessage::ChangeRoomNumber(room_number) => {
                    self.room_number.update(room_number);
                    Task::none()
                }
                AddRoomMessage::ChangeFloor(floor) => {
                    self.floor.update(floor);
                    Task::none()
                }
                AddRoomMessage::ChagePrice(price) => {
                    self.price.update(price);
                    Task::none()
                }
                AddRoomMessage::ChangeBathroomType(bathroom_type) => {
                    self.bathroom_type_combo_box.update(bathroom_type);
                    Task::none()
                }
                AddRoomMessage::ChangeBedSize { bed_size, input_id } => {
                    if let Some(bed_count_input) = self.bed_count_inputs.get_mut(&input_id) {
                        bed_count_input.bed_size.update(bed_size);
                    }
                    Task::none()
                }
                AddRoomMessage::ChangeBedCount { count, input_id } => {
                    if let Some(bed_count_input) = self.bed_count_inputs.get_mut(&input_id) {
                        bed_count_input.count.update(count);
                    }
                    Task::none()
                }
                AddRoomMessage::RemoveBedSizeInput(id) => {
                    self.bed_count_inputs.remove(&id);
                    Task::none()
                }
                AddRoomMessage::AddBedSizeInput => {
                    self.bed_count_inputs
                        .insert(self.id_counter, BedCountInput::new(self.id_counter));
                    self.id_counter += 1;
                    Task::none()
                }
                AddRoomMessage::AddRoom => {
                    let add_room_input = self.get_input();
                    match add_room_input {
                        Ok(input) => {
                            let global_state_input = global_state.clone();

                            Task::perform(
                                async {
                                    services::add_room::add_room(global_state_input, input).await
                                },
                                move |res| match res {
                                    Ok(AddRoomResult::Added(uuid)) => {
                                        AppMessage::AddRoomMessage(AddRoomMessage::RoomAdded(uuid))
                                    }
                                    Ok(AddRoomResult::Forbidden) => AppMessage::TokenExpired,
                                    Ok(AddRoomResult::BadRequest(bad_request)) => {
                                        AppMessage::AddRoomMessage(AddRoomMessage::ShowError(
                                            bad_request,
                                        ))
                                    }
                                    Err(err) => {
                                        AppMessage::AddRoomMessage(AddRoomMessage::ShowError(err))
                                    }
                                },
                            )
                        }
                        Err(input_err) => Task::done(AppMessage::AddRoomMessage(
                            AddRoomMessage::ShowError(input_err),
                        )),
                    }
                }
                AddRoomMessage::RoomAdded(_uuid) => {
                    self.clear_inputs();
                    Task::done(show_notification("Room added", NotificationType::Success))
                }
                AddRoomMessage::ShowError(err) => {
                    self.error = err;
                    Task::none()
                }
            },
            _ => Task::none(),
        }
    }

    fn view(&self, global_state: Arc<Mutex<GlobalState>>) -> Element<AppMessage> {
        scrollable(
            column![
                text!("Add Room")
                    .align_x(Center)
                    .size(TITLE_FONT_SIZE)
                    .width(Fill),
                text_input("Room Number", self.room_number.get_text())
                    .on_input(|x| AppMessage::AddRoomMessage(AddRoomMessage::ChangeRoomNumber(x)))
                    .align_x(Center)
                    .width(TEXT_BOX_WIDTH)
                    .line_height(1.5),
                text_input("Floor", self.floor.get_text())
                    .on_input(|x| AppMessage::AddRoomMessage(AddRoomMessage::ChangeFloor(x)))
                    .align_x(Center)
                    .width(TEXT_BOX_WIDTH)
                    .line_height(1.5),
                text_input("Price", self.price.get_text())
                    .on_input(|x| AppMessage::AddRoomMessage(AddRoomMessage::ChagePrice(x)))
                    .align_x(Center)
                    .width(TEXT_BOX_WIDTH)
                    .line_height(1.5),
                text!("Bathroom type:"),
                self.bathroom_type_combo_box
                    .view(|x| AppMessage::AddRoomMessage(AddRoomMessage::ChangeBathroomType(x)))
                    .width(TEXT_BOX_WIDTH),
                self.view_bed_count_inputs(global_state),
                text!("{}", self.error)
                    .color(ERROR_COLOR)
                    .size(18)
                    .align_x(Center)
                    .width(Fill),
                button("Add")
                    .on_press(AppMessage::AddRoomMessage(AddRoomMessage::AddRoom))
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

struct BedCountInput {
    id: u64,
    count: NumberTextBox,
    bed_size: BedSizeComboBox,
}
impl BedCountInput {
    fn new(id: u64) -> Self {
        Self {
            id,
            count: NumberTextBox::new("1", 1, NumberType::PositiveInteger),
            bed_size: BedSizeComboBox::new(),
        }
    }

    fn view(&self, _global_state: Arc<Mutex<GlobalState>>) -> Element<AppMessage> {
        row![
            self.bed_size.view(
                |x| AppMessage::AddRoomMessage(AddRoomMessage::ChangeBedSize {
                    bed_size: x,
                    input_id: self.id
                })
            ),
            text_input("Count", self.count.get_text())
                .on_input(
                    |x| AppMessage::AddRoomMessage(AddRoomMessage::ChangeBedCount {
                        count: x,
                        input_id: self.id
                    })
                )
                .width(80.0)
                .align_x(Center),
            button("Remove")
                .height(30)
                .width(80)
                .on_press(AppMessage::AddRoomMessage(
                    AddRoomMessage::RemoveBedSizeInput(self.id)
                ))
        ]
        .spacing(10.0)
        .into()
    }
}
