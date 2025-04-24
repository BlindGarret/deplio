use text_io::read;

pub fn handle_command(app_name: &Option<String>, owner: &Option<String>) {
    let app_name = app_name.clone().unwrap_or_else(|| {
        // todo: check default config for values

        // no value and no config means we need to prompt for the value
        print!("Application Name: ");
        let name: String = read!("{}\n");
        name
    });

    let owner = owner.clone().unwrap_or_else(|| {
        // todo: check default config for values

        // no value and no config means we need to prompt for the value
        print!("Owner Name: ");
        let owner: String = read!("{}\n");
        owner
    });

    println!("app_name: {}", app_name);
    println!("owner: {}", owner);
}
