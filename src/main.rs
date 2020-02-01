#[macro_use]
extern crate clap;
#[macro_use]
extern crate json;

use clap::{App, AppSettings};
use std::env;
use std::fs::File;
use std::io::{stdout, stdin, Write};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();
    let file = get_settings_file();

    if matches.is_present("add") {
        add(matches
            .subcommand_matches("add").unwrap()
            .value_of("NAME").unwrap()
            .to_string()
        )
    }
}

fn get_settings_file() -> File {
    let settings_file_path = format!("{}/.pm.json", env::var("HOME").unwrap());
    let settings_file_res = File::open(settings_file_path.clone());

    return match settings_file_res {
        Ok(t) => t,
        Err(_) => File::open(settings_file_path.clone()).unwrap()
    };
}

fn add(name: String) {

    let mut command = String::new();

    while command != "\n" {
        print!("Input the path of where you clone your repos (default is $HOME/Desktop): ");
        stdout().flush().unwrap();
        stdin().read_line(&mut command).unwrap();
        dbg!(command.clone());
    };
}
