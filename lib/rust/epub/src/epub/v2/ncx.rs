use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::read_to_string;

use quick_xml::de::from_str;
use railgun::error::ResultExt;
use serde::Deserialize;
use zip::ZipArchive;
use zip::read::ZipFile;

use crate::Error;
use crate::epub::FromParameterizedZip;
use crate::error::IoErrorContext;
use crate::error::Result;
use crate::error::XmlErrorContext;
use crate::error::parse_error::MissingRequiredFileContext;

#[derive(Debug, Deserialize)]
pub struct Ncx {
    #[serde(rename = "navMap")]
    pub navmap: NavMap,
}

#[derive(Debug, Deserialize)]
pub struct NavMap {
    #[serde(rename = "navPoint")]
    pub nav_points: Vec<NavPoint>,
}

#[derive(Debug, Deserialize)]
pub struct NavPoint {
    #[serde(rename = "navLabel")]
    pub nav_label: NavLabel,
    pub content: Content,
}

#[derive(Debug, Deserialize)]
pub struct NavLabel {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct Content {
    #[serde(rename = "@src")]
    pub src: String,
}

impl TryFrom<ZipFile<'_>> for Ncx {
    type Error = Error;

    fn try_from(value: ZipFile<'_>) -> core::result::Result<Self, Self::Error> {
        let data = read_to_string(value).context(IoErrorContext {})?;
        let container: Self = from_str(&data).context(XmlErrorContext {})?;

        Ok(container)
    }
}

impl<'a> FromParameterizedZip<'a> for Ncx {
    type Params = &'a str;
    type Type = Ncx;

    fn parse(zip: &'a mut ZipArchive<impl Read + Seek>, path: Self::Params) -> Result<Self::Type> {
        let file = zip
            .by_name(path)
            .context(MissingRequiredFileContext { path })?;

        let data = read_to_string(file).context(IoErrorContext {})?;
        let package: Self = from_str(&data)?;

        Ok(package)
    }
}
