use crate::app_config::traits::Upgrader;
use semver::Version;

static UPGRADERS: &'static [&dyn Upgrader] = &[];

pub fn upgrade_data(
    from_version: &str,
    to_version: &str,
    data: &str,
    upgraders_override: Option<&[&dyn Upgrader]>,
) -> Result<String, String> {
    // Select upgraders to use
    let upgraders = match upgraders_override {
        Some(upgraders) => upgraders,
        None => UPGRADERS,
    };
    // Validate version format
    let from_version_sv =
        Version::parse(from_version).map_err(|e| format!("Invalid from_version: {}", e))?;
    let to_version_sv =
        Version::parse(to_version).map_err(|e| format!("Invalid to_version: {}", e))?;

    // Ensure that the from_version is less than or equal to the to_version
    if from_version_sv > to_version_sv {
        return Err(format!(
            "Cannot upgrade from version {} to {}: from_version must be less than or equal to to_version",
            from_version, to_version
        ));
    }

    let mut current_data = data.to_string();
    let mut current_version = from_version.to_string();

    // Auto return data if no upgrade is needed
    if from_version == to_version {
        return Ok(current_data);
    }

    // Check if the target version is supported
    if !upgraders.iter().any(|u| u.version() == to_version) {
        return Err(format!("Target version {} is not supported", to_version));
    }

    // Run upgraders in order until we reach the target version
    for upgrader in upgraders.iter() {
        if upgrader.can_upgrade(current_version.as_str()) {
            if let Some(message) = upgrader.breaking_change_message() {
                eprintln!("Breaking change detected: {}", message);
                return Err(format!(
                    "Cannot upgrade from version {} to {}: {}",
                    current_version,
                    upgrader.version(),
                    message
                ));
            }
            current_version = upgrader.version().to_string();
            current_data = upgrader.upgrade(&current_data);
            if current_version == to_version {
                break;
            }
        }
    }
    if current_version != to_version {
        return Err(format!(
            "Could not upgrade to target version {}: no applicable upgrade route found",
            to_version
        ));
    }
    Ok(current_data)
}
