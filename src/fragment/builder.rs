use std::fmt::{Display, Formatter};

use crate::fragment::{Fragment, FragmentError};
use crate::template::Template;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct FragmentBuilder {
    name: Option<String>,
    template: Option<Template>,
}

impl FragmentBuilder {
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

    pub fn build(self) -> Result<Fragment, FragmentError> {
        let name = self.name.ok_or(FragmentError::MissingField("name"))?;
        let template = self
            .template
            .ok_or(FragmentError::MissingField("template"))?;

        Ok(Fragment { name, template })
    }
}

impl Display for FragmentBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FragmentBuilder {{ name: {:?}, template: {:?} }}",
            self.name, self.template
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_requires_name() {
        let error = FragmentBuilder::new()
            .template(Template::new("template"))
            .build()
            .unwrap_err();

        assert_eq!(FragmentError::MissingField("name"), error);
    }

    #[test]
    fn build_requires_template() {
        let error = FragmentBuilder::new().name("name").build().unwrap_err();

        assert_eq!(FragmentError::MissingField("template"), error);
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<FragmentBuilder>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<FragmentBuilder>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<FragmentBuilder>();
    }
}
