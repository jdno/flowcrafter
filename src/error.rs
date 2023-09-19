use thiserror::Error;

use crate::fragment::FragmentError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to parse configuration: {0}")]
    Configuration(&'static str),

    #[error("{0}")]
    Fragment(#[from] FragmentError),

    #[error("{0}")]
    GitHub(#[from] octocrab::Error),

    #[error("{0}")]
    InvalidTemplate(String),

    #[error("{0}")]
    IO(#[from] std::io::Error),

    #[error("failed to find '{0}' in {1}")]
    NotFound(String, String),

    #[error("failed to render workflow: {0}")]
    Render(String),

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Error>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Error>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Error>();
    }
}
