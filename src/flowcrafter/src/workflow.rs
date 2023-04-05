use std::fmt::{Display, Formatter};

use yaml_rust::Yaml;

use crate::{Error, Job};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Workflow {
    name: String,
    template: Yaml,
    jobs: Vec<Job>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct WorkflowBuilder {
    name: Option<String>,
    template: Option<Yaml>,
    jobs: Vec<Job>,
}

impl WorkflowBuilder {
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

    pub fn job(mut self, job: Job) -> Self {
        self.jobs.push(job);
        self
    }

    pub fn build(self) -> Result<Workflow, Error> {
        let name = self.name.ok_or(Error::MissingField("name".into()))?;
        let template = self
            .template
            .ok_or(Error::MissingField("template".into()))?;
        let jobs = self.jobs;

        Ok(Workflow {
            name,
            template,
            jobs,
        })
    }
}

impl Display for Workflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Display for WorkflowBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "WorkflowBuilder")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_requires_name() {
        let builder = WorkflowBuilder::new()
            .template(Yaml::from_str("template"))
            .build()
            .unwrap_err();

        assert_eq!(Error::MissingField("name".into()), builder);
    }

    #[test]
    fn build_requires_template() {
        let builder = WorkflowBuilder::new().name("name").build().unwrap_err();

        assert_eq!(Error::MissingField("template".into()), builder);
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
