use clap::{CommandFactory, Parser};
use cli::{
    config, init,
    parser::{Cli, Commands},
};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init { app_name, owner }) => init::handle_command(app_name, owner),
        Some(Commands::Update { version }) => {}
        Some(Commands::Config { edit }) => wrap_error(config::handle_command(edit)),
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
