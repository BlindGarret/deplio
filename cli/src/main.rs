use clap::{CommandFactory, Parser};
use cli::{
    config, init,
    parser::{Cli, Commands},
};

fn main() {
    let conf = config::load_config();
    println!("{:?}", conf);
    let cli = Cli::parse();

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
