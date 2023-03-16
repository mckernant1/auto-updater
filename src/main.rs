use clap::Parser;
use color_eyre::Result;
extern crate clap;

mod args;
pub mod settings;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = crate::args::Cli::parse();
    match args.command {
        args::Commands::Add(a) => a.add(),
        args::Commands::Info(i) => i.info(),
        args::Commands::List(l) => l.list(),
        args::Commands::Upgrade(u) => u.upgrade(),
    };
    Ok(())
}
