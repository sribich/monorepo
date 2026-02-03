use railgun::error::Error;
use railgun::error::Location;

#[derive(Error)]
pub struct LanguageTypeError {
    value: String,
    location: Location,
}

#[derive(Clone)]
pub enum LanguageType {
    Monolingual,
    Bilingual,
}

impl LanguageType {
    pub fn from_str<S: AsRef<str>>(value: S) -> Result<LanguageType, LanguageTypeError> {
        match value.as_ref() {
            "mono" => Ok(LanguageType::Monolingual),
            "bi" => Ok(LanguageType::Bilingual),
            _ => LanguageTypeErrorContext {
                value: value.as_ref().to_owned(),
            }
            .fail(),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            LanguageType::Monolingual => "mono",
            LanguageType::Bilingual => "bi",
        }
    }
}
