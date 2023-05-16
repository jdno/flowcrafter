use std::fmt::{Display, Formatter};

use liquid::{Object, ParserBuilder};

use crate::Error;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Template(String);

impl Template {
    pub fn new(template: impl Into<String>) -> Self {
        Self(template.into())
    }

    pub fn render(&self, globals: &Object) -> Result<String, Error> {
        let template = ParserBuilder::with_stdlib().build()?.parse(&self.0)?;

        let output = template.render(&globals)?;

        Ok(output)
    }
}

impl Display for Template {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Template {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for Template {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use liquid::{object, Object};

    use super::*;

    #[test]
    fn render_empty() {
        let template = Template::new("");

        let render = template.render(&Object::new()).unwrap();

        assert_eq!("", render);
    }

    #[test]
    fn render_template() {
        let template = Template::new("{{num}}");

        let render = template
            .render(&object!({
                "num": 42
            }))
            .unwrap();

        assert_eq!("42", render);
    }

    #[test]
    fn trait_display() {
        let template = Template::new("{{foo}}");
        assert_eq!("{{foo}}", format!("{}", template));
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Template>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Template>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Template>();
    }
}
