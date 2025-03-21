use iced::border::Radius;
use iced::widget::container::Style;
use iced::widget::{button, column, container, row};
use iced::Length::Fill;
use iced::{Background, Border, Color, Element, Padding, Shadow, Theme, Vector};
use std::sync::{Arc, Mutex};

use crate::app::{AppMessage, GlobalState, Screen, ScreenType};
use crate::styles::NAVIGATION_BUTTON_WIDTH;

const PADDING_SIZE: f32 = 5.0;
const BUTTON_SPACING: f32 = 6.0;
const BORDER_SIZE: f32 = 1.0;
const BORDER_RADIUS: f32 = 8.0;
const SHADOW_OFFSET: Vector<f32> = Vector::new(5.0, 5.0);
const SHADOW_BLUR_RADIUS: f32 = 10.0;

pub fn view_admin(
    global_state: Arc<Mutex<GlobalState>>,
    current_screen: &dyn Screen,
) -> Element<AppMessage> {
    row![
        add_container(
            column![
                button("Logout")
                    .on_press(AppMessage::Logout)
                    .width(NAVIGATION_BUTTON_WIDTH),
                button("Add Room")
                    .on_press(AppMessage::NavigateTo(ScreenType::AddRoom))
                    .width(NAVIGATION_BUTTON_WIDTH),
                button("Add Guest")
                    .on_press(AppMessage::NavigateTo(ScreenType::AddGuest))
                    .width(NAVIGATION_BUTTON_WIDTH),
                button("Book room")
                    .on_press(AppMessage::NavigateTo(ScreenType::BookRoom))
                    .width(NAVIGATION_BUTTON_WIDTH),
            ]
            .spacing(BUTTON_SPACING)
            .into()
        ),
        current_screen.view(global_state)
    ]
    .height(Fill)
    .into()
}

pub fn view_user(
    global_state: Arc<Mutex<GlobalState>>,
    current_screen: &dyn Screen,
) -> Element<AppMessage> {
    row![
        add_container(
            column![
                button("Logout")
                    .on_press(AppMessage::Logout)
                    .width(NAVIGATION_BUTTON_WIDTH),
                button("Placeholder2").width(NAVIGATION_BUTTON_WIDTH),
                button("Placeholder3").width(NAVIGATION_BUTTON_WIDTH),
            ]
            .spacing(BUTTON_SPACING)
            .into()
        ),
        current_screen.view(global_state)
    ]
    .height(Fill)
    .into()
}

fn get_style(theme: &Theme) -> Style {
    let background_color = theme.palette().primary.scale_alpha(0.4);

    Style {
        shadow: Shadow {
            color: Color::BLACK,
            offset: SHADOW_OFFSET,
            blur_radius: SHADOW_BLUR_RADIUS,
        },
        border: Border {
            color: Color::BLACK,
            width: BORDER_SIZE,
            radius: Radius {
                top_left: BORDER_RADIUS,
                top_right: BORDER_RADIUS,
                bottom_right: BORDER_RADIUS,
                bottom_left: BORDER_RADIUS,
            },
        },
        background: Some(Background::Color(background_color)),
        ..Default::default()
    }
}

fn add_container(element: Element<AppMessage>) -> Element<AppMessage> {
    container(element)
        .style(get_style)
        .padding(Padding {
            top: PADDING_SIZE,
            right: PADDING_SIZE,
            bottom: PADDING_SIZE,
            left: PADDING_SIZE,
        })
        .height(Fill)
        .into()
}
