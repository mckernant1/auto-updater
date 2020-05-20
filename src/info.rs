use crate::settings::{get_settings_json};
use std::process;
use chrono::{DateTime, Local, Duration};


pub fn info(name: String) {
    let json = get_settings_json();
    if !json.has_key(name.clone().as_str()) {
        println!("There is no package manager with this name");
        process::exit(1);
    }
    let member = &json[name.clone()];
    let date = DateTime::parse_from_rfc3339(member["lastUpdated"].as_str().unwrap()).unwrap();
    let freq_string =  member["frequency"].clone().to_string();
    let frequency = freq_string.chars();
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
        e => {
            println!("Invalid Character: '{}' format should be <INT><d, w, m, y>", e);
            process::exit(1);
        }
    };
    let next_update = date + duration;

    println!("Name: {}\n\
Last updated: {}\n\
Update every: {}\n\
Next update: {},\n\
Commands: {}",
        name.clone(), date.with_timezone(&Local).to_rfc2822(),
        frequency.collect::<String>(), 
        next_update.with_timezone(&Local).to_rfc2822(),
        member["commands"].to_string());
}
