use anyhow::Result;

use crate::commands::Command;

pub struct Create {
    workflow: String,
    jobs: Vec<String>,
}

impl Create {
    pub fn new(workflow: impl Into<String>, jobs: Vec<String>) -> Self {
        Self {
            workflow: workflow.into(),
            jobs,
        }
    }
}

impl Command for Create {
    fn run(&self) -> Result<()> {
        println!("Create workflow: {}", self.workflow);
        println!("Create jobs: {:?}", self.jobs);

        Ok(())
    }
}
