use std::fmt::Display;

use typed_builder::TypedBuilder;
use url::Url;

use crate::github::owner::Owner;
use crate::github::repository::Repository;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GitHubConfiguration {
    #[cfg_attr(feature = "serde", serde(default = "default_instance"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "is_default_instance"))]
    #[builder(default = default_instance())]
    instance: Url,

    #[builder(setter(into))]
    owner: Owner,

    #[builder(setter(into))]
    repository: Repository,
}

impl GitHubConfiguration {
    pub fn instance(&self) -> &Url {
        &self.instance
    }

    pub fn owner(&self) -> &Owner {
        &self.owner
    }

    pub fn repository(&self) -> &Repository {
        &self.repository
    }
}

impl Display for GitHubConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "repository: {}/{}", self.owner, self.repository)
    }
}

fn default_instance() -> Url {
    Url::parse("https://api.github.com").expect("failed to parse hard-coded GitHub URL 🤯")
}

fn is_default_instance(instance: &Url) -> bool {
    instance == &default_instance()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[cfg(feature = "serde")]
    #[test]
    fn trait_deserialize_with_default_instance() {
        let yaml = indoc!(
            r#"
            ---
            owner: jdno
            repository: flowcrafter
            "#
        );

        let configuration = serde_yaml::from_str::<GitHubConfiguration>(yaml).unwrap();

        assert_eq!(default_instance(), *configuration.instance());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn trait_deserialize_with_custom_instance() {
        let yaml = indoc!(
            r#"
            ---
            instance: https://github.example.com
            owner: jdno
            repository: flowcrafter
            "#
        );

        let configuration = serde_yaml::from_str::<GitHubConfiguration>(yaml).unwrap();

        assert_eq!(
            "https://github.example.com/",
            configuration.instance().to_string()
        );
    }

    #[test]
    fn trait_display() {
        let configuration = GitHubConfiguration::builder()
            .owner(Owner::from("jdno"))
            .repository(Repository::from("flowcrafter"))
            .build();

        assert_eq!("repository: jdno/flowcrafter", configuration.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<GitHubConfiguration>();
    }

    #[cfg(feature = "serde")]
    #[test]
    fn trait_serialize_with_default_instance() {
        let configuration = GitHubConfiguration::builder()
            .owner("jdno")
            .repository("flowcrafter")
            .build();

        let yaml = indoc!(
            r#"
            owner: jdno
            repository: flowcrafter
            "#
        );

        assert_eq!(yaml, serde_yaml::to_string(&configuration).unwrap());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn trait_serialize_with_custom_instance() {
        let configuration = GitHubConfiguration::builder()
            .instance(Url::parse("https://github.example.com").unwrap())
            .owner("jdno")
            .repository("flowcrafter")
            .build();

        let yaml = indoc!(
            r#"
            instance: https://github.example.com/
            owner: jdno
            repository: flowcrafter
            "#
        );

        assert_eq!(yaml, serde_yaml::to_string(&configuration).unwrap());
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<GitHubConfiguration>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<GitHubConfiguration>();
    }
}
