// main.rs
mod model;
mod profile;
mod selector;

use iced::widget::{center, text};
use iced::{Element, Size, Task};
use model::Model;

fn main() -> iced::Result {
    iced::application("iced devices example", App::update, App::view)
        .window_size(Size::new(800.0, 600.0))
        .centered()
        .antialiasing(true)
        .run_with(App::new)
}

#[derive(Debug)]
pub enum Message {
    LoadProfiles(Result<model::Model, String>),
    Profile(profile::Message),
    Selector(selector::Message),
}

pub struct App {
    screen: Screen,
}

#[derive(Default)]
pub enum Screen {
    #[default]
    Loading,
    Loaded {
        model: Model,
        selected: Option<usize>,
        viewer: profile::Profile,
    },
}

// simulate some slow loading of profiles and return a Result<Vec<String>, String>
async fn load_profiles() -> Result<model::Model, String> {
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    Ok(model::Model::default())
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                screen: Default::default(),
            },
            Task::perform(load_profiles(), Message::LoadProfiles),
        )
    }

    pub fn view(&self) -> Element<Message> {
        match &self.screen {
            Screen::Loading => center(text("Loading... (spinner goes here)")).into(),
            Screen::Loaded {
                selected,
                viewer,
                model,
            } => match selected {
                Some(usize) => viewer.view(&model.profiles[&usize]).map(Message::Profile),
                None => selector::view(&model.profiles).map(Message::Selector),
            },
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::LoadProfiles(Ok(model)) => {
                self.screen = Screen::Loaded {
                    model,
                    selected: None,
                    viewer: profile::Profile::default(),
                };
            }
            Message::LoadProfiles(Err(_)) => {
                // Handle error case
            }
            Message::Selector(selector::Message::Select(id)) => {
                if let Screen::Loaded { selected, .. } = &mut self.screen {
                    *selected = Some(id);
                }
            }
            Message::Profile(msg) => {
                if let Screen::Loaded {
                    model,
                    selected,
                    viewer,
                } = &mut self.screen
                {
                    if let Some(action) = viewer.update(msg) {
                        match action {
                            profile::Action::Back => {
                                *selected = None; // Go back to profile selection
                            }
                            profile::Action::SetAutoConnect { device_id, enabled } => {
                                // Update device settings in the model
                                for profile in model.profiles.values_mut() {
                                    for device in &mut profile.devices {
                                        if device.id == device_id {
                                            device.settings.auto_connect = enabled;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
