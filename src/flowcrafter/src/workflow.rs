use std::fmt::{Display, Formatter};

use anyhow::Context;

use crate::{Error, Job, Template};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Workflow {
    name: String,
    template: Template,
    jobs: Vec<Job>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct WorkflowBuilder {
    name: Option<String>,
    template: Option<Template>,
    jobs: Vec<Job>,
}

impl Workflow {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn template(&self) -> &Template {
        &self.template
    }

    pub fn jobs(&self) -> &[Job] {
        &self.jobs
    }
}

impl WorkflowBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn template(mut self, template: Template) -> Self {
        self.template = Some(template);
        self
    }

    pub fn job(mut self, job: Job) -> Self {
        self.jobs.push(job);
        self
    }

    pub fn build(self) -> Result<Workflow, Error> {
        let name = self.name.context("missing field 'name'")?;
        let template = self.template.context("missing field 'template'")?;
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
        let error = WorkflowBuilder::new()
            .template(Template::new("template"))
            .build()
            .unwrap_err();

        assert_eq!("missing field 'name'", error.to_string());
    }

    #[test]
    fn build_requires_template() {
        let error = WorkflowBuilder::new().name("name").build().unwrap_err();

        assert_eq!("missing field 'template'", error.to_string());
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
