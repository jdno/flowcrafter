use async_trait::async_trait;

use crate::error::Error;
use crate::fragment::Fragment;

#[async_trait]
pub trait FragmentLibrary {
    async fn workflow(&self, name: &str) -> Result<Fragment, Error>;
    async fn job(&self, workflow: &str, name: &str) -> Result<Fragment, Error>;
}
