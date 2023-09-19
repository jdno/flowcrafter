use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use async_trait::async_trait;

use crate::local::LocalConfiguration;
use crate::{Error, Fragment, FragmentLibrary, Project, Template};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct LocalLibrary {
    path: PathBuf,
}

impl LocalLibrary {
    pub fn new(project: &Project, config: &LocalConfiguration) -> Self {
        let path = project.path().join(config.path());

        Self { path }
    }

    fn read_template(&self, path: &PathBuf) -> Result<Template, Error> {
        if !path.exists() {
            return Err(Error::NotFound(
                path.file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    .to_string(),
                self.to_string(),
            ));
        }

        Ok(std::fs::read_to_string(path)?.into())
    }
}

impl Display for LocalLibrary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "path {}", self.path.display())
    }
}

#[async_trait]
impl<'a> FragmentLibrary<'a> for LocalLibrary {
    async fn workflow(&self, name: &'a str) -> Result<Fragment, Error> {
        let path = self.path.join(name).join("workflow.yml");
        let template = self.read_template(&path)?;

        Ok(Fragment::builder().name(name).template(template).build())
    }

    async fn job(&self, workflow: &'a str, name: &'a str) -> Result<Fragment, Error> {
        let path = self.path.join(workflow).join(format!("{name}.yml"));
        let template = self.read_template(&path)?;

        Ok(Fragment::builder().name(name).template(template).build())
    }
}

#[cfg(test)]
mod tests {
    use std::fs::create_dir;

    use crate::TestProject;

    use super::*;

    #[test]
    fn new_with_absolute_path() {
        let test_project = TestProject::new().unwrap();

        let library = LocalLibrary::new(
            test_project.project(),
            &LocalConfiguration::builder()
                .path("/owner/repo/path")
                .build(),
        );

        let path = PathBuf::from("/owner/repo/path");

        assert_eq!(path, library.path);
    }

    #[test]
    fn new_with_relative_path() {
        let test_project = TestProject::new().unwrap();

        let library = LocalLibrary::new(
            test_project.project(),
            &LocalConfiguration::builder()
                .path("owner/repo/path")
                .build(),
        );

        let path = test_project.path().join("owner/repo/path");

        assert_eq!(path, library.path);
    }

    #[tokio::test]
    async fn workflow() {
        let test_project = TestProject::new().unwrap();

        create_dir(test_project.path().join("workflow")).unwrap();
        std::fs::write(
            test_project.path().join("workflow/workflow.yml"),
            "name: workflow",
        )
        .unwrap();

        let library = LocalLibrary::new(
            test_project.project(),
            &LocalConfiguration::builder().path(".").build(),
        );

        let workflow = library.workflow("workflow").await.unwrap();

        assert_eq!(workflow.template().get(), "name: workflow");
    }

    #[tokio::test]
    async fn workflow_not_found() {
        let test_project = TestProject::new().unwrap();

        let library = LocalLibrary::new(
            test_project.project(),
            &LocalConfiguration::builder().path(".").build(),
        );

        let error = library.workflow("workflow").await.unwrap_err();

        assert_eq!(
            format!(
                "failed to find 'workflow.yml' in path {}/.",
                test_project.path().display()
            ),
            error.to_string()
        );
    }

    #[test]
    fn test_display() {
        let test_project = TestProject::new().unwrap();

        let library = LocalLibrary::new(
            test_project.project(),
            &LocalConfiguration::builder()
                .path("owner/repo/path")
                .build(),
        );

        let path = test_project.path().join("owner/repo/path");

        assert_eq!(library.to_string(), format!("path {}", path.display()));
    }
}
