#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use crate::cli::Cli;
use crate::cli::Commands;
use crate::cli::NewCommands;
use anyhow::Context;
use clap::Parser;
use std::env;

mod blog;
mod cli;
mod embedded_resources;
mod post;
mod utils;

fn main() -> anyhow::Result<()> {
    let cli_args = Cli::parse();
    let working_dir = env::current_dir().context("Failed to obtain current working directory.")?;

    // TODO: Implement a deploy command, that pushes to a remote specified in the config file.
    match cli_args.command {
        Commands::Build => println!("Starting build process ..."),
        Commands::Init(init_args) => blog::init(working_dir.as_path(), &init_args.dir_name)
            .context("Failed to initialize new blog environment.")?,
        Commands::Info => {
            blog::info(working_dir.as_path()).context("Failed to load blog information")?;
        }
        Commands::New(new_args) => match new_args.command {
            NewCommands::Post {} => blog::create_post(&working_dir)?,
            NewCommands::Page {} => println!("Creating new page ..."),
        },
    }

    Ok(())
}
