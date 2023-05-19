use url::Url;

use crate::error::Error;
use crate::github::{GitHubConfiguration, Owner, Repository};

const REPO_PARSE_ERROR: &str = "repository must be provided in the format 'owner/repository'";

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct GitHubConfigurationBuilder {
    instance: Url,
    owner: Option<Owner>,
    repository: Option<Repository>,
}

impl GitHubConfigurationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn full_name(mut self, full_name: &str) -> Result<Self, Error> {
        let mut parts = full_name.split('/');

        let owner = parts.next().ok_or(Error::Configuration(REPO_PARSE_ERROR))?;
        let repository = parts.next().ok_or(Error::Configuration(REPO_PARSE_ERROR))?;

        self.owner = Some(Owner::from(owner));
        self.repository = Some(Repository::from(repository));

        Ok(self)
    }

    pub fn instance(mut self, instance: Url) -> Self {
        self.instance = instance;
        self
    }

    pub fn owner(mut self, owner: Owner) -> Self {
        self.owner = Some(owner);
        self
    }

    pub fn repository(mut self, repository: Repository) -> Self {
        self.repository = Some(repository);
        self
    }

    pub fn build(self) -> Result<GitHubConfiguration, Error> {
        let owner = self
            .owner
            .ok_or(Error::Configuration("missing field 'owner'"))?;

        let repository = self
            .repository
            .ok_or(Error::Configuration("missing field 'repository'"))?;

        Ok(GitHubConfiguration {
            instance: self.instance,
            owner,
            repository,
        })
    }
}

impl Default for GitHubConfigurationBuilder {
    fn default() -> Self {
        let default_url =
            Url::parse("https://api.github.com").expect("failed to parse hard-coded GitHub URL 🤯");

        Self {
            instance: default_url,
            owner: None,
            repository: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_name() {
        let mut builder = GitHubConfigurationBuilder::default();

        builder = builder.full_name("jdno/flowcrafter").unwrap();

        assert_eq!(Some(Owner::from("jdno")), builder.owner);
        assert_eq!(Some(Repository::from("flowcrafter")), builder.repository);
    }

    #[test]
    fn full_name_with_invalid_format() {
        let builder = GitHubConfigurationBuilder::default();

        let result = builder.full_name("jdno").unwrap_err();

        assert!(result.to_string().contains(REPO_PARSE_ERROR));
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<GitHubConfigurationBuilder>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<GitHubConfigurationBuilder>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<GitHubConfigurationBuilder>();
    }
}
