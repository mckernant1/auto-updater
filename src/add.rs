use crate::settings::{get_settings_json, write_settings_json};
use chrono::Utc;
use std::io::{stdin, stdout, Write};

pub fn add(name: string) {
    let how_often = get_how_often();
    let commands = get_commands(name.clone());
    let timestamp = utc::now();
    let mut settings_json = get_settings_json();

    settings_json[name.clone()] = object! {
        "frequency" => how_often,
        "lastupdated" => timestamp.to_rfc3339(),
        "commands" => commands
    };
    write_settings_json(json::stringify(settings_json));
}

fn get_how_often() -> string {
    print!("enter how often you want this to be run (1w): ");
    stdout().flush().unwrap();
    let mut how_often_string = string::new();
    stdin().read_line(&mut how_often_string).unwrap();
    if how_often_string == "\n" {
        how_often_string = string::from("1w");
    }
    return how_often_string.replace("\n", "");
}

fn get_commands(name: string) -> vec<string> {
    println!("enter the commands for {}", name);
    println!("press enter when done.");
    let mut commands: vec<string> = vec![];

    loop {
        print!("enter the command you wish to add: ");
        stdout().flush().unwrap();
        let mut command = string::new();
        stdin().read_line(&mut command).unwrap();
        if command == "\n" {
            break;
        }
        command = command.replace("\n", "");
        commands.push(command.clone());
    }
    return commands;
}
