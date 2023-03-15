use crate::settings::{get_settings_json, write_settings_json};
use chrono::Utc;
use clap::Args;
use json::object;
use std::io::{stdin, stdout, Write};

#[derive(Args, Debug, Clone)]
pub struct Add {
    /// The name of the new manager
    name: String,
}

impl Add {
    pub fn add(&self) {
        let how_often = self.get_how_often();
        let commands = self.get_commands(self.name.clone());
        let timestamp = Utc::now();
        let mut settings_json = get_settings_json();

        settings_json[self.name.clone()] = object! {
            "frequency" => how_often,
            "lastUpdated" => timestamp.to_rfc3339(),
            "commands" => commands
        };
        write_settings_json(json::stringify(settings_json));
    }

    fn get_how_often(&self) -> String {
        print!("enter how often you want this to be run (1w): ");
        stdout().flush().unwrap();
        let mut how_often_string = String::new();
        stdin().read_line(&mut how_often_string).unwrap();
        if how_often_string == "\n" {
            how_often_string = String::from("1w");
        }
        return how_often_string.replace("\n", "");
    }

    fn get_commands(&self, name: String) -> Vec<String> {
        println!("enter the commands for {}", name);
        println!("press enter when done.");
        let mut commands: Vec<String> = vec![];

        loop {
            print!("enter the command you wish to add: ");
            stdout().flush().unwrap();
            let mut command = String::new();
            stdin().read_line(&mut command).unwrap();
            if command == "\n" {
                break;
            }
            command = command.replace("\n", "");
            commands.push(command.clone());
        }
        return commands;
    }
}
