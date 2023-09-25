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
mod page;
mod post;
mod utils;
// TODO: Add blog config options such as enabled plugins, author name, copyright year etc

fn main() -> anyhow::Result<()> {
    let cli_args = Cli::parse();
    let working_dir = env::current_dir().context("Failed to obtain current working directory.")?;

    match cli_args.command {
        Commands::Deploy => todo!("Deploying blog to specified remote ..."),
        Commands::Build => blog::build(working_dir.as_path())?,
        Commands::Init(init_args) => blog::init(working_dir.as_path(), &init_args.dir_name)
            .context("Failed to initialize new blog environment.")?,
        Commands::Info => {
            blog::info(working_dir.as_path()).context("Failed to load blog information")?;
        }
        Commands::New(new_args) => match new_args.command {
            NewCommands::Post {} => blog::create(&working_dir)?,
            NewCommands::Page {} => todo!("Creating a new blog page"),
        },
    }

    Ok(())
}
