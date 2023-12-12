use std::fmt::{Display, Formatter};

use anyhow::{Context, Error};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::Project;

pub use self::library::LibraryConfiguration;
pub use self::workflow::WorkflowConfiguration;

mod library;
mod workflow;

const CONFIG_FILE_NAME: &str = "flowcrafter.yml";

#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize, TypedBuilder,
)]
pub struct Configuration {
    #[serde(with = "serde_yaml::with::singleton_map")]
    library: LibraryConfiguration,
    #[serde(default)]
    workflows: Vec<WorkflowConfiguration>,
}

impl Configuration {
    pub fn save(&self, project: &Project) -> Result<(), Error> {
        let github_path = project.path().join(".github");
        if !github_path.exists() {
            std::fs::create_dir_all(github_path.clone())
                .context("failed to create .github directory in project")?;
        }

        let config_path = github_path.join(CONFIG_FILE_NAME);

        let serialized =
            serde_yaml::to_string(self).context("failed to serialize configuration to YAML")?;

        std::fs::write(config_path, serialized).context("failed to write configuration to file")?;

        Ok(())
    }

    pub fn load(project: &Project) -> Result<Self, Error> {
        let config_path = project.path().join(".github").join(CONFIG_FILE_NAME);

        let serialized =
            std::fs::read_to_string(config_path).context("failed to read configuration file")?;

        let config =
            serde_yaml::from_str(&serialized).context("failed to deserialize configuration")?;

        Ok(config)
    }

    pub fn library(&self) -> &LibraryConfiguration {
        &self.library
    }
}

impl Display for Configuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Configuration")
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use tempfile::{tempdir, TempDir};

    use crate::github::GitHubConfiguration;

    use super::*;

    const SERIALIZED_CONFIGURATION: &str = indoc!(
        r#"
        ---
        library:
          github:
            owner: jdno
            repository: flowcrafter
        workflows:
          - name: rust
            jobs:
              - lint
              - style
              - test
        "#
    );

    fn configuration() -> Configuration {
        Configuration::builder()
            .library(LibraryConfiguration::GitHub(
                GitHubConfiguration::builder()
                    .owner("jdno")
                    .repository("flowcrafter")
                    .build(),
            ))
            .workflows(vec![WorkflowConfiguration::builder()
                .name("rust")
                .jobs(vec!["lint".into(), "style".into(), "test".into()])
                .build()])
            .build()
    }

    fn temp_dir() -> TempDir {
        // Create project directory
        let temp_dir = tempdir().unwrap();

        // Create .git directory
        let git_dir = temp_dir.path().join(".git");
        std::fs::create_dir(git_dir).unwrap();

        // Create .github directory
        let github_dir = temp_dir.path().join(".github");
        std::fs::create_dir(github_dir).unwrap();

        temp_dir
    }

    #[test]
    fn save_writes_configuration() {
        let project_directory = temp_dir();
        let project = Project::at(project_directory.path().into()).unwrap();

        let saved_config = configuration();
        saved_config.save(&project).unwrap();

        let loaded_config = Configuration::load(&project).unwrap();
        assert_eq!(saved_config, loaded_config);
    }

    #[test]
    fn save_creates_directory_if_not_exists() {
        // Create project directory
        let temp_dir = tempdir().unwrap();

        // Create .git directory
        let git_dir = temp_dir.path().join(".git");
        std::fs::create_dir(git_dir).unwrap();

        // Create configuration
        let project = Project::at(temp_dir.path().into()).unwrap();
        configuration().save(&project).unwrap();

        assert!(project.path().join(".github").exists());
    }

    #[test]
    fn save_overwrites_existing_configuration() {
        let project_directory = temp_dir();
        let project = Project::at(project_directory.path().into()).unwrap();

        std::fs::write(
            project.path().join(".github").join(CONFIG_FILE_NAME),
            "This is not a valid configuation file in YAML format.",
        )
        .unwrap();

        let saved_config = configuration();
        saved_config.save(&project).unwrap();

        let loaded_config = Configuration::load(&project).unwrap();
        assert_eq!(saved_config, loaded_config);
    }

    #[test]
    fn load_returns_configuration() {
        let project_directory = temp_dir();
        let project = Project::at(project_directory.path().into()).unwrap();

        std::fs::write(
            project.path().join(".github").join(CONFIG_FILE_NAME),
            SERIALIZED_CONFIGURATION,
        )
        .unwrap();

        let loaded_config = Configuration::load(&project).unwrap();

        assert!(matches!(
            loaded_config.library,
            LibraryConfiguration::GitHub(_)
        ));
    }

    #[test]
    fn load_returns_error_if_file_not_found() {
        let project_directory = temp_dir();
        let project = Project::at(project_directory.path().into()).unwrap();

        let error = Configuration::load(&project).unwrap_err();

        assert_eq!("failed to read configuration file", error.to_string());
    }

    #[test]
    fn load_returns_error_if_file_not_yaml() {}

    #[cfg(feature = "serde")]
    #[test]
    fn trait_deserialize() {
        let configuration: Configuration = serde_yaml::from_str(SERIALIZED_CONFIGURATION).unwrap();

        assert!(matches!(
            configuration.library,
            LibraryConfiguration::GitHub(_)
        ));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn trait_deserialize_without_workflows() {
        let yaml = indoc!(
            r#"
            ---
            library:
              github:
                owner: jdno
                repository: flowcrafter
            "#
        );

        let configuration: Configuration = serde_yaml::from_str(yaml).unwrap();

        assert!(matches!(
            configuration.library,
            LibraryConfiguration::GitHub(_)
        ));
    }

    #[test]
    fn trait_display() {
        assert_eq!("Configuration", configuration().to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Configuration>();
    }

    #[test]
    fn trait_serialize() {
        let yaml = serde_yaml::to_string(&configuration()).unwrap();

        let expected = indoc!(
            r#"
            library:
              github:
                owner: jdno
                repository: flowcrafter
            workflows:
            - name: rust
              jobs:
              - lint
              - style
              - test
            "#
        );

        assert_eq!(expected, yaml);
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Configuration>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Configuration>();
    }
}
