use std::process;

use chrono::Local;
use clap::Args;

use crate::{settings::get_settings_json, utils::parse_json};
#[derive(Args, Debug, Clone)]
pub struct Info {
    /// name of the manager
    name: String,
}

impl Info {
    pub fn info(&self) {
        let name = self.name.clone();
        let json = get_settings_json();
        if !json.has_key(name.clone().as_str()) {
            println!("There is no package manager with this name");
            process::exit(1);
        }
        let member = &json[name.clone()];
        let (last_updated, freq_str, next_update, commands) = parse_json(member);
        println!(
            "Name: {}\n\
Last updated: {}\n\
Update every: {}\n\
Next update: {},\n\
Commands: {:?}",
            name.clone(),
            last_updated.with_timezone(&Local),
            freq_str,
            next_update.with_timezone(&Local),
            commands
        );
    }
}
