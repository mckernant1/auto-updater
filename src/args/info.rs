use clap::Args;

use crate::settings::get_settings_json;
#[derive(Args, Debug, Clone)]
pub struct Info {
    /// name of the manager
    name: String,
}

impl Info {
    pub fn info(&self) {
        let name = self.name.clone();
        let json = get_settings_json();
        if !json.contains_key(&name) {
            panic!("There is no package manager with this name");
        }
        let setting = json.get(&name).unwrap();
        println!(
            "Name: {}\n\
Last updated: {}\n\
Update every: {}\n\
Next update: {},\n\
Commands: {:?}",
            name.clone(),
            setting.last_updated,
            setting.frequency,
            setting.next_trigger(),
            setting.commands
        );
    }
}
