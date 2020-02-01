use std::fs::File;
use std::{env, fs};
use std::io::Read;
use json::JsonValue;

pub fn get_settings_file() -> File {
    let settings_file_path =
        format!("{}/.auto_updater.json", env::var("HOME").unwrap());
    let settings_file_res = File::open(settings_file_path.clone());

    return match settings_file_res {
        Ok(t) => t,
        Err(_) => {
            fs::write(settings_file_path.clone(), "{}").unwrap();
            return File::open(settings_file_path.clone()).unwrap();
        }
    };
}

pub fn get_settings_json() -> JsonValue {
    let mut f = get_settings_file();
    let mut settings_string = String::new();
    f.read_to_string(&mut settings_string).unwrap();

    return json::parse(settings_string.as_str()).unwrap();
}
