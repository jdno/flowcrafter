use anyhow::Context;
use std::fmt::Display;

use yaml_rust::Yaml;

use crate::Error;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Job {
    name: String,
    template: Yaml,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct JobBuilder {
    name: Option<String>,
    template: Option<Yaml>,
}

impl JobBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn template(mut self, template: Yaml) -> Self {
        self.template = Some(template);
        self
    }

    pub fn build(self) -> Result<Job, Error> {
        let name = self.name.context("missing field 'name'")?;
        let template = self.template.context("missing field 'template'")?;

        Ok(Job { name, template })
    }
}

impl Display for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Display for JobBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JobBuilder")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_requires_name() {
        let error = JobBuilder::new()
            .template(Yaml::from_str("template"))
            .build()
            .unwrap_err();

        assert_eq!("missing field 'name'", error.to_string());
    }

    #[test]
    fn build_requires_template() {
        let error = JobBuilder::new().name("name").build().unwrap_err();

        assert_eq!("missing field 'template'", error.to_string());
    }

    #[test]
    fn trait_display() {
        let job = JobBuilder::new()
            .name("name")
            .template(Yaml::from_str("template"))
            .build()
            .unwrap();

        assert_eq!("name", job.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Job>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Job>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Job>();
    }
}
