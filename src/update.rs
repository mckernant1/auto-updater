use crate::settings::{get_settings_json, write_settings_json};

use chrono::{Utc, DateTime, FixedOffset};

use std::process::Command;
use json::JsonValue;
use std::io::{stdout, stdin, Write};
use std::process;
use crate::utils::parse_json;


pub fn upgrade(name: String, force: bool) {
    let mut settings_json = get_settings_json();
    if name == "" {
        let entries = settings_json.entries_mut();
        entries.for_each(|(name, value)| {
            run_command(value, force, name);
        });
    } else if settings_json.has_key(name.clone().as_str()) {
        run_command(&mut settings_json[name.clone()], force, name.as_str());
    } else {
        println!("You do not have an obejct with the name {}\n\
        If you want to add one do \n\n\tauto-updater add {}\n", name.clone(), name.clone());
        process::exit(0);
    }
    write_settings_json(json::stringify(settings_json));
}


fn run_command(value: &mut JsonValue, force: bool, name: &str) {
    let (last_updated,
        _,
        next_update,
        commands) = parse_json(value);
    let today_timestamp = JsonValue::from(Utc::now().to_rfc3339());
    if force {
        if run_commands(commands, name) {
            value["lastUpdated"] = today_timestamp.clone();
        }
    } else if next_update < (DateTime::from(Utc::now()) as DateTime<FixedOffset>) {
        print!("It's time to update {}, would you like to update now (y/N/s): ", name);
        stdout().flush().unwrap();
        let mut update_prompt = String::new();
        stdin().read_line(&mut update_prompt).unwrap();
        if update_prompt.to_lowercase().starts_with("y") {
            if run_commands(commands, name) {
                value["lastUpdated"] = today_timestamp.clone();
            } else {
                println!("Something went wrong. Good luck!")
            }
        } else if update_prompt.to_lowercase().starts_with("s") {
            value["lastUpdated"] = today_timestamp.clone();
        } else {
            println!("You will be prompted to update again on your next shell start");
            println!("{} was last updated {}", name, last_updated)
        }
    }
}

fn run_commands(commands: Vec<&str>, name: &str) -> bool {
    println!("Now updating {} with commands {:?}", name, commands);
    let mut worked = true;
    for item in commands {
        let cmd_vec = item.split_whitespace().collect::<Vec<&str>>();
        match Command::new(cmd_vec[0])
            .args(&cmd_vec[1..])
            .spawn() {
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

