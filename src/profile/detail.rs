use crate::model::Device;
use iced::widget::{column, text, toggler};
use iced::Element;

#[derive(Debug, Clone)]
pub enum Message {
    ToggleAutoConnect(String, bool),
}

pub fn view(device: &Device) -> Element<Message> {
    column![
        text(&device.name),
        toggler(device.settings.auto_connect)
            .on_toggle(|b| Message::ToggleAutoConnect(device.id.clone(), b)),
    ]
    .spacing(10)
    .into()
}
