mod add;
mod info;
mod list;
mod upgrade;

use clap::{Parser, Subcommand};

use self::add::Add;
use self::info::Info;
use self::list::List;
use self::upgrade::Upgrade;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Add a new manager
    Add(Add),
    /// Get info about an existing manager
    Info(Info),
    /// List all existing managers
    List(List),
    /// Run an upgrade for a manager
    Upgrade(Upgrade),
}
