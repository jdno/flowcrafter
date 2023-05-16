use std::fmt::{Display, Formatter};

use anyhow::{anyhow, Context};
use base64::engine::general_purpose::{GeneralPurpose, PAD};
use base64::{alphabet, Engine};
use octocrab::models::repos::Content;
use octocrab::Octocrab;
use url::Url;

use crate::{Error, Job, JobBuilder, Workflow, WorkflowBuilder};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Library {
    instance: Url,
    owner: String,
    name: String,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct LibraryBuilder {
    instance: Url,
    owner: Option<String>,
    name: Option<String>,
}

impl Library {
    pub fn instance(&self) -> &Url {
        &self.instance
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub async fn workflow(&self, name: &str) -> Result<Workflow, Error> {
        let path = format!("{name}/workflow.yml");

        let workflow = self.download(&path).await?;
        let content = self.decode_content(workflow)?;

        WorkflowBuilder::new()
            .name(name)
            .template(content.into())
            .build()
    }

    pub async fn job(&self, workflow: &str, name: &str) -> Result<Job, Error> {
        let path = format!("{workflow}/{name}.yml");

        let job = self.download(&path).await?;
        let content = self.decode_content(job)?;

        JobBuilder::new()
            .name(name)
            .template(content.into())
            .build()
    }

    async fn download(&self, path: &str) -> Result<Content, Error> {
        let items = Octocrab::builder()
            .base_url(self.instance.clone())
            .context("failed to set URL for GitHub API")?
            .build()
            .context("failed to initialize GitHub API client")?
            .repos(&self.owner, &self.name)
            .get_content()
            .path(path)
            .send()
            .await
            .context(format!("failed to download '{path}' from GitHub"))?;

        items.items.into_iter().next().ok_or(
            anyhow!(format!(
                "failed to find '{path}' in repository {}/{}",
                self.owner, self.name
            ))
            .into(),
        )
    }

    fn decode_content(&self, content: Content) -> Result<String, Error> {
        let base64_encoded_file = content
            .content
            .context(format!("'{}' is empty", content.path))?;

        let bytes = GeneralPurpose::new(&alphabet::STANDARD, PAD)
            .decode(base64_encoded_file)
            .context(format!("failed to decode '{}'", content.path))?;

        String::from_utf8(bytes)
            .map_err(|_| anyhow!(format!("failed to decode '{}' as UTF-8", content.path)).into())
    }
}

impl LibraryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn instance(mut self, instance: Url) -> Self {
        self.instance = instance;
        self
    }

    pub fn owner(mut self, owner: String) -> Self {
        self.owner = Some(owner);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn build(self) -> Result<Library, Error> {
        let instance = self.instance;
        let owner = self.owner.context("missing field 'owner'")?;
        let name = self.name.context("missing field 'name'")?;

        Ok(Library {
            instance,
            owner,
            name,
        })
    }
}

impl Default for LibraryBuilder {
    fn default() -> Self {
        Self {
            instance: Url::parse("https://api.github.com").unwrap(),
            owner: None,
            name: None,
        }
    }
}

impl Display for Library {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.owner, self.name)
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::Template;

    use super::*;

    const WORKFLOW_RESPONSE: &str = indoc! {r#"
        {
            "type": "file",
            "encoding": "base64",
            "size": 40,
            "name": "workflow.yml",
            "path": "test/workflow.yml",
            "content": "LS0tCm5hbWU6IFRlc3QKCiJvbiI6CiAgcHVzaDoKCmpvYnM6IHt9Cg==",
            "sha": "3d21ec53a331a6f037a91c368710b99387d012c1",
            "url": "https://api.github.com/repos/owner/name/contents/test/workflow.yml",
            "git_url": "https://api.github.com/repos/owner/name/git/blobs/3d21ec53a331a6f037a91c368710b99387d012c1",
            "html_url": "https://github.com/owner/name/blob/master/test/workflow.yml",
            "download_url": "https://raw.githubusercontent.com/owner/name/master/test/workflow.yml",
            "_links": {
                "git": "https://api.github.com/repos/owner/name/git/blobs/3d21ec53a331a6f037a91c368710b99387d012c1",
                "self": "https://api.github.com/repos/owner/name/contents/test/workflow.yml",
                "html": "https://github.com/owner/name/blob/master/test/workflow.yml"
            }
        }
    "#};

    const WORKFLOW: &str = indoc! {r#"
        ---
        name: Test

        "on":
          push:

        jobs: {}
    "#};

    const JOB_RESPONSE: &str = indoc! {r#"
        {
            "type": "file",
            "encoding": "base64",
            "size": 40,
            "name": "job.yml",
            "path": "test/job.yml",
            "content": "LS0tCm5hbWU6IEpvYgpydW5zLW9uOiB1YnVudHUtbGF0ZXN0CgpzdGVwczogW10K",
            "sha": "3d21ec53a331a6f037a91c368710b99387d012c1",
            "url": "https://api.github.com/repos/owner/name/contents/test/job.yml",
            "git_url": "https://api.github.com/repos/owner/name/git/blobs/3d21ec53a331a6f037a91c368710b99387d012c1",
            "html_url": "https://github.com/owner/name/blob/master/test/job.yml",
            "download_url": "https://raw.githubusercontent.com/owner/name/master/test/job.yml",
            "_links": {
                "git": "https://api.github.com/repos/owner/name/git/blobs/3d21ec53a331a6f037a91c368710b99387d012c1",
                "self": "https://api.github.com/repos/owner/name/contents/test/job.yml",
                "html": "https://github.com/owner/name/blob/master/test/job.yml"
            }
        }
    "#};

    const JOB: &str = indoc! {r#"
        ---
        name: Job
        runs-on: ubuntu-latest

        steps: []
    "#};

    fn build_library(server_url: &str) -> Library {
        LibraryBuilder::new()
            .instance(server_url.parse().unwrap())
            .owner("owner".to_string())
            .name("name".to_string())
            .build()
            .unwrap()
    }

    #[tokio::test]
    async fn test_workflow() {
        let mut server = mockito::Server::new();
        let _mock = server
            .mock("GET", "/repos/owner/name/contents/test/workflow.yml")
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body(WORKFLOW_RESPONSE)
            .create();

        let library = build_library(&server.url());
        let workflow = library.workflow("test").await.unwrap();

        assert_eq!("test", workflow.name());
        assert_eq!(&Template::new(WORKFLOW), workflow.template());
    }

    #[tokio::test]
    async fn test_job() {
        let mut server = mockito::Server::new();
        let _mock = server
            .mock("GET", "/repos/owner/name/contents/test/job.yml")
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body(JOB_RESPONSE)
            .create();

        let library = build_library(&server.url());
        let job = library.job("test", "job").await.unwrap();

        assert_eq!("job", job.name());
        assert_eq!(&Template::new(JOB), job.template());
    }

    #[tokio::test]
    async fn test_download_workflow() {
        let mut server = mockito::Server::new();
        let _mock = server
            .mock("GET", "/repos/owner/name/contents/test/workflow.yml")
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body(WORKFLOW_RESPONSE)
            .create();

        let library = build_library(&server.url());
        let content = library.download("test/workflow.yml").await.unwrap();

        assert_eq!(40, content.size);
    }

    #[test]
    fn test_decode_content() {
        let library = LibraryBuilder::new()
            .owner("owner".to_string())
            .name("name".to_string())
            .build()
            .unwrap();

        let content = serde_json::from_str::<Content>(WORKFLOW_RESPONSE).unwrap();
        let workflow = library.decode_content(content).unwrap();

        assert_eq!(WORKFLOW, workflow);
    }

    #[test]
    fn trait_display() {
        let library = LibraryBuilder::new()
            .owner("owner".to_string())
            .name("name".to_string())
            .build()
            .unwrap();

        assert_eq!("owner/name", library.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Library>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Library>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Library>();
    }
}
