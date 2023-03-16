use chrono::{Duration, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::{env, fs};

#[derive(Serialize, Deserialize, Debug)]
pub struct Setting {
    pub frequency: String,
    #[serde(rename = "lastUpdated")]
    pub last_updated: NaiveDateTime,
    pub commands: Vec<String>,
}

impl Setting {
    pub fn next_trigger(&self) -> NaiveDateTime {
        let frequency = self.frequency.chars();
        let time_char = frequency.clone().last().unwrap();
        let digit = frequency
            .clone()
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<i64>()
            .unwrap();
        let duration = match time_char {
            'd' => Duration::days(digit),
            'w' => Duration::weeks(digit),
            'm' => Duration::weeks(digit * 4),
            'y' => Duration::weeks(digit * 52),
            e => {
                panic!(
                    "Invalid Character: '{}' format should be <INT><d, w, m, y>",
                    e
                );
            }
        };

        self.last_updated + duration
    }
}

pub fn get_settings_file() -> File {
    let path = format!("{}/.auto_updater.json", env::var("HOME").unwrap());
    let settings_file_res = File::open(path.clone());

    return match settings_file_res {
        Ok(t) => t,
        Err(_) => {
            fs::write(path.clone(), "{}").unwrap();
            return File::open(path.clone()).unwrap();
        }
    };
}

pub fn get_settings_json() -> HashMap<String, Setting> {
    let mut f = get_settings_file();
    let mut settings_string = String::new();
    f.read_to_string(&mut settings_string).unwrap();

    return serde_json::from_str(settings_string.as_str()).unwrap();
}

pub fn write_settings_json(contents: String) {
    let path = format!("{}/.auto_updater.json", env::var("HOME").unwrap());
    fs::write(path, contents).unwrap();
}
