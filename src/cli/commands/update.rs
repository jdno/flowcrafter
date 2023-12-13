use anyhow::Error;
use async_trait::async_trait;

use crate::cli::{Command, Configuration, Create};
use crate::Project;

pub struct Update<'a> {
    project: &'a Project,
}

impl<'a> Update<'a> {
    pub fn new(project: &'a Project) -> Self {
        Self { project }
    }
}

#[async_trait]
impl<'a> Command for Update<'a> {
    async fn run(&self) -> Result<(), Error> {
        let configuration = Configuration::load(self.project)?;

        for workflow in configuration.workflows() {
            let command = Create::new(self.project, workflow.name(), workflow.jobs());
            command.run().await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Update>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Update>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Update>();
    }
}
