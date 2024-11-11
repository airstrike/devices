use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Model {
    pub profiles: BTreeMap<usize, Profile>,
    next_profile_id: usize, // simplifying generation of new IDs
}

#[derive(Debug)]
pub struct Profile {
    pub id: usize,
    pub name: String,
    pub devices: Vec<Device>,
    pub tunnels: Vec<Tunnel>,
    pub settings: ProfileSettings,
}

#[derive(Debug)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub status: DeviceStatus,
    pub settings: DeviceSettings,
}

#[derive(Debug)]
pub enum DeviceStatus {
    Online,
    Offline,
    Connecting,
}

#[derive(Debug)]
pub struct DeviceSettings {
    pub auto_connect: bool,
    pub remote_access: bool,
}

#[derive(Debug)]
pub struct ProfileSettings {
    pub auto_login: bool,
    pub default_tunnel: Option<String>,
}

#[derive(Debug)]
pub struct Tunnel {
    pub id: String,
    pub name: String,
    pub connected: bool,
}

impl Model {
    // Helper method to add new profiles
    pub fn add_profile(&mut self, profile: Profile) -> usize {
        let id = self.next_profile_id;
        self.profiles.insert(id, profile);
        self.next_profile_id += 1;
        id
    }

    // Helper method to get a profile
    pub fn get_profile(&self, id: usize) -> Option<&Profile> {
        self.profiles.get(&id)
    }
}

impl Default for Model {
    fn default() -> Self {
        let mut model = Self {
            profiles: BTreeMap::new(),
            next_profile_id: 0,
        };

        // Add example profiles
        model.add_profile(Profile {
            id: 0,
            name: "Profile 1".to_string(),
            devices: vec![
                Device {
                    id: "device1".to_string(),
                    name: "Device 1".to_string(),
                    status: DeviceStatus::Online,
                    settings: DeviceSettings {
                        auto_connect: true,
                        remote_access: false,
                    },
                },
                Device {
                    id: "device2".to_string(),
                    name: "Device 2".to_string(),
                    status: DeviceStatus::Offline,
                    settings: DeviceSettings {
                        auto_connect: false,
                        remote_access: true,
                    },
                },
            ],
            tunnels: vec![
                Tunnel {
                    id: "tunnel1".to_string(),
                    name: "Tunnel 1".to_string(),
                    connected: true,
                },
                Tunnel {
                    id: "tunnel2".to_string(),
                    name: "Tunnel 2".to_string(),
                    connected: false,
                },
            ],
            settings: ProfileSettings {
                auto_login: true,
                default_tunnel: Some("tunnel1".to_string()),
            },
        });

        model.add_profile(Profile {
            id: 1,
            name: "Profile 2".to_string(),
            devices: vec![
                Device {
                    id: "device3".to_string(),
                    name: "Device 3".to_string(),
                    status: DeviceStatus::Connecting,
                    settings: DeviceSettings {
                        auto_connect: true,
                        remote_access: false,
                    },
                },
                Device {
                    id: "device4".to_string(),
                    name: "Device 4".to_string(),
                    status: DeviceStatus::Offline,
                    settings: DeviceSettings {
                        auto_connect: false,
                        remote_access: true,
                    },
                },
            ],
            tunnels: vec![
                Tunnel {
                    id: "tunnel3".to_string(),
                    name: "Tunnel 3".to_string(),
                    connected: true,
                },
                Tunnel {
                    id: "tunnel4".to_string(),
                    name: "Tunnel 4".to_string(),
                    connected: false,
                },
            ],
            settings: ProfileSettings {
                auto_login: false,
                default_tunnel: Some("tunnel3".to_string()),
            },
        });

        model
    }
}
