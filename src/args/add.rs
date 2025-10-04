use crate::settings::{get_settings_json, write_settings_json, Setting};
use chrono::Local;
use clap::Args;
use color_eyre::Result;
use std::io::{stdin, stdout, Write};

#[derive(Args, Debug, Clone)]
pub struct Add {
    /// The name of the new manager
    name: String,
}

impl Add {
    pub fn add(&self) -> Result<()> {
        let how_often = self.get_how_often()?;
        let commands = self.get_commands(self.name.clone())?;
        let timestamp = Local::now().naive_local();
        let mut settings_json = get_settings_json()?;

        settings_json.insert(
            self.name.clone(),
            Setting {
                frequency: how_often,
                last_updated: timestamp,
                commands,
            },
        );

        write_settings_json(settings_json)?;
        Ok(())
    }

    fn get_how_often(&self) -> Result<String> {
        print!("enter how often you want this to be run (1w): ");
        stdout().flush()?;
        let mut how_often_string = String::new();
        stdin().read_line(&mut how_often_string).unwrap();
        if how_often_string == "\n" {
            how_often_string = String::from("1w");
        }
        Ok(how_often_string.replace("\n", ""))
    }

    fn get_commands(&self, name: String) -> Result<Vec<String>> {
        println!("enter the commands for {}", name);
        println!("press enter when done.");
        let mut commands: Vec<String> = vec![];

        loop {
            print!("enter the command you wish to add: ");
            stdout().flush()?;
            let mut command = String::new();
            stdin().read_line(&mut command)?;
            if command == "\n" {
                break;
            }
            command = command.replace("\n", "");
            commands.push(command.clone());
        }
        Ok(commands)
    }
}
