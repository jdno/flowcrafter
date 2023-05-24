use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

pub use self::library::LibraryConfiguration;

mod library;

#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize, TypedBuilder,
)]
pub struct Configuration {
    #[serde(with = "serde_yaml::with::singleton_map")]
    library: LibraryConfiguration,
}

impl Configuration {
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

    use crate::github::GitHubConfiguration;

    use super::*;

    fn configuration() -> Configuration {
        Configuration::builder()
            .library(LibraryConfiguration::GitHub(
                GitHubConfiguration::builder()
                    .owner("jdno")
                    .repository("flowcrafter")
                    .build(),
            ))
            .build()
    }

    #[cfg(feature = "serde")]
    #[test]
    fn trait_deserialize() {
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
                    instance: https://api.github.com/
                    owner: jdno
                    repository: flowcrafter
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
