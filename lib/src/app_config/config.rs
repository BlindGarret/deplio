use crate::app_config::AppConfigV1_0_0;
use crate::app_config::templates::APP_CONFIG_TEMPLATE;
use regex::Regex;
use thiserror::Error;
use toml;

pub static CURRENT_VERSION: &str = "1.0.0";

/// Represents the error that can occur during deserialization of the app config.
#[derive(Debug, Error)]
pub enum DeserializationError {
    /// Error when the version of the app config does not match the current version.
    #[error("Version mismatch: expected {0}")]
    VersionMismatch(String),

    /// Error when deserializing the app config from TOML format.
    #[error("Failed to deserialize app config: {0}")]
    TomlError(String),
}

pub fn deserialize_app_config(app_config: &str) -> Result<AppConfigV1_0_0, DeserializationError> {
    let re = build_current_version_regex();
    let Some(_captures) = re.captures(app_config) else {
        return Err(DeserializationError::VersionMismatch(
            CURRENT_VERSION.to_string(),
        ));
    };

    match toml::from_str(app_config) {
        Ok(config) => Ok(config),
        Err(e) => Err(DeserializationError::TomlError(e.to_string())),
    }
}

pub fn write_app_config_template(app_name: &str, deplio_server: &str, owner: &str) -> String {
    APP_CONFIG_TEMPLATE
        .to_string()
        .replace("{{app_name}}", app_name)
        .replace("{{deplio_server}}", deplio_server)
        .replace("{{owner}}", owner)
        .replace("{{version}}", CURRENT_VERSION)
}

/// Builds a regex to match the current version in the app config.
pub fn build_current_version_regex() -> Regex {
    Regex::new(&format!(
        r#"\s*version\s*=\s*"{}"\s*"#,
        regex::escape(CURRENT_VERSION)
    ))
    .expect("Failed to create version regex")
}
