use anyhow::{Context, Result};
use clap::Parser;

use crate::cli::{Cli, Commands};
use crate::commands::{Command, Create, Init};

mod cli;
mod commands;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let cwd = std::env::current_dir().context("failed to detect current directory")?;

    let command: Box<dyn Command> = match &cli.command {
        Commands::Create { workflow, job } => Box::new(Create::new(workflow, job.to_vec())),
        Commands::Init { repository } => Box::new(Init::new(cwd, repository)),
    };

    command.run()
}
