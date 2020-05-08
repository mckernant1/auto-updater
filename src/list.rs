use crate::settings::{get_settings_json};

pub fn list() {
    let json = get_settings_json();
    json.entries().for_each(|(name, _)| println!("{}", name));
}
