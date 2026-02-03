#[derive(Clone, Debug)]
pub struct MediaTitle(String);

impl MediaTitle {
    pub fn new<S: AsRef<str>>(title: S) -> Self {
        Self(title.as_ref().to_string())
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}
