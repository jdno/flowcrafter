use anyhow::Error;
use async_trait::async_trait;
use clap::Subcommand;

use crate::Project;

pub use self::create::Create;
pub use self::init::Init;

mod create;
mod init;

#[async_trait]
pub trait Command {
    async fn run(&self) -> Result<(), Error>;
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Subcommand)]
pub enum Commands {
    Create {
        #[arg(short, long)]
        workflow: String,
        #[arg(short, long)]
        jobs: Vec<String>,
    },
    Init {
        #[arg(short, long)]
        repository: String,
    },
}

impl Commands {
    pub async fn execute(command: &Commands, project: &Project) -> Result<(), Error> {
        match command {
            Commands::Create { workflow, jobs } => Create::new(project, workflow, jobs).run().await,
            Commands::Init { repository } => Init::new(project, repository).run().await,
        }
    }
}
