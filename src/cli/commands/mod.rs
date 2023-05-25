use anyhow::Error;
use async_trait::async_trait;
use clap::Subcommand;

use crate::cli::project::Project;

pub use self::init::Init;

mod init;

#[async_trait]
pub trait Command {
    async fn run(&self) -> Result<(), Error>;
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Subcommand)]
pub enum Commands {
    Init {
        #[arg(short, long)]
        repository: String,
    },
}

impl Commands {
    pub async fn execute(command: &Commands, project: &Project) -> Result<(), Error> {
        match command {
            Commands::Init { repository } => Init::new(project, repository).run().await,
        }
    }
}
