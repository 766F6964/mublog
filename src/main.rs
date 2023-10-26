#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use crate::cli::Cli;
use crate::cli::Commands;
use crate::cli::NewCommands;
use anyhow::Context;
use clap::Parser;

use std::env;

mod blog;
mod blog_registry;
mod cli;
mod config;
mod embedded_resources;
mod features;
mod input;
mod page;
mod path_config;
mod pipeline;
mod post;
mod stages;
mod stylesheet;
mod utils;

fn main() -> anyhow::Result<()> {
    let cli_args = Cli::parse();

    let working_dir = env::current_dir().context("Failed to obtain current working directory")?;

    match cli_args.command {
        Commands::Deploy => todo!("Deploying blog to specified remote ..."),
        Commands::Build => blog::build(working_dir)?,
        Commands::Init(init_args) => blog::init(working_dir, &init_args.dir_name)
            .context("Failed to initialize new blog environment")?,
        Commands::Info => {
            blog::info(working_dir).context("Failed to load blog information")?;
        }
        Commands::New(new_args) => match new_args.command {
            NewCommands::Post {} => blog::create_post(working_dir)?,
            NewCommands::Page {} => blog::create_page(working_dir)?,
        },
    }

    Ok(())
}
