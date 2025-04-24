use crate::templates::CONFIG_TEMPLATE;
use dirs;
use serde::Deserialize;
use std::{env, fs, process};

pub fn handle_command(edit: &bool) -> Result<(), &'static str> {
    let home_dir = dirs::home_dir().expect("Unable to find home directory");
    let config_path = home_dir.join(".deplio");
    match fs::exists(&config_path) {
        Ok(true) => {
            println!("Configuration already exists at: {:?}", config_path);
        }
        Ok(false) => {
            fs::write(&config_path, CONFIG_TEMPLATE).expect("Unable to create configuration file");
        }
        Err(e) => {
            panic!("Error accessing deplio configuration: {}", e);
        }
    }

    if *edit {
        match env::var("EDITOR") {
            Ok(editor) => {
                println!("Opening configuration file in editor: {}", editor);
                process::Command::new(editor)
                    .arg(&config_path)
                    .status()
                    .expect("Failed to open editor");
            }
            Err(_) => {
                return Err("EDITOR environment variable not set. Unable to open editor.");
            }
        }
    }

    Ok(())
}

#[derive(Deserialize)]
struct Configuration {
    defaults: Defaults,
    debug: Debug,
}

#[derive(Deserialize)]
struct Defaults {}

#[derive(Deserialize)]
struct Debug {}
