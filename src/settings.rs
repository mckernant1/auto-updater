use json::JsonValue;
use std::fs::File;
use std::io::Read;
use std::{env, fs};

pub struct Setting {}

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

pub fn get_settings_json() -> JsonValue {
    let mut f = get_settings_file();
    let mut settings_string = String::new();
    f.read_to_string(&mut settings_string).unwrap();

    return json::parse(settings_string.as_str()).unwrap();
}

pub fn write_settings_json(contents: String) {
    let path = format!("{}/.auto_updater.json", env::var("HOME").unwrap());
    fs::write(path, contents).unwrap();
}
