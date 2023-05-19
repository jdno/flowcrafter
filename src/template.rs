use std::fmt::{Display, Formatter};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Template(String);

impl Template {
    pub fn new(template: impl Into<String>) -> Self {
        Self(template.into())
    }

    pub fn get(&self) -> &str {
        &self.0
    }
}

impl Display for Template {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Template {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for Template {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get() {
        let template = Template::new("{{foo}}");

        assert_eq!("{{foo}}", template.get());
    }

    #[test]
    fn trait_display() {
        let template = Template::new("{{foo}}");

        assert_eq!("{{foo}}", format!("{}", template));
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Template>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Template>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Template>();
    }
}
