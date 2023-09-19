use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

use typed_builder::TypedBuilder;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LocalConfiguration {
    #[builder(setter(into))]
    path: PathBuf,
}

impl LocalConfiguration {
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Display for LocalConfiguration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "local: {}", self.path.display())
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[cfg(feature = "serde")]
    #[test]
    fn trait_deserialize() {
        let yaml = indoc!(
            r#"
            ---
            path: .
            "#
        );

        let configuration = serde_yaml::from_str::<LocalConfiguration>(yaml).unwrap();

        assert_eq!(".", configuration.path().to_str().unwrap());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<LocalConfiguration>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<LocalConfiguration>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<LocalConfiguration>();
    }
}
