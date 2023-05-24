use anyhow::Context;
use clap::Parser;

use flowcrafter::cli::{Cli, Commands};
use flowcrafter::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    let cwd = std::env::current_dir().context("failed to detect current directory")?;

    Commands::execute(&cli.command, &cwd).await
}
