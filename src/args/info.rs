use clap::Args;
use color_eyre::eyre::eyre;
use color_eyre::Result;

use crate::settings::get_settings_json;
#[derive(Args, Debug, Clone)]
pub struct Info {
    /// name of the manager
    name: String,
}

impl Info {
    pub fn info(&self) -> Result<()> {
        let name = self.name.clone();
        let json = get_settings_json()?;
        if !json.contains_key(&name) {
            Err(eyre!("There is no package manager with this name"))?;
        }
        let setting = json.get(&name).unwrap();
        println!(
            "Name: {}\n\
Last updated: {}\n\
Update every: {}\n\
Next update: {},\n\
Commands: {:?}",
            name,
            setting.last_updated,
            setting.frequency,
            setting.next_trigger()?,
            setting.commands
        );
        Ok(())
    }
}
