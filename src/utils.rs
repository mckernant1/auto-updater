use json::JsonValue;
use chrono::{Duration, DateTime, FixedOffset};
use std::process;

pub fn parse_json(json: &JsonValue) -> (
    DateTime<FixedOffset>,
    String,
    DateTime<FixedOffset>,
    Vec<&str>) {
    let freq_str = json["frequency"].clone().to_string();
    let last_updated = DateTime::parse_from_rfc3339(json["lastUpdated"].as_str().unwrap()).unwrap();
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
        e => {
            println!("Invalid Character: '{}' format should be <INT><d, w, m, y>", e);
            process::exit(1);
        }
    };

    return (
        last_updated,
        freq_str,
        last_updated + duration,
        json["commands"].members()
            .map(|c| c.as_str().unwrap())
            .collect::<Vec<&str>>()
    );
}
