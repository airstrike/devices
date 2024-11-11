use std::collections::BTreeMap;

use crate::model::Profile;
use iced::widget::{button, center, column, horizontal_space, row, text};
use iced::{Element, Length};

#[derive(Debug, Clone)]
pub enum Message {
    Select(usize),
}

pub fn view(profiles: &BTreeMap<usize, Profile>) -> Element<Message> {
    let list = profiles.iter().map(|(id, profile)| {
        row![
            text(&profile.name),
            horizontal_space(),
            button("View")
                .on_press(Message::Select(*id))
                .width(Length::Fixed(100.0)),
        ]
        .spacing(10)
        .into()
    });

    center(column![
        text("Profiles:").size(24),
        column(list).spacing(20).width(Length::Fixed(400.0)),
    ])
    .into()
}
