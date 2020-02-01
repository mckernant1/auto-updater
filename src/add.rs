use std::io::{stdin, stdout, Write, Read};
use crate::settings::get_settings_file;


pub fn add(name: String) {
    let how_often = get_how_often();
    let commands = get_commands();
    let mut settings_file = get_settings_file();

    let mut settings_json = String::new();
    settings_file.read_to_string(&mut settings_json).unwrap();
}


fn get_how_often() -> String {
    print!("Enter how often you want this to be run (1w): ");
    let mut how_often_string = String::new();
    stdin().read_line(&mut how_often_string).unwrap();
    if how_often_string == "\n" {
        how_often_string = String::from("1w");
    }
    return how_often_string;
}

fn get_commands() -> Vec<String> {
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
