use std::fmt::Display;

use async_trait::async_trait;

use crate::error::Error;
use crate::fragment::Fragment;

#[async_trait]
pub trait FragmentLibrary<'a>: Display {
    async fn workflow(&self, name: &'a str) -> Result<Fragment, Error>;
    async fn job(&self, workflow: &'a str, name: &'a str) -> Result<Fragment, Error>;
}
