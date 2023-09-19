use std::fmt::Display;

use async_trait::async_trait;
use base64::engine::general_purpose::PAD;
use base64::engine::GeneralPurpose;
use base64::{alphabet, Engine};
use octocrab::models::repos::Content;
use octocrab::Octocrab;

use crate::error::Error;
use crate::fragment::{Fragment, FragmentLibrary};
use crate::github::GitHubConfiguration;
use crate::template::Template;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct GitHubLibrary<'a> {
    config: &'a GitHubConfiguration,
}

impl<'a> GitHubLibrary<'a> {
    pub fn new(config: &'a GitHubConfiguration) -> Self {
        Self { config }
    }

    async fn download(&self, name: &str, path: &str) -> Result<Fragment, Error> {
        let file = self.fetch_from_github(path).await?;
        let content = self.decode_content(file)?;

        let fragment = Fragment::builder()
            .name(name)
            .template(Template::new(content))
            .build();

        Ok(fragment)
    }

    async fn fetch_from_github(&self, path: &str) -> Result<Content, Error> {
        let items = Octocrab::builder()
            .base_uri(self.config.instance().to_string())?
            .build()?
            .repos(self.config.owner().get(), self.config.repository().get())
            .get_content()
            .path(path)
            .send()
            .await?;

        items
            .items
            .into_iter()
            .next()
            .ok_or(Error::NotFound(path.into(), self.to_string()))
    }

    fn decode_content(&self, content: Content) -> Result<String, Error> {
        let base64_encoded_file = content.content.ok_or(Error::InvalidTemplate(
            "template from GitHub is empty".into(),
        ))?;

        let bytes = base64_decode(&base64_encoded_file).map_err(|_| {
            Error::InvalidTemplate("failed to base64 decode template from GitHub".into())
        })?;

        String::from_utf8(bytes).map_err(|_| {
            Error::InvalidTemplate(format!(
                "failed to decode template '{}' as UTF-8",
                content.path
            ))
        })
    }
}

#[async_trait]
impl<'a> FragmentLibrary<'a> for GitHubLibrary<'a> {
    async fn workflow(&self, name: &'a str) -> Result<Fragment, Error> {
        let path = format!("{name}/workflow.yml");
        self.download(name, &path).await
    }

    async fn job(&self, workflow: &'a str, name: &'a str) -> Result<Fragment, Error> {
        let path = format!("{workflow}/{name}.yml");
        self.download(name, &path).await
    }
}

impl Display for GitHubLibrary<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "repository {}/{}",
            self.config.owner(),
            self.config.repository()
        )
    }
}

fn base64_decode(base64_encoded_string: &str) -> Result<Vec<u8>, base64::DecodeError> {
    let sanitized_input = base64_encoded_string.replace('\n', "");

    GeneralPurpose::new(&alphabet::STANDARD, PAD).decode(sanitized_input)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::github::{Owner, Repository};

    use super::*;

    const WORKFLOW: &str = indoc! {r#"
        ---
        name: Test

        "on":
          push:

        jobs: {}
    "#};

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

    const JOB: &str = indoc! {r#"
        ---
        name: Job
        runs-on: ubuntu-latest

        steps: []
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

    fn build_config(server_url: &str) -> GitHubConfiguration {
        GitHubConfiguration::builder()
            .instance(server_url.parse().unwrap())
            .owner("owner")
            .repository("name")
            .build()
    }

    #[tokio::test]
    async fn workflow() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/repos/owner/name/contents/test/workflow.yml")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body(WORKFLOW_RESPONSE)
            .create();

        let config = build_config(&server.url());
        let library = GitHubLibrary::new(&config);

        let workflow = library.workflow("test").await.unwrap();

        mock.assert();
        assert_eq!("test", workflow.name());
        assert_eq!(&Template::new(WORKFLOW), workflow.template());
    }

    #[tokio::test]
    async fn job() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/repos/owner/name/contents/test/job.yml")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body(JOB_RESPONSE)
            .create();

        let config = build_config(&server.url());
        let library = GitHubLibrary::new(&config);

        let job = library.job("test", "job").await.unwrap();

        mock.assert();
        assert_eq!("job", job.name());
        assert_eq!(&Template::new(JOB), job.template());
    }

    #[test]
    fn base64_decode_ok() {
        let base64_encoded_string =
            "LS0tCm5hbWU6IEpvYgpydW5zLW9uOiB1YnVudHUtbGF0ZXN0CgpzdGVwczogW10K";

        let decoded_bytes = base64_decode(base64_encoded_string).unwrap();
        let decoded_string = String::from_utf8(decoded_bytes).unwrap();

        assert_eq!(JOB, decoded_string);
    }

    #[test]
    fn base64_decode_ignores_newlines() {
        let base64_encoded_string = indoc!(
            r#"
            LS0tCm5hbW
            U6IEpvYgpy
            dW5zLW9uOi
            B1YnVudHUt
            bGF0ZXN0Cg
            pzdGVwczog
            W10K
            "#
        );

        let decoded_bytes = base64_decode(base64_encoded_string).unwrap();
        let decoded_string = String::from_utf8(decoded_bytes).unwrap();

        assert_eq!(JOB, decoded_string);
    }

    #[test]
    fn decode_content() {
        let config = build_config("https://example.com");
        let library = GitHubLibrary::new(&config);

        let content = serde_json::from_str::<Content>(WORKFLOW_RESPONSE).unwrap();
        let workflow = library.decode_content(content).unwrap();

        assert_eq!(WORKFLOW, workflow);
    }

    #[tokio::test]
    async fn fetch_from_github() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/repos/owner/name/contents/test/workflow.yml")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body(WORKFLOW_RESPONSE)
            .create();

        let config = build_config(&server.url());
        let library = GitHubLibrary::new(&config);

        let content = library
            .fetch_from_github("test/workflow.yml")
            .await
            .unwrap();

        mock.assert();
        assert_eq!(40, content.size);
    }

    #[tokio::test]
    async fn fetch_from_github_not_found() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/repos/owner/name/contents/test/workflow.yml")
            .match_query(mockito::Matcher::Any)
            .with_status(404)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body(indoc!(
                r#"
                {
                    "message": "Not Found",
                    "documentation_url": "https://docs.github.com/rest/reference/repos#get-repository-content"
                }
                "#
            ))
            .create();

        let config = build_config(&server.url());
        let library = GitHubLibrary::new(&config);

        let error = library
            .fetch_from_github("test/workflow.yml")
            .await
            .unwrap_err();

        mock.assert();
        assert!(error.to_string().contains("Not Found"));
    }

    #[test]
    fn trait_display() {
        let configuration = GitHubConfiguration::builder()
            .owner(Owner::from("jdno"))
            .repository(Repository::from("flowcrafter"))
            .build();

        let library = GitHubLibrary::new(&configuration);

        assert_eq!("repository jdno/flowcrafter", library.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<GitHubLibrary>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<GitHubLibrary>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<GitHubLibrary>();
    }
}
