use crate::settings::get_settings_json;
use std::time::SystemTime;
use chrono::{Utc, Date, DateTime, Duration};
use std::str::Chars;
use std::process::Command;
use json::JsonValue;

pub fn upgrade(name: String, force: bool) {
    let settings_json = get_settings_json();
    if name == "" {
        let entries = settings_json.entries();
        entries.for_each(|(name, value)| {
            println!("Now running {}", name);
            let time = DateTime::parse_from_rfc3339(value["lastUpdated"].as_str().unwrap()).unwrap();
            let freq_str = value["frequency"].clone().to_string();
            let frequency = freq_str.chars();
            let time_char = frequency.clone().last().unwrap();
            let digit = frequency.clone()
                .take_while(|c| c.is_digit(10))
                .collect::<String>()
                .parse::<i64>().unwrap();
            let duration = match time_char {
                'd' => Duration::days(digit),
                'w' => Duration::weeks(digit),
                'm' => Duration::weeks(digit * 4),
                'y' => Duration::weeks(digit * 52),
                _ => panic!("Incorrect Format")
            };

            if time + duration < DateTime::from(Utc::now()) {
                run_commands(value)
            }
        })
    }
}


fn run_commands(value: &JsonValue) {
    value["commands"].members().for_each(|item| {
        let cmd_vec: Vec<&str> = item.as_str().unwrap()
            .split_whitespace().collect();
        match Command::new(cmd_vec[0])
            .args(&cmd_vec[1..])
            .spawn() {
            Ok(mut child) => {
                child.wait().unwrap();
            }
            Err(_) => {
                eprintln!("This command does not exist.")
            }
        }
    })
}

