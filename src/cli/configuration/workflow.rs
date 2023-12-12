use typed_builder::TypedBuilder;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorkflowConfiguration {
    #[builder(setter(into))]
    name: String,
    #[builder(setter(into))]
    jobs: Vec<String>,
}

impl WorkflowConfiguration {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn jobs(&self) -> &[String] {
        &self.jobs
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[cfg(feature = "serde")]
    #[test]
    fn trait_deserialize_with_empty_jobs() {
        let yaml = indoc!(
            r#"
            ---
            name: test
            jobs: []
            "#
        );

        let config: WorkflowConfiguration =
            serde_yaml::from_str(yaml).expect("failed to deserialize YAML");

        assert_eq!(config.name(), "test");
        assert!(config.jobs().is_empty());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn trait_deserialize_with_jobs() {
        let yaml = indoc!(
            r#"
            ---
            name: test
            jobs:
              - lint
              - style
            "#
        );

        let config: WorkflowConfiguration =
            serde_yaml::from_str(yaml).expect("failed to deserialize YAML");

        assert_eq!(config.name(), "test");
        assert_eq!(config.jobs(), &["lint", "style"]);
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<WorkflowConfiguration>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<WorkflowConfiguration>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<WorkflowConfiguration>();
    }
}
