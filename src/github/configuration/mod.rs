use std::fmt::Display;

use url::Url;

use crate::github::owner::Owner;
use crate::github::repository::Repository;

pub use self::builder::GitHubConfigurationBuilder;

mod builder;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct GitHubConfiguration {
    instance: Url,
    owner: Owner,
    repository: Repository,
}

impl GitHubConfiguration {
    pub fn build() -> GitHubConfigurationBuilder {
        GitHubConfigurationBuilder::new()
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_display() {
        let configuration = GitHubConfiguration::build()
            .owner(Owner::from("jdno"))
            .repository(Repository::from("flowcrafter"))
            .build()
            .unwrap();

        assert_eq!("repository: jdno/flowcrafter", configuration.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<GitHubConfiguration>();
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
