use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
    process::Command,
};

use chrono::Local;
use clap::Args;

use crate::settings::{get_settings_json, write_settings_json, Setting};

#[derive(Args, Debug, Clone)]
pub struct Upgrade {
    /// Name of the manager
    name: Option<String>,

    /// Run the command regardless of time since last run
    #[arg(short, long)]
    force: bool,
}

impl Upgrade {
    pub fn upgrade(&self) {
        let mut settings_json = get_settings_json();
        match self.name.clone() {
            Some(name) if settings_json.contains_key(&name) => Self::run_command(
                HashMap::from([(&name, settings_json.get_mut(&name).unwrap())]),
                self.force,
            ),
            Some(name) => {
                println!(
                    "You do not have an obejct with the name {}\n
                If you want to add one do \n\n\tauto-updater add {}\n",
                    name.clone(),
                    name.clone()
                );
                return;
            }
            None => {
                let entries = settings_json.iter_mut();
                let now = Local::now().naive_local();
                let entries = entries
                    .filter(|(_, value)| self.force || value.next_trigger() < now)
                    .collect::<HashMap<&String, &mut Setting>>();
                if !entries.is_empty() {
                    Self::run_command(entries, self.force);
                }
            }
        }
        write_settings_json(serde_json::to_string(&settings_json).unwrap());
    }

    fn run_command(settings: HashMap<&String, &mut Setting>, force: bool) {
        let now = Local::now().naive_local();
        if force {
            for (name, mut value) in settings {
                if Self::run_commands(value.commands.clone(), name.as_str()) {
                    value.last_updated = now.clone();
                }
            }
        } else {
            print!(
                "It's time to update {}, would you like to update now (y/N/s): ",
                settings
                    .keys()
                    .cloned()
                    .cloned()
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            stdout().flush().unwrap();
            let mut update_prompt = String::new();
            stdin().read_line(&mut update_prompt).unwrap();
            if update_prompt.to_lowercase().starts_with("y") {
                for (name, mut value) in settings {
                    if Self::run_commands(value.commands.clone(), name.as_str()) {
                        value.last_updated = now.clone();
                    } else {
                        println!("Something went wrong. Good luck!")
                    }
                }
            } else if update_prompt.to_lowercase().starts_with("s") {
                for (_name, mut value) in settings {
                    value.last_updated = now.clone();
                }
            } else {
                println!("You will be prompted to update again on your next shell start");
                for (name, value) in settings {
                    println!("{} was last updated {}", name, value.last_updated);
                }
            }
        }
    }

    fn run_commands(commands: Vec<String>, name: &str) -> bool {
        println!("Now updating {} with commands {:?}", name, commands);
        let mut worked = true;
        for item in commands {
            let cmd_vec = item.split_whitespace().collect::<Vec<&str>>();
            match Command::new(cmd_vec[0]).args(&cmd_vec[1..]).spawn() {
                Ok(mut child) => {
                    child.wait().unwrap();
                }
                Err(_) => {
                    worked = false;
                    eprintln!("This command does not exist.")
                }
            }
        }
        return worked;
    }
}
