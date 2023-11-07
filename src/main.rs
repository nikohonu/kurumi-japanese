extern crate chrono;
extern crate skim;

use clap::{Parser, Subcommand};
mod anime;
mod data;
mod log;
mod recommend;

#[derive(Parser)]
#[command(author("Niko Honu"), version("0.1"), about("A small cli program for tracking language learning process."), long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Log immersion time
    Log(log::LogArgs),
    /// Get a recommendation for today's immersion
    Recommend(recommend::RecommendArgs),
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Recommend(cmd)) => cmd.run(),
        Some(Commands::Log(cmd)) => cmd.run(),
        _ => {}
    };
}
