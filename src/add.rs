use std::io::{stdin, stdout, Write};
use crate::settings::{get_settings_json, write_settings_json};
use chrono::Utc;


pub fn add(name: String) {
    let how_often = get_how_often();
    let commands = get_commands(name.clone());
    let timestamp = Utc::now();
    let mut settings_json = get_settings_json();

    settings_json[name.clone()] = object! {
        "frequency" => how_often,
        "lastUpdated" => timestamp.to_rfc3339(),
        "commands" => commands
    };
    write_settings_json(json::stringify(settings_json));
}


fn get_how_often() -> String {
    print!("Enter how often you want this to be run (1w): ");
    stdout().flush().unwrap();
    let mut how_often_string = String::new();
    stdin().read_line(&mut how_often_string).unwrap();
    if how_often_string == "\n" {
        how_often_string = String::from("1w");
    }
    return how_often_string.replace("\n", "");
}

fn get_commands(name: String) -> Vec<String> {
    println!("Enter the commands for {}", name);
    println!("Press enter when done.");
    let mut commands: Vec<String> = vec![];

    loop {
        print!("Enter the command you wish to add: ");
        stdout().flush().unwrap();
        let mut command = String::new();
        stdin().read_line(&mut command).unwrap();
        if command == "\n" {
            break;
        }
        command = command.replace("\n", "");
        commands.push(command.clone());
    };
    return commands;
}
