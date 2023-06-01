use std::fmt::{Display, Formatter};

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
        let mut rendered = Vec::new();

        rendered.push(self.workflow.template().get().into());

        if !self.workflow.template().get().contains("jobs:") {
            rendered.push("jobs:".into());
        }

        let rendered_jobs = self
            .jobs
            .iter()
            .map(|job| self.indent(job.template().get()))
            .collect::<Vec<String>>()
            .join("\n");
        rendered.push(rendered_jobs);

        let rendered = rendered.join("\n");

        Ok(Workflow::new(rendered))
    }

    fn indent(&self, content: &str) -> String {
        let mut indented = String::new();

        for line in content.lines() {
            if line.is_empty() {
                indented.push('\n');
            } else {
                indented.push_str(&format!("  {}\n", line));
            }
        }

        indented
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

    fn fragment(template: &str) -> Fragment {
        Fragment::builder()
            .name("test")
            .template(template.into())
            .build()
    }

    #[test]
    fn render_without_jobs_section() {
        let workflow = fragment(indoc!(
            r#"
            ---
            name: Workflow
            "#
        ));

        let jobs = vec![
            fragment(indoc!(
                r#"
                first:
                  name: First

                  runs-on: ubuntu-latest
                "#
            )),
            fragment(indoc!(
                r#"
                second:
                  name: Second
                "#
            )),
        ];

        let rendered = Renderer::new(&workflow, &jobs).render().unwrap();

        assert_eq!(
            indoc!(
                r#"
                ---
                name: Workflow

                jobs:
                  first:
                    name: First

                    runs-on: ubuntu-latest

                  second:
                    name: Second
                "#
            ),
            rendered.get()
        );
    }

    #[test]
    fn render_with_jobs_section() {
        let workflow = fragment(indoc!(
            r#"
            ---
            name: Workflow

            jobs:
              first:
                name: First
            "#
        ));

        let jobs = vec![fragment(indoc!(
            r#"
            second:
              name: Second
            "#
        ))];

        let rendered = Renderer::new(&workflow, &jobs).render().unwrap();

        assert_eq!(
            indoc!(
                r#"
                ---
                name: Workflow

                jobs:
                  first:
                    name: First

                  second:
                    name: Second
                "#
            ),
            rendered.get()
        );
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
