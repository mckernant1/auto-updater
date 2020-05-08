#[macro_use]
extern crate clap;
#[macro_use]
extern crate json;

mod add;
mod update;
mod settings;
mod list;
mod info;

use clap::{App, AppSettings};
use crate::add::add;
use crate::update::upgrade;
use crate::list::list;
use crate::info::info;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    if matches.is_present("add") {
        add(matches
            .subcommand_matches("add").unwrap()
            .value_of("NAME").unwrap()
            .to_string()
        )
    } else if matches.is_present("upgrade") {
        upgrade(matches
                    .subcommand_matches("upgrade").unwrap()
                    .value_of("NAME").unwrap_or("")
                    .to_string(),
                matches
                    .subcommand_matches("upgrade").unwrap()
                    .is_present("force"),
        )
    } else if matches.is_present("list") {
        list();
    } else if matches.is_present("info") {
        info(matches
            .subcommand_matches("info").unwrap()
            .value_of("NAME").unwrap()
            .to_string());
    }
}
