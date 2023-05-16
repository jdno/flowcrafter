use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct Configuration {
    pub library: LibraryConfiguration,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct LibraryConfiguration {
    pub repository: RepositoryConfiguration,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Deserialize, Serialize)]
pub struct RepositoryConfiguration {
    pub owner: String,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const CONFIG: &str = indoc! {r#"
        library:
          repository:
            owner: owner
            name: name
    "#};

    #[test]
    fn trait_deserialize() {
        let config: Configuration = serde_yaml::from_str(CONFIG).unwrap();

        assert_eq!(
            LibraryConfiguration {
                repository: RepositoryConfiguration {
                    owner: "owner".to_string(),
                    name: "name".to_string(),
                },
            },
            config.library
        );
    }

    #[test]
    fn trait_serialize() {
        let config = Configuration {
            library: LibraryConfiguration {
                repository: RepositoryConfiguration {
                    owner: "owner".to_string(),
                    name: "name".to_string(),
                },
            },
        };

        let yaml = serde_yaml::to_string(&config).unwrap();

        assert_eq!(CONFIG, yaml);
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Configuration>();
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
