use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use iced::{
    border::Radius,
    widget::{button, column, container::Style, row, scrollable::Viewport, text},
    Border, Element, Task, Theme,
};
use uuid::Uuid;

use crate::{
    app::{AppMessage, GlobalState},
    model::room::Room,
    services::get_room::{get_room, GetRoomResult},
};

pub struct RoomListInput {
    ids: Vec<Uuid>,
    selected: Option<Uuid>,
    loaded: HashMap<Uuid, Room>,
}
impl RoomListInput {
    pub fn new() -> Self {
        const INITIAL_CAPACITY: usize = 100;
        Self {
            ids: Vec::with_capacity(INITIAL_CAPACITY),
            loaded: HashMap::with_capacity(INITIAL_CAPACITY),
            selected: None,
        }
    }

    fn get_room_container_style(theme: &Theme) -> Style {
        let color = theme.palette().primary;

        Style {
            border: Border {
                color,
                width: 2.0,
                radius: Radius::new(4),
            },
            ..Default::default()
        }
    }

    fn view_element<F>(&self, room: Option<&Room>, on_selected: F) -> Element<AppMessage>
    where
        F: Fn(Uuid) -> AppMessage,
    {
        const WIDTH: u16 = 450;
        const HEIGHT: u16 = 120;
        let room = if let Some(some) = room {
            some
        } else {
            return iced::widget::container(text!("Loading ...").center().size(18))
                .width(WIDTH)
                .height(HEIGHT)
                .style(Self::get_room_container_style)
                .into();
        };

        let message = on_selected(room.id);
        let select_button = if Some(room.id) == self.selected {
            button("Selected")
        } else {
            button("Select").on_press(message)
        };

        iced::widget::container(column![
            row![text!("{} floor:{}", room.room_number, room.floor)],
            select_button
        ])
        .width(WIDTH)
        .height(HEIGHT)
        .style(Self::get_room_container_style)
        .padding(5)
        .into()
    }

    pub fn view<F, S>(&self, on_selected: F, on_scroll: S) -> Element<AppMessage>
    where
        F: Fn(Uuid) -> AppMessage,
        S: Fn(Viewport) -> AppMessage + 'static,
    {
        if self.ids.is_empty() {
            return column![].into();
        }

        let rooms: Vec<_> = self.ids.iter().map(|id| self.loaded.get(id)).collect();

        let mut room_views = column![];
        for room in rooms {
            room_views = room_views.push(self.view_element(room, &on_selected));
        }

        iced::widget::scrollable(room_views.spacing(5))
            .height(350)
            .spacing(10)
            .on_scroll(on_scroll)
            .into()
    }

    pub fn get_selected(&self) -> Option<Uuid> {
        self.selected
    }

    pub fn set_selected(&mut self, selected: Option<Uuid>) {
        self.selected = selected;
    }

    pub fn update_ids<F>(
        &mut self,
        global_state: Arc<Mutex<GlobalState>>,
        room_ids: Vec<Uuid>,
        map_result: F,
    ) -> Task<AppMessage>
    where
        F: Send
            + Sync
            + Fn(Result<GetRoomResult, String>) -> AppMessage
            + Clone
            + Send
            + Sync
            + 'static,
    {
        self.selected = None;
        self.ids = room_ids;
        let token = if let Some(some) = global_state
            .lock()
            .unwrap()
            .token
            .as_ref()
            .map(|t| t.token_string.clone())
        {
            some
        } else {
            return Task::done(map_result(Ok(GetRoomResult::Forbidden)));
        };

        const INITIAL_LOAD: usize = 5;

        self.load_elements(token, INITIAL_LOAD, map_result)
    }

    pub fn load_scrolled<F>(
        &mut self,
        global_state: Arc<Mutex<GlobalState>>,
        scroll_percent: f32,
        map_result: F,
    ) -> Task<AppMessage>
    where
        F: Send
            + Sync
            + Fn(Result<GetRoomResult, String>) -> AppMessage
            + Clone
            + Send
            + Sync
            + 'static,
    {
        let token = if let Some(some) = global_state
            .lock()
            .unwrap()
            .token
            .as_ref()
            .map(|t| t.token_string.clone())
        {
            some
        } else {
            return Task::done(map_result(Ok(GetRoomResult::Forbidden)));
        };

        let number_to_load = (scroll_percent * self.ids.len() as f32) as usize;

        self.load_elements(token, number_to_load, map_result)
    }

    pub fn update_loaded(&mut self, room: Room) {
        self.loaded.insert(room.id, room);
    }

    fn load_elements<F>(&mut self, token: String, number: usize, map_result: F) -> Task<AppMessage>
    where
        F: Send
            + Sync
            + Fn(Result<GetRoomResult, String>) -> AppMessage
            + Clone
            + Send
            + Sync
            + 'static,
    {
        const EXTRA_TO_LOAD: usize = 5;
        let elements_to_load = (number + EXTRA_TO_LOAD).clamp(0, self.ids.len());
        let to_load: Vec<_> = self
            .ids
            .iter()
            .take(elements_to_load)
            .filter_map(|uuid| {
                if self.loaded.contains_key(uuid) {
                    None
                } else {
                    Some(*uuid)
                }
            })
            .collect();

        let tasks: Vec<_> = to_load
            .into_iter()
            .map(|id| {
                let token_copy = token.clone();
                Task::perform(get_room(token_copy, id), map_result.clone())
            })
            .collect();

        Task::batch(tasks)
    }
}
