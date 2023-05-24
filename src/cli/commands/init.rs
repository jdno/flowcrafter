use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

use async_trait::async_trait;

use crate::cli::{CliError, Command, Configuration, LibraryConfiguration};
use crate::github::{GitHubConfiguration, Owner, Repository};
use crate::Error;

const REPO_PARSE_ERROR: &str = "repository must be provided in the format 'owner/repository'";

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Init<'a> {
    repository: &'a str,
    cwd: &'a Path,
}

impl<'a> Init<'a> {
    pub fn new(repository: &'a str, cwd: &'a Path) -> Self {
        Self { repository, cwd }
    }

    fn parse_repository(self) -> Result<(Owner, Repository), CliError> {
        let mut parts = self.repository.split('/');

        let owner = parts
            .next()
            .ok_or(CliError::InvalidInput(REPO_PARSE_ERROR))?
            .into();
        let repository = parts
            .next()
            .ok_or(CliError::InvalidInput(REPO_PARSE_ERROR))?
            .into();

        Ok((owner, repository))
    }

    fn find_project_directory(&self) -> Result<PathBuf, CliError> {
        let mut current_directory = self.cwd.to_path_buf();

        loop {
            let git_directory = current_directory.join(".git");

            if git_directory.exists() {
                return Ok(current_directory);
            }

            if !current_directory.pop() {
                return Err(CliError::CwdNotGitRepository(
                    "flowcrafter must be run inside a Git repository",
                ));
            }
        }
    }

    fn find_or_create_directory(&self, directory: PathBuf) -> Result<PathBuf, CliError> {
        if !directory.exists() {
            std::fs::create_dir_all(directory.clone())?;
        }

        Ok(directory)
    }

    fn find_or_create_config(
        &self,
        config_path: PathBuf,
        owner: Owner,
        repository: Repository,
    ) -> Result<Configuration, CliError> {
        let config = Configuration::builder()
            .library(LibraryConfiguration::GitHub(
                GitHubConfiguration::builder()
                    .owner(owner)
                    .repository(repository)
                    .build(),
            ))
            .build();

        let serialized_config = serde_yaml::to_string(&config)?;
        std::fs::write(config_path, serialized_config)?;

        Ok(config)
    }
}

#[async_trait]
impl Command for Init<'_> {
    async fn run(&self) -> Result<(), Error> {
        let (owner, repository) = self.parse_repository()?;

        let project_directory = self.find_project_directory()?;
        let github_directory = self.find_or_create_directory(project_directory.join(".github"))?;
        let config_path = github_directory.join("flowcrafter.yml");

        let _config = self.find_or_create_config(config_path, owner, repository)?;

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
    use tempfile::{tempdir, TempDir};

    use super::*;

    fn temp_dir() -> TempDir {
        // Create project directory
        let temp_dir = tempdir().unwrap();

        // Create .git directory
        let git_dir = temp_dir.path().join(".git");
        std::fs::create_dir(git_dir).unwrap();

        temp_dir
    }

    #[test]
    fn parse_repository() {
        let command = Init::new("jdno/flowcrafter", Path::new("."));

        let (owner, repository) = command.parse_repository().unwrap();

        assert_eq!(Owner::from("jdno"), owner);
        assert_eq!(Repository::from("flowcrafter"), repository);
    }

    #[test]
    fn parse_repository_with_invalid_format() {
        let command = Init::new("flowcrafter", Path::new("."));

        let result = command.parse_repository().unwrap_err();

        assert!(result.to_string().contains(REPO_PARSE_ERROR));
    }

    #[tokio::test]
    async fn run_parses_repository_input() {
        let temp_dir = temp_dir();
        let init = Init::new("jdno/flowcrafter", temp_dir.path());

        assert!(init.run().await.is_ok());
    }

    #[tokio::test]
    async fn run_errors_if_repository_not_owner_name() {
        let temp_dir = temp_dir();
        let init = Init::new("flowcrafter", temp_dir.path());

        let error = init.run().await.unwrap_err();

        assert_eq!(REPO_PARSE_ERROR, error.to_string());
    }

    #[tokio::test]
    async fn run_finds_git_repository() {
        let temp_dir = temp_dir();

        // Create a subdirectory
        let sub_dir = temp_dir.path().join("sub");
        std::fs::create_dir(sub_dir.clone()).unwrap();

        let init = Init::new("jdno/flowcrafter", sub_dir.as_path());

        assert!(init.run().await.is_ok());
    }

    #[tokio::test]
    async fn run_errors_if_not_git_repository() {
        let temp_dir = tempdir().unwrap();

        let init = Init::new("jdno/flowcrafter", temp_dir.path());

        let error = init.run().await.unwrap_err();

        assert_eq!(
            "flowcrafter must be run inside a Git repository",
            error.to_string()
        );
    }

    #[tokio::test]
    async fn run_finds_github_directory() {
        let temp_dir = temp_dir();

        // Create a .github directory
        let github_dir = temp_dir.path().join(".github").join("workflows");
        std::fs::create_dir_all(github_dir).unwrap();

        let init = Init::new("jdno/flowcrafter", temp_dir.path());

        assert!(init.run().await.is_ok());
    }

    #[tokio::test]
    async fn run_creates_github_directory() {
        let temp_dir = temp_dir();

        let init = Init::new("jdno/flowcrafter", temp_dir.path());

        assert!(init.run().await.is_ok());
        assert!(temp_dir.path().join(".github").exists());
    }

    #[tokio::test]
    async fn run_writes_flowcrafter_config() {
        let temp_dir = temp_dir();

        let init = Init::new("jdno/flowcrafter", temp_dir.path());

        assert!(init.run().await.is_ok());

        let config = temp_dir.path().join(".github").join("flowcrafter.yml");
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
