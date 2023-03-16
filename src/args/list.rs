use clap::Args;

use crate::settings::get_settings_json;
#[derive(Args, Debug, Clone)]
pub struct List;

impl List {
    pub fn list(&self) {
        let json = get_settings_json();
        json.keys().for_each(|name| println!("{}", name));
    }
}
