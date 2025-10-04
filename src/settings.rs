use chrono::{Duration, NaiveDateTime};
use color_eyre::eyre::eyre;
use color_eyre::Result;
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
    pub fn next_trigger(&self) -> Result<NaiveDateTime> {
        let frequency = self.frequency.chars();
        let time_char = frequency.clone().last().unwrap();
        let digit = frequency
            .clone()
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<i64>()?;
        let duration = match time_char {
            'd' => Duration::days(digit),
            'w' => Duration::weeks(digit),
            'm' => Duration::weeks(digit * 4),
            'y' => Duration::weeks(digit * 52),
            e => {
                return Err(eyre!(
                    "Invalid Character: '{}' format should be <INT><d, w, m, y>",
                    e
                ));
            }
        };

        Ok(self.last_updated + duration)
    }
}

pub fn get_settings_file() -> Result<File> {
    let path = format!("{}/.auto_updater.json", env::var("HOME")?);
    let settings_file_res = File::open(path.clone());

    match settings_file_res {
        Ok(t) => Ok(t),
        Err(_) => {
            fs::write(path.clone(), "{}")?;
            Ok(File::open(path.clone())?)
        }
    }
}

pub fn get_settings_json() -> Result<HashMap<String, Setting>> {
    let mut f = get_settings_file()?;
    let mut settings_string = String::new();
    f.read_to_string(&mut settings_string)?;

    Ok(serde_json::from_str(settings_string.as_str())?)
}

pub fn write_settings_json(settings: HashMap<String, Setting>) -> Result<()> {
    let contents = serde_json::to_string(&settings)?;
    let path = format!("{}/.auto_updater.json", env::var("HOME")?);
    fs::write(path, contents)?;
    Ok(())
}
