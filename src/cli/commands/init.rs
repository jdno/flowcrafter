use std::fmt::{Display, Formatter};

use anyhow::{Context, Error};
use async_trait::async_trait;

use crate::cli::{Command, Configuration, LibraryConfiguration};
use crate::github::{GitHubConfiguration, Owner, Repository};
use crate::Project;

const REPO_PARSE_ERROR: &str = "repository must be provided in the format 'owner/repository'";

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Init<'a> {
    project: &'a Project,
    repository: &'a str,
}

impl<'a> Init<'a> {
    pub fn new(project: &'a Project, repository: &'a str) -> Self {
        Self {
            project,
            repository,
        }
    }

    fn parse_repository(self) -> Result<(Owner, Repository), Error> {
        let mut parts = self.repository.split('/');

        let owner = parts.next().context(REPO_PARSE_ERROR)?.into();
        let repository = parts.next().context(REPO_PARSE_ERROR)?.into();

        Ok((owner, repository))
    }

    fn create_config(&self, owner: Owner, repository: Repository) -> Result<Configuration, Error> {
        let config = Configuration::builder()
            .library(LibraryConfiguration::GitHub(
                GitHubConfiguration::builder()
                    .owner(owner)
                    .repository(repository)
                    .build(),
            ))
            .build();

        config.save(self.project)?;

        Ok(config)
    }
}

#[async_trait]
impl Command for Init<'_> {
    async fn run(&self) -> Result<(), Error> {
        let (owner, repository) = self.parse_repository()?;

        let _config = self.create_config(owner, repository)?;

        Ok(())
    }
}

impl Display for Init<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "init -r {}", self.repository)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Project, TestProject};

    use super::*;

    #[test]
    fn parse_repository() {
        let project = Project::at(".".into()).unwrap();
        let command = Init::new(&project, "jdno/flowcrafter");

        let (owner, repository) = command.parse_repository().unwrap();

        assert_eq!(Owner::from("jdno"), owner);
        assert_eq!(Repository::from("flowcrafter"), repository);
    }

    #[test]
    fn parse_repository_with_invalid_format() {
        let project = Project::at(".".into()).unwrap();
        let command = Init::new(&project, "flowcrafter");

        let result = command.parse_repository().unwrap_err();

        assert!(result.to_string().contains(REPO_PARSE_ERROR));
    }

    #[tokio::test]
    async fn run_parses_repository_input() {
        let test_project = TestProject::new().unwrap();
        let init = Init::new(test_project.project(), "jdno/flowcrafter");

        assert!(init.run().await.is_ok());
    }

    #[tokio::test]
    async fn run_errors_if_repository_not_owner_name() {
        let test_project = TestProject::new().unwrap();
        let init = Init::new(test_project.project(), "flowcrafter");

        let error = init.run().await.unwrap_err();

        assert_eq!(REPO_PARSE_ERROR, error.to_string());
    }

    #[tokio::test]
    async fn run_finds_git_repository() {
        let test_project = TestProject::new().unwrap();

        // Create a subdirectory
        let sub_dir = test_project.path().join("sub");
        std::fs::create_dir(sub_dir.clone()).unwrap();

        let project = Project::find(sub_dir).unwrap();
        let init = Init::new(&project, "jdno/flowcrafter");

        assert!(init.run().await.is_ok());
    }

    #[tokio::test]
    async fn run_finds_github_directory() {
        let test_project = TestProject::new().unwrap();

        // Create a .github directory
        let github_dir = test_project.path().join(".github").join("workflows");
        std::fs::create_dir_all(github_dir).unwrap();

        let project = Project::at(test_project.path().to_path_buf()).unwrap();
        let init = Init::new(&project, "jdno/flowcrafter");

        assert!(init.run().await.is_ok());
    }

    #[tokio::test]
    async fn run_creates_github_directory() {
        let test_project = TestProject::new().unwrap();
        let init = Init::new(test_project.project(), "jdno/flowcrafter");

        assert!(init.run().await.is_ok());
        assert!(test_project.path().join(".github").exists());
    }

    #[tokio::test]
    async fn run_writes_flowcrafter_config() {
        let test_project = TestProject::new().unwrap();
        let init = Init::new(test_project.project(), "jdno/flowcrafter");

        assert!(init.run().await.is_ok());

        let config = test_project.path().join(".github").join("flowcrafter.yml");
        let contents = std::fs::read_to_string(config).unwrap();

        assert!(contents.contains("owner: jdno"));
        assert!(contents.contains("repository: flowcrafter"));
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Init>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Init>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Init>();
    }
}
