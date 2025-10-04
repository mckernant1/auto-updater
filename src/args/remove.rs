use crate::settings::{get_settings_json, write_settings_json};
use clap::Args;
use color_eyre::eyre::eyre;
use color_eyre::Result;

#[derive(Args, Debug, Clone)]
pub struct Remove {
    /// The name of the manager to remove
    name: String,
}

impl Remove {
    pub fn remove(&self) -> Result<()> {
        let mut settings_json = get_settings_json()?;

        if !settings_json.contains_key(self.name.as_str()) {
            return Err(eyre!(
                "{} manager does not exist. See available with auto-updater list",
                self.name
            ));
        }

        settings_json.remove(self.name.as_str());

        write_settings_json(settings_json)?;
        Ok(())
    }
}
