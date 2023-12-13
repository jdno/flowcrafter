use std::fmt::{Display, Formatter};
use std::ops::Deref;

use anyhow::{Context, Error};
use async_trait::async_trait;

use crate::cli::configuration::WorkflowConfiguration;
use crate::cli::{Command, Configuration, LibraryConfiguration};
use crate::github::GitHubLibrary;
use crate::local::LocalLibrary;
use crate::{Error as CrateError, Fragment, FragmentLibrary, Project, Renderer, Workflow};

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

    async fn download_fragments(
        &self,
        configuration: &Configuration,
    ) -> Result<(Fragment, Vec<Fragment>), Error> {
        let library = self.init_library(configuration);

        let workflow = self.get_workflow(library.deref()).await?;
        let jobs = self.get_jobs(library.deref()).await?;

        Ok((workflow, jobs))
    }

    fn init_library(&self, configuration: &'a Configuration) -> Box<dyn FragmentLibrary<'a>> {
        match configuration.library() {
            LibraryConfiguration::GitHub(github_configuration) => {
                Box::new(GitHubLibrary::new(github_configuration.clone()))
            }
            LibraryConfiguration::Local(local_configuration) => {
                Box::new(LocalLibrary::new(self.project, local_configuration))
            }
        }
    }

    async fn get_workflow(&self, library: &dyn FragmentLibrary<'a>) -> Result<Fragment, Error> {
        library.workflow(self.workflow).await.context(format!(
            "failed to download workflow '{}' from GitHub",
            self.workflow
        ))
    }

    async fn get_jobs(&self, library: &dyn FragmentLibrary<'a>) -> Result<Vec<Fragment>, Error> {
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

    fn update_configuration(&self, configuration: &mut Configuration) -> Result<(), Error> {
        let workflow = WorkflowConfiguration::builder()
            .name(self.workflow)
            .jobs(self.jobs.to_vec())
            .build();

        configuration.add_workflow(workflow);
        configuration.save(self.project)
    }
}

#[async_trait]
impl<'a> Command for Create<'a> {
    async fn run(&self) -> Result<(), Error> {
        let mut configuration = Configuration::load(self.project)?;

        let (workflow, jobs) = self.download_fragments(&configuration).await?;

        let rendered_workflow = self.render_workflow(&workflow, &jobs)?;
        self.save_workflow(&rendered_workflow)?;

        self.update_configuration(&mut configuration)?;

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
