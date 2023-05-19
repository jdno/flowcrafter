use std::fmt::{Display, Formatter};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Workflow(String);

impl Workflow {
    pub fn new(workflow: impl Into<String>) -> Self {
        Self(workflow.into())
    }

    pub fn get(&self) -> &str {
        &self.0
    }
}

impl Display for Workflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for Workflow {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "workflow";

    #[test]
    fn get() {
        let workflow = Workflow::new(CONTENT);

        assert_eq!(CONTENT, workflow.get());
    }

    #[test]
    fn trait_display() {
        let workflow = Workflow::new(CONTENT);

        assert_eq!(CONTENT, format!("{}", workflow));
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Workflow>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Workflow>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Workflow>();
    }
}
