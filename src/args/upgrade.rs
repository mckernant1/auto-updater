use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
    process::Command,
};

use chrono::Local;
use clap::Args;
use color_eyre::eyre::eyre;
use color_eyre::Result;

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
    pub fn upgrade(&self) -> Result<()> {
        let mut settings_json = get_settings_json()?;
        match self.name.clone() {
            Some(name) if settings_json.contains_key(&name) => Self::run_command(
                HashMap::from([(&name, settings_json.get_mut(&name).unwrap())]),
                self.force,
            )?,
            Some(name) => {
                return Err(eyre!(
                    "You do not have an object with the name {}\n
                If you want to add one do \n\n\tauto-updater add {}\n",
                    name,
                    name
                ));
            }
            None => {
                let entries = settings_json.iter_mut();
                let now = Local::now().naive_local();

                let mut entries_to_run: HashMap<&String, &mut Setting> = HashMap::new();
                for (command, setting) in entries {
                    if self.force || setting.next_trigger()? < now {
                        entries_to_run.insert(command, setting);
                    }
                }
                if !entries_to_run.is_empty() {
                    Self::run_command(entries_to_run, self.force)?;
                }
            }
        };
        write_settings_json(settings_json)?;
        Ok(())
    }

    fn run_command(settings: HashMap<&String, &mut Setting>, force: bool) -> Result<()> {
        let now = Local::now().naive_local();
        if force {
            for (name, value) in settings {
                Self::run_commands(value.commands.clone(), name.as_str())?;
                value.last_updated = now;
            }
        } else {
            print!(
                "It's time to update {}, would you like to update now (y/N/s): ",
                settings
                    .keys()
                    .map(|k| k.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            stdout().flush()?;
            let mut update_prompt = String::new();
            stdin().read_line(&mut update_prompt)?;
            if update_prompt.to_lowercase().starts_with("y") {
                for (name, value) in settings {
                    Self::run_commands(value.commands.clone(), name.as_str())?;
                    value.last_updated = now;
                }
            } else if update_prompt.to_lowercase().starts_with("s") {
                for (_name, value) in settings {
                    value.last_updated = now;
                }
            } else {
                println!("You will be prompted to update again on your next shell start");
                for (name, value) in settings {
                    println!("{} was last updated {}", name, value.last_updated);
                }
            }
        }
        Ok(())
    }

    fn run_commands(commands: Vec<String>, name: &str) -> Result<()> {
        println!("Now updating {} with commands {:?}", name, commands);
        for item in commands {
            let cmd_vec = item.split_whitespace().collect::<Vec<&str>>();
            let mut child = Command::new(cmd_vec[0]).args(&cmd_vec[1..]).spawn()?;
            child.wait()?;
        }
        Ok(())
    }
}
