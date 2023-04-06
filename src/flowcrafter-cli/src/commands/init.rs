use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};

use flowcrafter::{Configuration, LibraryConfiguration};

use crate::commands::Command;

pub struct Init {
    cwd: PathBuf,
    repository: String,
}

impl Init {
    pub fn new(cwd: PathBuf, repository: impl Into<String>) -> Self {
        Self {
            cwd,
            repository: repository.into(),
        }
    }

    fn validate_repository(&self) -> Result<()> {
        let parts: Vec<&str> = self.repository.split('/').collect();

        if parts.len() != 2 {
            return Err(anyhow!("repository must have the format owner/name"));
        }

        Ok(())
    }

    fn find_project_directory(&self) -> Result<PathBuf> {
        let mut current_directory = self.cwd.clone();

        loop {
            let git_directory = current_directory.join(".git");

            if git_directory.exists() {
                return Ok(current_directory);
            }

            if !current_directory.pop() {
                return Err(anyhow!("flowcrafter must be run inside a Git repository"));
            }
        }
    }

    fn find_or_create_directory(&self, directory: PathBuf) -> Result<PathBuf> {
        if !directory.exists() {
            std::fs::create_dir_all(directory.clone())?;
        }

        Ok(directory)
    }

    fn find_or_create_config(&self, config_path: PathBuf) -> Result<Configuration> {
        let config = Configuration {
            library: LibraryConfiguration {
                repository: self.repository.clone(),
            },
        };

        let serialized_config =
            serde_yaml::to_string(&config).context("failed to serialize default configuration")?;

        std::fs::write(config_path, serialized_config)?;

        Ok(config)
    }
}

impl Command for Init {
    fn run(&self) -> Result<()> {
        self.validate_repository()?;

        let project_directory = self.find_project_directory()?;
        let github_directory = self.find_or_create_directory(project_directory.join(".github"))?;

        let _config = self.find_or_create_config(github_directory.join("flowcrafter.yml"))?;

        Ok(())
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
    fn run_validates_repository() {
        let temp_dir = temp_dir();
        let init = Init::new(temp_dir.path().to_path_buf(), "owner/name");

        assert!(init.run().is_ok());
    }

    #[test]
    fn run_errors_if_not_owner_name() {
        let temp_dir = temp_dir();
        let init = Init::new(temp_dir.path().to_path_buf(), "repository");

        let error = init.run().unwrap_err();

        assert_eq!(
            "repository must have the format owner/name",
            error.to_string()
        );
    }

    #[test]
    fn run_finds_git_repository() {
        let temp_dir = temp_dir();

        // Create a subdirectory
        let sub_dir = temp_dir.path().join("sub");
        std::fs::create_dir(sub_dir.clone()).unwrap();

        let init = Init::new(sub_dir, "owner/name");

        assert!(init.run().is_ok());
    }

    #[test]
    fn run_errors_if_not_git_repository() {
        let temp_dir = tempdir().unwrap();

        let init = Init::new(temp_dir.path().to_path_buf(), "owner/name");

        let error = init.run().unwrap_err();

        assert_eq!(
            "flowcrafter must be run inside a Git repository",
            error.to_string()
        );
    }

    #[test]
    fn run_finds_github_directory() {
        let temp_dir = temp_dir();

        // Create a .github directory
        let github_dir = temp_dir.path().join(".github").join("workflows");
        std::fs::create_dir_all(github_dir).unwrap();

        let init = Init::new(temp_dir.path().to_path_buf(), "owner/name");

        assert!(init.run().is_ok());
    }

    #[test]
    fn run_creates_github_directory() {
        let temp_dir = temp_dir();

        let init = Init::new(temp_dir.path().to_path_buf(), "owner/name");

        assert!(init.run().is_ok());
        assert!(temp_dir.path().join(".github").exists());
    }

    #[test]
    fn run_writes_flowcrafter_config() {
        let temp_dir = temp_dir();

        let init = Init::new(temp_dir.path().to_path_buf(), "owner/name");

        assert!(init.run().is_ok());

        let config = temp_dir.path().join(".github").join("flowcrafter.yml");
        let contents = std::fs::read_to_string(config).unwrap();

        assert!(contents.contains("owner/name"));
    }
}
