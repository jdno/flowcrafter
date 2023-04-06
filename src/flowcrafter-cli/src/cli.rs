use clap::{Parser, Subcommand};

#[derive(Clone, Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Subcommand)]
pub enum Commands {
    Create {
        #[arg(short, long)]
        workflow: String,

        #[arg(short, long)]
        job: Vec<String>,
    },
    Init {
        repository: String,
    },
}
