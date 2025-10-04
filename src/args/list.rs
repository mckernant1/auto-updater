use crate::settings::get_settings_json;
use clap::Args;
use color_eyre::Result;
#[derive(Args, Debug, Clone)]
pub struct List;

impl List {
    pub fn list(&self) -> Result<()> {
        let json = get_settings_json()?;
        json.keys().for_each(|name| println!("{}", name));
        Ok(())
    }
}
