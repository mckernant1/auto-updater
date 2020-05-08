use crate::settings::{get_settings_json};
use std::process;
use chrono::{DateTime, Local};


pub fn info(name: String) {
    let json = get_settings_json();
    if !json.has_key(name.clone().as_str()) {
        println!("There is no package manager with this name");
        process::exit(1);
    }
    let member = &json[name.clone()];
    let date = DateTime::parse_from_rfc3339(member["lastUpdated"].as_str().unwrap()).unwrap();
    println!("Name: {}\n\
Last updated: {}\n\
Update every: {}\n\
Commands: {}",
        name.clone(), date.with_timezone(&Local).to_rfc2822(),
        member["frequency"], member["commands"].to_string());
}
