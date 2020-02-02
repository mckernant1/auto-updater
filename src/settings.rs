use std::fs::File;
use std::{env, fs};
use std::io::Read;
use json::JsonValue;

const SETTINGS_FILE_PATH: String =
    format!("{}/.auto_updater.json", env::var("HOME").unwrap());

pub fn get_settings_file() -> File {

    let settings_file_res = File::open(SETTINGS_FILE_PATH.clone());

    return match settings_file_res {
        Ok(t) => t,
        Err(_) => {
            fs::write(SETTINGS_FILE_PATH.clone(), "{}").unwrap();
            return File::open(SETTINGS_FILE_PATH.clone()).unwrap();
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
    let mut f = get_settings_file();
    fs::write(SETTINGS_FILE_PATH, contents).unwrap();
}
