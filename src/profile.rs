pub mod detail;

use crate::model;
use iced::widget::{button, column, container, horizontal_space, row, text};
use iced::Length::Fill;
use iced::{Element, Length};

#[derive(Debug, Clone)]
pub enum Message {
    Back,
    SelectDevice(String),          // Select a device within the profile
    UpdateDevice(detail::Message), // Forward device detail messages
}

#[derive(Debug, Default)]
pub struct Profile {
    selected_device: Option<String>, // Track which device is being viewed in detail
}

pub enum Action {
    Back, // Go back to profile selection
    SetAutoConnect { device_id: String, enabled: bool },
}

impl Profile {
    pub fn update(&mut self, message: Message) -> Option<Action> {
        match message {
            Message::Back => {
                if self.selected_device.is_some() {
                    self.selected_device = None;
                    None
                } else {
                    Some(Action::Back)
                }
            }
            Message::SelectDevice(device_id) => {
                self.selected_device = Some(device_id);
                None
            }
            Message::UpdateDevice(detail::Message::ToggleAutoConnect(device_id, enabled)) => {
                Some(Action::SetAutoConnect { device_id, enabled })
            }
        }
    }

    pub fn view<'a>(&'a self, profile: &'a model::Profile) -> Element<'a, Message> {
        container(
            column![
                // Add back button at the top
                button("← Back to Profiles").on_press(Message::Back),
                // Rest of the content based on selected_device
                match &self.selected_device {
                    Some(device_id) => {
                        if let Some(device) = profile.devices.iter().find(|d| d.id == *device_id) {
                            column![
                                text(&device.name).size(24),
                                detail::view(device).map(Message::UpdateDevice)
                            ]
                        } else {
                            column![text("Device not found")]
                        }
                    }
                    None => {
                        column![
                            text(&profile.name).size(24),
                            column(profile.devices.iter().map(|device| {
                                row![
                                    text(&device.name),
                                    text(format!("({:?})", device.status)),
                                    text(format!(
                                        "Auto-connect: {:?}",
                                        device.settings.auto_connect
                                    )),
                                    horizontal_space(),
                                    button("View")
                                        .on_press(Message::SelectDevice(device.id.clone()))
                                        .width(Length::Fixed(100.0))
                                ]
                                .spacing(10)
                                .into()
                            }))
                            .spacing(20),
                        ]
                    }
                }
            ]
            .width(Length::Fixed(400.0))
            .height(Fill)
            .spacing(20),
        )
        .center(Fill)
        .style(container::bordered_box)
        .padding(20)
        .into()
    }
}
