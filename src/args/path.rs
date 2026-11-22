use clap::Args;
use color_eyre::Result;
use std::env;

#[derive(Args, Debug, Clone)]
pub struct Path;

impl Path {
    pub fn path(&self) -> Result<()> {
        let path = format!("{}/.auto_updater.json", env::var("HOME")?);
        println!("{}", path);
        Ok(())
    }
}
