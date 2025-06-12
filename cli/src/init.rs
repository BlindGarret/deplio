use text_io::read;

use crate::config;

pub fn handle_command(
    app_name: &Option<String>,
    owner: &Option<String>,
) -> Result<(), &'static str> {
    let app_name = app_name.clone().unwrap_or_else(|| {
        // no value and no config means we need to prompt for the value
        print!("Application Name: ");
        let name: String = read!("{}\n");
        name
    });

    let config = config::load_config(None)
        .expect("No configuration found, please run config command to setup system first.");

    let owner = owner.clone().unwrap_or_else(|| {
        if let Some(default_owner) = config.defaults.owner {
            return default_owner;
        }

        // no value and no config means we need to prompt for the value
        print!("Owner Name: ");
        let owner: String = read!("{}\n");
        owner
    });

    println!("app_name: {}", app_name);
    println!("owner: {}", owner);
    Ok(())
}
