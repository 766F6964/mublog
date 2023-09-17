use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "mublog")]
#[command(about = "A minimal static blog site generator\n", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Creates a new page or post
    #[command(arg_required_else_help = true)]
    New(NewArgs),
    /// Initializes a new blog environment
    #[command(arg_required_else_help = true)]
    Init(InitArgs),
    /// Shows information about the current blog
    Info,
    /// Builds the blog into a static blog site
    Build,
}

#[derive(Debug, Subcommand)]
pub enum NewCommands {
    /// Creates a new blog post
    Post,
    /// Creates a new blog page
    Page,
}

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct NewArgs {
    #[command(subcommand)]
    pub command: NewCommands,
}

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct InitArgs {
    /// The name of the directory for the new blog environment
    #[arg(required = true)]
    pub dir_name: String,
}

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct BuildArgs {}

#[derive(Debug, Subcommand)]
pub enum NewPostCommand {}

#[derive(Debug, Subcommand)]
pub enum NewPageCommand {}
