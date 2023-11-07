pub mod anime;

use clap::{Parser, Subcommand};
#[derive(clap::Args)]
pub struct LogArgs {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Log anime immersion time
    Anime(anime::AnimeArgs),
}

impl LogArgs {
    pub fn run(&self) {
        match &self.command {
            Some(Commands::Anime(cmd))  => cmd.run(),
            _ => {}
        }
    }
}
