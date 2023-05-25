use anyhow::Context;
use anyhow::Error;
use clap::Parser;

use flowcrafter::cli::{Cli, Commands, Project};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let cwd = std::env::current_dir().context("failed to detect current directory")?;
    let project = Project::find(cwd)?;

    Commands::execute(&cli.command, &project).await
}
