use crate::app_config::traits::Upgrader;
use semver::Version;
use thiserror::Error;

static UPGRADERS: &'static [&dyn Upgrader] = &[];

#[derive(Debug, Error)]
pub enum UpgradeError {
    #[error("Invalid version format: {0}")]
    InvalidVersionFormat(String),
    #[error("Cannot upgrade from version {from} to {to}: Downgrades are not supported")]
    DowngradeNotSupported { from: String, to: String },
    #[error("Breaking change detected (from: {from}, to: {to}): {message}")]
    BreakingChange {
        from: String,
        to: String,
        message: String,
    },
    #[error("No upgrade route found to version {0}")]
    NoRouteFound(String),
    #[error("Target version {0} is not supported")]
    UnsupportedTargetVersion(String),
}

pub fn upgrade_data(
    from_version: &str,
    to_version: &str,
    data: &str,
    upgraders_override: Option<&[&dyn Upgrader]>,
) -> Result<String, UpgradeError> {
    // Select upgraders to use
    let upgraders = match upgraders_override {
        Some(upgraders) => upgraders,
        None => UPGRADERS,
    };
    // Validate version format
    let from_version_sv = Version::parse(from_version)
        .map_err(|e| UpgradeError::InvalidVersionFormat(format!("Invalid from_version: {}", e)))?;
    let to_version_sv = Version::parse(to_version)
        .map_err(|e| UpgradeError::InvalidVersionFormat(format!("Invalid to_version: {}", e)))?;

    // Ensure that the from_version is less than or equal to the to_version
    if from_version_sv > to_version_sv {
        return Err(UpgradeError::DowngradeNotSupported {
            from: from_version.to_string(),
            to: to_version.to_string(),
        });
    }

    let mut current_data = data.to_string();
    let mut current_version = from_version.to_string();

    // Auto return data if no upgrade is needed
    if from_version == to_version {
        return Ok(current_data);
    }

    // Check if the target version is supported
    if !upgraders.iter().any(|u| u.version() == to_version) {
        return Err(UpgradeError::UnsupportedTargetVersion(
            to_version.to_string(),
        ));
    }

    // Run upgraders in order until we reach the target version
    for upgrader in upgraders.iter() {
        if upgrader.can_upgrade(current_version.as_str()) {
            if let Some(message) = upgrader.breaking_change_message() {
                return Err(UpgradeError::BreakingChange {
                    from: current_version.clone(),
                    to: upgrader.version().to_string(),
                    message: message.to_string(),
                });
            }
            current_version = upgrader.version().to_string();
            current_data = upgrader.upgrade(&current_data);
            if current_version == to_version {
                break;
            }
        }
    }
    if current_version != to_version {
        return Err(UpgradeError::NoRouteFound(to_version.to_string()));
    }
    Ok(current_data)
}
