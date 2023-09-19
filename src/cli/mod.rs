use clap::Parser;

pub use self::{
    commands::*,
    configuration::{Configuration, LibraryConfiguration},
};

mod commands;
mod configuration;

#[derive(Clone, Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
