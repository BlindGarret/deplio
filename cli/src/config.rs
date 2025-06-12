use crate::templates::CONFIG_TEMPLATE;
use dirs;
use serde::Deserialize;
use std::{env, fs, path::PathBuf, process};

const DEPLIO_CONFIG_FILE_NAME: &str = ".deplio";

#[derive(Debug)]
pub enum ConfigurationError {
    HomeDirNotFound,
    IoFail(String),
    FileReadFail(String),
    DeserializationFail(String),
}

pub fn handle_command(
    edit: &bool,
    overwrite: &bool,
    home_dir_override: Option<&str>,
) -> Result<(), &'static str> {
    let home_dir = match home_dir_override {
        Some(path) => PathBuf::from(path),
        None => dirs::home_dir().ok_or("Unable to find home directory")?,
    };
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

pub fn load_config(home_dir_override: Option<&str>) -> Result<Configuration, ConfigurationError> {
    let home_dir = match home_dir_override {
        Some(path) => PathBuf::from(path),
        None => dirs::home_dir().ok_or(ConfigurationError::HomeDirNotFound)?,
    };

    //dirs::home_dir().ok_or(ConfigurationError::HomeDirNotFound)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_handle_command_creates_new_config() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let temp_home_str = temp_dir.path().to_str().unwrap();

        let result = handle_command(&false, &false, Some(temp_home_str));
        assert!(result.is_ok());

        let config_path = temp_dir.path().join(DEPLIO_CONFIG_FILE_NAME);
        assert!(config_path.exists());

        let content = fs::read_to_string(&config_path).expect("Failed to read config");
        assert_eq!(content, CONFIG_TEMPLATE);
    }

    #[test]
    fn test_handle_command_with_existing_config_no_overwrite() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let temp_home_str = temp_dir.path().to_str().unwrap();
        let config_path = temp_dir.path().join(DEPLIO_CONFIG_FILE_NAME);
        let existing_content = "existing config content";

        // Create existing config
        fs::write(&config_path, existing_content).expect("Failed to write existing config");

        let result = handle_command(&false, &false, Some(temp_home_str));
        assert!(result.is_ok());

        // Verify existing content is preserved when overwrite is false
        let content = fs::read_to_string(&config_path).expect("Failed to read config");
        assert_eq!(content, existing_content);
    }

    #[test]
    fn test_handle_command_with_existing_config_with_overwrite() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let temp_home_str = temp_dir.path().to_str().unwrap();
        let config_path = temp_dir.path().join(DEPLIO_CONFIG_FILE_NAME);
        let existing_content = "existing config content";

        // Create existing config
        fs::write(&config_path, existing_content).expect("Failed to write existing config");

        let result = handle_command(&false, &true, Some(temp_home_str));
        assert!(result.is_ok());

        // Verify content was overwritten
        let content = fs::read_to_string(&config_path).expect("Failed to read config");
        assert_eq!(content, CONFIG_TEMPLATE);
    }

    #[test]
    fn test_handle_command_with_edit_flag_no_editor() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let temp_home_str = temp_dir.path().to_str().unwrap();

        // Ensure EDITOR is not set
        let original_editor = env::var("EDITOR");
        unsafe {
            env::remove_var("EDITOR");
        }

        let result = handle_command(&true, &false, Some(temp_home_str));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "EDITOR environment variable not set. Unable to open editor."
        );

        // Restore original EDITOR if it was set
        if let Ok(editor) = original_editor {
            unsafe {
                env::set_var("EDITOR", editor);
            }
        }
    }

    #[test]
    fn test_load_config_with_no_file_returns_default() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let temp_home_str = temp_dir.path().to_str().unwrap();

        let result = load_config(Some(temp_home_str));
        assert!(result.is_ok());

        let config = result.unwrap();
        assert!(config.debug.synth_working_dir.is_none());
        assert!(config.debug.override_params.is_none());
        assert!(config.defaults.deplio_server.is_none());
        assert!(config.defaults.owner.is_none());
    }

    #[test]
    fn test_load_config_with_valid_toml() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let temp_home_str = temp_dir.path().to_str().unwrap();
        let config_path = temp_dir.path().join(DEPLIO_CONFIG_FILE_NAME);
        let valid_toml = r#"
[defaults]
deplio_server = "https://api.deplio.com"
owner = "test-owner"

[debug]
synth_working_dir = "/tmp/deplio"
override_params = "--verbose --debug"
"#;

        fs::write(&config_path, valid_toml).expect("Failed to write config");

        let result = load_config(Some(temp_home_str));
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(
            config.defaults.deplio_server,
            Some("https://api.deplio.com".to_string())
        );
        assert_eq!(config.defaults.owner, Some("test-owner".to_string()));
        assert_eq!(
            config.debug.synth_working_dir,
            Some("/tmp/deplio".to_string())
        );
        assert_eq!(
            config.debug.override_params,
            Some("--verbose --debug".to_string())
        );
    }

    #[test]
    fn test_load_config_with_invalid_toml() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let temp_home_str = temp_dir.path().to_str().unwrap();
        let config_path = temp_dir.path().join(DEPLIO_CONFIG_FILE_NAME);
        let invalid_toml = r#"
[defaults
deplio_server = "missing closing bracket"
"#;

        fs::write(&config_path, invalid_toml).expect("Failed to write config");

        let result = load_config(Some(temp_home_str));
        assert!(result.is_err());

        match result.unwrap_err() {
            ConfigurationError::DeserializationFail(_) => {}
            _ => panic!("Expected DeserializationFail error"),
        }
    }

    #[test]
    fn test_load_config_with_partial_config() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let temp_home_str = temp_dir.path().to_str().unwrap();
        let config_path = temp_dir.path().join(DEPLIO_CONFIG_FILE_NAME);
        let partial_toml = r#"
[defaults]
owner = "partial-owner"

[debug]
"#;

        fs::write(&config_path, partial_toml).expect("Failed to write config");

        let result = load_config(Some(temp_home_str));
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.defaults.owner, Some("partial-owner".to_string()));
        assert!(config.defaults.deplio_server.is_none());
        assert!(config.debug.synth_working_dir.is_none());
        assert!(config.debug.override_params.is_none());
    }

    #[test]
    fn test_configuration_error_debug_display() {
        let errors = vec![
            ConfigurationError::HomeDirNotFound,
            ConfigurationError::IoFail("IO error".to_string()),
            ConfigurationError::FileReadFail("Read error".to_string()),
            ConfigurationError::DeserializationFail("Parse error".to_string()),
        ];

        // Test that all error variants can be formatted for debugging
        for error in errors {
            let debug_str = format!("{:?}", error);
            assert!(!debug_str.is_empty());
        }
    }

    #[test]
    fn test_config_template_is_valid_toml() {
        // Verify that the CONFIG_TEMPLATE is valid TOML that can be parsed
        let result: Result<Configuration, _> = toml::from_str(CONFIG_TEMPLATE);
        assert!(result.is_ok(), "CONFIG_TEMPLATE should be valid TOML");
    }
}
