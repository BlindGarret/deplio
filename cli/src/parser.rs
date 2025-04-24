use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "deplio")]
#[command(version = "0.1")]
#[command(about = "Setup tool for deplio projects")]
#[command(long_about = "Setup tool for deplio projects. Sets up configuration,
    project initialzation, github actions, handles updates to configuration version
")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(
        about,
        long_about = "Creates the deplio configuration file for the user"
    )]
    Config {
        #[arg(
            short,
            long,
            help = "Open the configuration file in the default editor"
        )]
        edit: bool,
    },
    #[command(
        about = "Initializes the project files for a given repository using the latest version"
    )]
    #[command(long_about = "
Initializes the project files for a given repository using the latest version.

This includes the project configuration and the github actions. 
If requirements are provided with the call, or through configuration, they will be prompted for.")]
    Init {
        #[arg(short, long, help = "The name of the application to initialize")]
        app_name: Option<String>,
        #[arg(short, long, help = "The owner of the project to initialize")]
        owner: Option<String>,
    },
    Update {
        #[arg(
            short,
            long,
            help = "The version to upgrade to. If not included 'latest' is assumed"
        )]
        version: Option<String>,
    },
    Debug(Debug),
}

#[derive(Parser)]
#[command(about = "A set of debug commands useful for development on the project")]
pub struct Debug {
    #[command(subcommand)]
    pub subcommand: Option<DebugCommands>,
}

#[derive(Subcommand)]
pub enum DebugCommands {
    #[command(
        about = "Creates a backup of generated project files for quick restore when testing upgrade features"
    )]
    ProjBackup,
    #[command(about = "Restores a backup of generated project files")]
    ProjRestore {
        #[arg(short, long = "Clear the backup when restored")]
        purge: bool,
    },
}
