use crate::templates::CONFIG_TEMPLATE;
use dirs;
use serde::Deserialize;
use std::{env, fs, process};

const DEPLIO_CONFIG_FILE_NAME: &str = ".deplio";

#[derive(Debug)]
pub enum ConfigurationError {
    HomeDirNotFound,
    IoFail(String),
    FileReadFail(String),
    DeserializationFail(String),
}

pub fn handle_command(edit: &bool, overwrite: &bool) -> Result<(), &'static str> {
    let home_dir = dirs::home_dir().expect("Unable to find home directory");
    let config_path = home_dir.join(DEPLIO_CONFIG_FILE_NAME);
    match fs::exists(&config_path) {
        Ok(true) => {
            if *overwrite {
                fs::remove_file(&config_path).expect("Unable to delete old configuration file");
                fs::write(&config_path, CONFIG_TEMPLATE)
                    .expect("Unable to create configuration file");
            } else {
                println!("Configuration already exists at: {:?}", config_path);
            }
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

pub fn load_config() -> Result<Configuration, ConfigurationError> {
    let home_dir = dirs::home_dir().ok_or(ConfigurationError::HomeDirNotFound)?;
    let config_path = home_dir.join(DEPLIO_CONFIG_FILE_NAME);
    let config_exists = fs::exists(&config_path);
    match config_exists {
        Ok(true) => (),
        Ok(false) => {
            return Ok(Configuration {
                debug: Debug {
                    synth_working_dir: None,
                    override_params: None,
                },
                defaults: Defaults {
                    deplio_server: None,
                    owner: None,
                },
            });
        }
        Err(e) => return Err(ConfigurationError::IoFail(e.to_string())),
    }
    let contents = fs::read_to_string(config_path);
    if let Err(err) = &contents {
        return Err(ConfigurationError::FileReadFail(err.to_string()));
    }

    match toml::from_str::<Configuration>(&contents.unwrap()) {
        Ok(config) => Ok(config),
        Err(err) => Err(ConfigurationError::DeserializationFail(
            err.message().to_string(),
        )),
    }
}

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub defaults: Defaults,
    pub debug: Debug,
}

#[derive(Deserialize, Debug)]
pub struct Defaults {
    pub deplio_server: Option<String>,
    pub owner: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Debug {
    pub synth_working_dir: Option<String>,
    pub override_params: Option<String>,
}
