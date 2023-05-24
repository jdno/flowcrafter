use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("{0}")]
    CwdNotGitRepository(&'static str),

    #[error("{0}")]
    InvalidInput(&'static str),

    #[error("failed to access the local file system: '{0}'")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Serialization(#[from] serde_yaml::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<CliError>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<CliError>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<CliError>();
    }
}
