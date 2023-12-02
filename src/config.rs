/// Just some hardcoded settings to keep the config separate from the code. To
/// support reading from config files and environment, look into serde and
/// config-rs.
use lazy_static::lazy_static;
pub struct Day02Settings {
    pub bag_contents: [u8; 3],
}

pub struct Settings {
    pub day02: Day02Settings,
}

impl Settings {
    pub fn new() -> Self {
        let day02 = Day02Settings {
            bag_contents: [12, 13, 14],
        };

        Settings { day02 }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new();
}
