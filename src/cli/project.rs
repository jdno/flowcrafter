use std::path::{Path, PathBuf};

use anyhow::{anyhow, Error};
#[cfg(test)]
use tempfile::{tempdir, TempDir};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Project {
    path: PathBuf,
}

impl Project {
    pub fn at(path: PathBuf) -> Result<Self, Error> {
        let git_directory = path.join(".git");

        if !git_directory.exists() {
            return Err(anyhow!("flowcrafter must be run inside a Git repository"));
        }

        Ok(Self { path })
    }

    pub fn find(path: PathBuf) -> Result<Self, Error> {
        let mut current_directory = path;

        loop {
            match Project::at(current_directory.clone()) {
                Ok(project) => return Ok(project),
                Err(error) => {
                    if !current_directory.pop() {
                        return Err(error);
                    }
                }
            }
        }
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }
}

#[cfg(test)]
#[derive(Debug)]
pub struct TestProject {
    temp_dir: TempDir,
    project: Project,
}

#[cfg(test)]
impl TestProject {
    pub fn new() -> Result<Self, Error> {
        // Create project directory
        let temp_dir = tempdir().unwrap();

        // Create .git directory
        let git_dir = temp_dir.path().join(".git");
        std::fs::create_dir(git_dir).unwrap();

        let project = Project::at(temp_dir.path().to_path_buf())?;

        Ok(Self { temp_dir, project })
    }

    pub fn path(&self) -> &Path {
        self.temp_dir.path()
    }

    pub fn project(&self) -> &Project {
        &self.project
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn at() {
        let test_project = TestProject::new().unwrap();

        let project = Project::at(test_project.path().to_path_buf());

        assert!(project.is_ok());
    }

    #[test]
    fn at_returns_error_outside_git_repository() {
        // Create project directory
        let temp_dir = tempdir().unwrap();

        let error = Project::at(temp_dir.path().to_path_buf()).unwrap_err();

        assert_eq!(
            "flowcrafter must be run inside a Git repository",
            error.to_string()
        );
    }

    #[test]
    fn find() {
        let test_project = TestProject::new().unwrap();

        // Create a subdirectory
        let sub_dir = test_project.path().join("sub");
        std::fs::create_dir(sub_dir.clone()).unwrap();

        let project = Project::find(sub_dir);

        assert!(project.is_ok());
    }

    #[test]
    fn find_returns_error_outside_git_repository() {
        // Create project directory
        let temp_dir = tempdir().unwrap();

        // Create a subdirectory
        let sub_dir = temp_dir.path().join("sub");
        std::fs::create_dir(sub_dir.clone()).unwrap();

        let project = Project::find(sub_dir);

        assert!(project.is_err());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Project>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Project>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Project>();
    }
}
