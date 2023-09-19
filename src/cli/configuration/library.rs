use serde::{Deserialize, Serialize};

use crate::github::GitHubConfiguration;
use crate::local::LocalConfiguration;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LibraryConfiguration {
    GitHub(GitHubConfiguration),
    Local(LocalConfiguration),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<LibraryConfiguration>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<LibraryConfiguration>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<LibraryConfiguration>();
    }
}
