use clap::{CommandFactory, Parser};
use cli::{
    config, init,
    parser::{Cli, Commands},
};

fn main() {
    let conf = config::load_config().expect("unable to load configuration");
    let cli: Cli;
    if conf.debug.override_params.is_some() {
        let dev_params = shlex::split(&conf.debug.override_params.unwrap());
        if let Some(params) = dev_params {
            let mut padded_params = vec![""];
            let mut casted_params: Vec<&str> = params.iter().map(AsRef::as_ref).collect();
            padded_params.append(&mut casted_params);
            cli = Cli::parse_from(padded_params);
        } else {
            println!("illegal dev params provided");
            cli = Cli::parse();
        }
    } else {
        cli = Cli::parse();
    }

    match &cli.command {
        Some(Commands::Init { app_name, owner }) => init::handle_command(app_name, owner),
        Some(Commands::Update { version }) => {}
        Some(Commands::Config { edit, overwrite }) => {
            wrap_error(config::handle_command(edit, overwrite))
        }
        Some(Commands::Debug(_)) => {}
        None => {
            let mut cmd = Cli::command();
            cmd.print_help().unwrap();
        }
    }
}

fn wrap_error<T>(result: Result<T, &str>) -> T {
    match result {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
