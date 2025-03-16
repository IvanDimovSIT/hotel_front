use std::collections::HashMap;

use iced::{
    border::Radius,
    widget::{button, column, container::Style, row, text},
    Border, Element, Theme,
};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use iced::{widget::scrollable::Viewport, Task};

use crate::model::guest::Guest;
use crate::{
    app::{AppMessage, GlobalState},
    services::get_guest::{get_guest, GetGuestResult},
};

pub struct GuestListInput {
    ids: Vec<Uuid>,
    loaded: HashMap<Uuid, Guest>,
}
impl GuestListInput {
    pub fn new() -> Self {
        const INITIAL_CAPACITY: usize = 100;
        Self {
            ids: Vec::with_capacity(INITIAL_CAPACITY),
            loaded: HashMap::with_capacity(INITIAL_CAPACITY),
        }
    }

    fn get_guest_container_style(theme: &Theme) -> Style {
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

    fn view_element<F>(&self, guest: Option<&Guest>, on_selected: F) -> Element<AppMessage>
    where
        F: Fn(Uuid) -> AppMessage,
    {
        const WIDTH: u16 = 450;
        const HEIGHT: u16 = 120;
        let guest = if let Some(some) = guest {
            some
        } else {
            return iced::widget::container(text!("Loading ...").center().size(18))
                .width(WIDTH)
                .height(HEIGHT)
                .style(Self::get_guest_container_style)
                .into();
        };

        let mut col = column![row![text!(
            "{} {}, {}",
            guest.first_name,
            guest.last_name,
            guest.date_of_birth
        )],];
        let mut optional_row = row![];
        if let Some(id_card) = &guest.id_card {
            optional_row = optional_row.push(text!("UCN: {}", id_card.ucn))
        }
        if let Some(phone) = &guest.phone_number {
            optional_row = optional_row.push(text!("Phone: {phone}"))
        }

        col = col.push(optional_row.spacing(10));
        col = col.push(button("Add").on_press(on_selected(guest.id)));

        iced::widget::container(col.spacing(5))
            .width(WIDTH)
            .height(HEIGHT)
            .style(Self::get_guest_container_style)
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

    pub fn update_ids<F>(
        &mut self,
        global_state: Arc<Mutex<GlobalState>>,
        guest_ids: Vec<Uuid>,
        map_result: F,
    ) -> Task<AppMessage>
    where
        F: Send
            + Sync
            + Fn(Result<GetGuestResult, String>) -> AppMessage
            + Clone
            + Send
            + Sync
            + 'static,
    {
        self.ids = guest_ids;
        let token = if let Some(some) = global_state
            .lock()
            .unwrap()
            .token
            .as_ref()
            .map(|t| t.token_string.clone())
        {
            some
        } else {
            return Task::done(AppMessage::TokenExpired);
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
            + Fn(Result<GetGuestResult, String>) -> AppMessage
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
            return Task::done(AppMessage::TokenExpired);
        };

        let number_to_load = (scroll_percent * self.ids.len() as f32) as usize;

        self.load_elements(token, number_to_load, map_result)
    }

    pub fn update_loaded(&mut self, guest: Guest) {
        self.loaded.insert(guest.id, guest);
    }

    fn load_elements<F>(&mut self, token: String, number: usize, map_result: F) -> Task<AppMessage>
    where
        F: Send
            + Sync
            + Fn(Result<GetGuestResult, String>) -> AppMessage
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
                Task::perform(get_guest(token_copy, id), map_result.clone())
            })
            .collect();

        Task::batch(tasks)
    }

    pub fn get_loaded(&self, id: Uuid) -> Option<Guest> {
        self.loaded.get(&id).map(|guest| guest.clone())
    }
}
