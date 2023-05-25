use anyhow::Context;
use clap::Parser;

use flowcrafter::cli::{Cli, Commands, Project};
use flowcrafter::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let cwd = std::env::current_dir().context("failed to detect current directory")?;
    let project = Project::find(cwd)?;

    Commands::execute(&cli.command, &project).await
}
