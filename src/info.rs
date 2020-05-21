use crate::settings::{get_settings_json};
use std::process;
use crate::utils::parse_json;


pub fn info(name: String) {
    let json = get_settings_json();
    if !json.has_key(name.clone().as_str()) {
        println!("There is no package manager with this name");
        process::exit(1);
    }
    let member = &json[name.clone()];
    let (last_updated, freq_str, next_update, commands) = parse_json(member);
    println!("Name: {}\n\
Last updated: {}\n\
Update every: {}\n\
Next update: {},\n\
Commands: {:?}",
             name.clone(),
             last_updated,
             freq_str,
             next_update,
             commands);
}
