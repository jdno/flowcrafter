use std::fmt::{Display, Formatter};

use typed_builder::TypedBuilder;

use crate::template::Template;

pub use self::error::FragmentError;
pub use self::library::FragmentLibrary;

mod error;
mod library;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, TypedBuilder)]
pub struct Fragment {
    #[builder(setter(into))]
    name: String,
    template: Template,
}

impl Fragment {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn template(&self) -> &Template {
        &self.template
    }
}

impl Display for Fragment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NAME: &str = "test";
    const TEMPLATE: &str = "{{template}}";

    fn fragment() -> Fragment {
        Fragment {
            name: NAME.to_string(),
            template: Template::new(TEMPLATE),
        }
    }

    #[test]
    fn name() {
        let fragment = fragment();

        assert_eq!(NAME, fragment.name());
    }

    #[test]
    fn template() {
        let fragment = fragment();

        assert_eq!(TEMPLATE, fragment.template().get());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Fragment>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Fragment>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Fragment>();
    }
}
