use std::fmt::{Display, Formatter};

use liquid::{object, Object, ParserBuilder};

use crate::error::Error;
use crate::fragment::Fragment;
use crate::workflow::Workflow;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Renderer<'a> {
    workflow: &'a Fragment,
    jobs: &'a [Fragment],
}

impl<'a> Renderer<'a> {
    pub fn new(workflow: &'a Fragment, jobs: &'a [Fragment]) -> Self {
        Self { workflow, jobs }
    }

    pub fn render(&self) -> Result<Workflow, Error> {
        let job_templates = self.render_jobs()?;

        let workflow_template = ParserBuilder::with_stdlib()
            .build()?
            .parse(self.workflow.template().get())?;

        let globals = object!({
            "jobs": job_templates,
        });

        workflow_template
            .render(&globals)
            .map(Workflow::new)
            .map_err(Error::from)
    }

    fn render_jobs(&self) -> Result<Vec<String>, Error> {
        let globals = Object::new();

        let mut jobs = Vec::new();
        for job in self.jobs {
            let job_template = ParserBuilder::with_stdlib()
                .build()?
                .parse(job.template().get())?;

            let job_rendered = job_template.render(&globals)?;

            jobs.push(job_rendered);
        }

        Ok(jobs)
    }
}

impl<'a> Display for Renderer<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Renderer {{ workflow: {}, job_count: {} }}",
            self.workflow,
            self.jobs.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn render() {
        let workflow = Fragment::builder()
            .name("workflow")
            .template(
                indoc!(
                    r#"
                    jobs:
                    {%- for job in jobs %}
                      - {{ job }}
                    {% endfor -%}
                    "#
                )
                .into(),
            )
            .build();

        let jobs = vec![Fragment::builder()
            .name("job")
            .template("job".into())
            .build()];

        let renderer = Renderer::new(&workflow, &jobs);

        let rendered = renderer.render().unwrap();

        assert_eq!(
            indoc!(
                r#"
                jobs:
                  - job
                "#
            ),
            rendered.get()
        );
    }

    #[test]
    fn render_empty_workflow() {
        let workflow = Fragment::builder()
            .name("workflow")
            .template("".into())
            .build();

        let jobs = vec![Fragment::builder()
            .name("job")
            .template("job".into())
            .build()];

        let renderer = Renderer::new(&workflow, &jobs);

        let rendered = renderer.render().unwrap();

        assert_eq!("", rendered.get());
    }

    #[test]
    fn render_without_variable() {
        let workflow = Fragment::builder()
            .name("workflow")
            .template("workflow".into())
            .build();

        let jobs = vec![Fragment::builder()
            .name("job")
            .template("job".into())
            .build()];

        let renderer = Renderer::new(&workflow, &jobs);

        let rendered = renderer.render().unwrap();

        assert_eq!("workflow", rendered.get());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Renderer>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Renderer>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Renderer>();
    }
}
