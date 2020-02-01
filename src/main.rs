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
    // Get the commands to run when called
    println!("Enter the commands for {}", name);
    println!("Press enter when done.");
    let mut command = String::new();
    let mut commands: Vec<String> = vec![];
    loop {
        print!("Enter the command you wish to add: ");
        command = String::new();
        stdout().flush().unwrap();
        stdin().read_line(&mut command).unwrap();
        if command == "\n" {
            break;
        }
        command = command.replace("\n", "");
        commands.push(command.clone());
    };
    dbg!(commands);
    // Get how often to run the command
    
}
