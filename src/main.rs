use clap::Parser;
use color_eyre::Result;
extern crate clap;

mod args;
pub mod settings;

use args::Commands as C;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = crate::args::Cli::parse();
    match args.command {
        C::Add(a) => a.add(),
        C::Info(i) => i.info(),
        C::List(l) => l.list(),
        C::Upgrade(u) => u.upgrade(),
        C::Remove(r) => r.remove(),
    }?;
    Ok(())
}
