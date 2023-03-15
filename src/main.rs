use clap::Parser;

extern crate clap;

mod args;
pub mod settings;
pub mod utils;

fn main() {
    let args = crate::args::Cli::parse();
    match args.command {
        args::Commands::Add(a) => a.add(),
        args::Commands::Info(i) => i.info(),
        args::Commands::List(l) => l.list(),
        args::Commands::Upgrade(u) => u.upgrade(),
    }
}
