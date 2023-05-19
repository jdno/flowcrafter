use std::fmt::{Display, Formatter};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Owner(String);

impl Owner {
    pub fn new(owner: impl Into<String>) -> Self {
        Self(owner.into())
    }

    pub fn get(&self) -> &str {
        &self.0
    }
}

impl Display for Owner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Owner {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for Owner {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get() {
        let owner = Owner::new("jdno");

        assert_eq!("jdno", owner.get());
    }

    #[test]
    fn trait_display() {
        let owner = Owner::new("jdno");

        assert_eq!("jdno", format!("{}", owner));
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Owner>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Owner>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Owner>();
    }
}
