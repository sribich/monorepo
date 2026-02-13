#[derive(Clone)]
pub enum Sex {
    M,
    F,
}

impl Sex {
    pub fn from_str<S: AsRef<str>>(value: S) -> Sex {
        match value.as_ref() {
            "m" => Sex::M,
            "f" => Sex::F,
            _ => panic!("Unknown sex {}", value.as_ref()),
        }
    }
}

impl ToString for Sex {
    fn to_string(&self) -> String {
        match self {
            Sex::M => "m".to_string(),
            Sex::F => "f".to_string(),
        }
    }
}
