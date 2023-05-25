use clap::Parser;

pub use self::{
    commands::*,
    configuration::{Configuration, LibraryConfiguration},
    project::Project,
};

mod commands;
mod configuration;
mod project;

#[derive(Clone, Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
