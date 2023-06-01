use std::fmt::{Display, Formatter};

use anyhow::{Context, Error};
use async_trait::async_trait;

use crate::cli::{Command, Configuration, LibraryConfiguration, Project};
use crate::github::GitHubLibrary;
use crate::{Error as CrateError, Fragment, FragmentLibrary, Renderer, Workflow};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Create<'a> {
    project: &'a Project,
    workflow: &'a str,
    jobs: &'a [String],
}

impl<'a> Create<'a> {
    pub fn new(project: &'a Project, workflow: &'a str, jobs: &'a [String]) -> Self {
        Self {
            project,
            workflow,
            jobs,
        }
    }

    fn init_library(&self, configuration: &'a Configuration) -> impl FragmentLibrary<'a> {
        match configuration.library() {
            LibraryConfiguration::GitHub(github_configuration) => {
                GitHubLibrary::new(github_configuration)
            }
        }
    }

    async fn get_workflow(&self, library: &impl FragmentLibrary<'a>) -> Result<Fragment, Error> {
        library.workflow(self.workflow).await.context(format!(
            "failed to download workflow '{}' from GitHub",
            self.workflow
        ))
    }

    async fn get_jobs(&self, library: &impl FragmentLibrary<'a>) -> Result<Vec<Fragment>, Error> {
        let mut jobs = Vec::new();

        for job in self.jobs {
            let job = library
                .job(self.workflow, job)
                .await
                .context(format!("failed to download job '{}' from GitHub", job))?;

            jobs.push(job);
        }

        Ok(jobs)
    }

    fn render_workflow(
        &self,
        workflow: &Fragment,
        jobs: &[Fragment],
    ) -> Result<Workflow, CrateError> {
        let renderer = Renderer::new(workflow, jobs);
        renderer.render()
    }

    fn save_workflow(&self, workflow: &Workflow) -> Result<(), Error> {
        let path = self
            .project
            .path()
            .join(".github")
            .join("workflows")
            .join(format!("{}.yml", self.workflow));

        std::fs::write(path, workflow.to_string()).context("failed to write workflow file")
    }
}

#[async_trait]
impl<'a> Command for Create<'a> {
    async fn run(&self) -> Result<(), Error> {
        let configuration = Configuration::load(self.project)?;
        let library = self.init_library(&configuration);

        let workflow = self.get_workflow(&library).await?;
        let jobs = self.get_jobs(&library).await?;

        let rendered_workflow = self.render_workflow(&workflow, &jobs)?;
        self.save_workflow(&rendered_workflow)?;

        Ok(())
    }
}

impl Display for Create<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "create -w {} -j {}",
            self.workflow,
            self.jobs.join(" -j ")
        )
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[tokio::test]
    async fn run_errors_without_configuration() {
        // Create project directory
        let temp_dir = tempdir().unwrap();

        // Create .git directory
        let git_dir = temp_dir.path().join(".git");
        std::fs::create_dir(git_dir).unwrap();

        // Create .github directory
        let git_dir = temp_dir.path().join(".github");
        std::fs::create_dir(git_dir).unwrap();

        let project = Project::at(temp_dir.path().into()).unwrap();

        let jobs = vec!["job1".into(), "job2".into()];
        let command = Create::new(&project, "workflow", &jobs);

        let error = command.run().await.unwrap_err();

        assert_eq!("failed to read configuration file", error.to_string());
    }

    #[test]
    fn trait_display() {
        let project = Project::at(".".into()).unwrap();

        let jobs = vec!["job1".into(), "job2".into()];
        let command = Create::new(&project, "workflow", &jobs);

        assert_eq!("create -w workflow -j job1 -j job2", command.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Create>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Create>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Create>();
    }
}
