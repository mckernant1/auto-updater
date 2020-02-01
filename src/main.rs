#[macro_use]
extern crate clap;
#[macro_use]
extern crate json;

mod add;
mod update;
mod settings;

use clap::{App, AppSettings};
use std::env;
use std::fs::File;
use std::io::{stdout, stdin, Write};
use crate::add::{add};

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
    }
}
