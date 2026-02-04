use std::borrow::{Borrow, Cow};

#[derive(Clone, Debug)]
pub struct GenericType(pub Cow<'static, str>);

impl GenericType {
    pub const fn from_str(value: &'static str) -> GenericType {
        Self(Cow::Borrowed(value))
    }
}

impl Borrow<str> for GenericType {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl From<Cow<'static, str>> for GenericType {
    fn from(value: Cow<'static, str>) -> Self {
        Self(value)
    }
}
