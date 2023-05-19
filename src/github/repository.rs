use std::fmt::{Display, Formatter};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Repository(String);

impl Repository {
    pub fn new(repository: impl Into<String>) -> Self {
        Self(repository.into())
    }

    pub fn get(&self) -> &str {
        &self.0
    }
}

impl Display for Repository {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Repository {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for Repository {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get() {
        let repository = Repository::new("flowcrafter");

        assert_eq!("flowcrafter", repository.get());
    }

    #[test]
    fn trait_display() {
        let repository = Repository::new("flowcrafter");

        assert_eq!("flowcrafter", format!("{}", repository));
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Repository>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Repository>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Repository>();
    }
}
