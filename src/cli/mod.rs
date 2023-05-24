use clap::Parser;

pub use self::{
    commands::*,
    configuration::{Configuration, LibraryConfiguration},
    error::CliError,
};

mod commands;
mod configuration;
mod error;
mod project;

#[derive(Clone, Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
