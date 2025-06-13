static APP_CONFIG_TEMPLATE: &str = include_str!("../../templates/app_config.toml");

pub static CURRENT_VERSION: &str = "1.0.0";

pub fn write_app_config_template(app_name: &str, deplio_server: &str, owner: &str) -> String {
    APP_CONFIG_TEMPLATE
        .to_string()
        .replace("{{app_name}}", app_name)
        .replace("{{deplio_server}}", deplio_server)
        .replace("{{owner}}", owner)
        .replace("{{version}}", CURRENT_VERSION)
}
