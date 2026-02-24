use std::io::read_to_string;

use quick_xml::de::from_str;
use railgun::error::ResultExt;
use serde::Deserialize;
use serde::de::Visitor;
use zip::ZipArchive;

use crate::Error;
use crate::archive::EpubFile;
use crate::error::IoErrorContext;

#[derive(Deserialize)]
pub struct PackageSpecifier {
    #[serde(rename = "@version")]
    pub version: PackageVersion,
}

impl PackageSpecifier {
    pub fn parse(zip: &mut ZipArchive<EpubFile>, path: &str) -> Result<Self, Error> {
        let file = zip.by_name(path).unwrap();

        let data = read_to_string(file).context(IoErrorContext {}).unwrap();
        let container: Self = from_str(&data).unwrap();

        Ok(container)
    }
}

pub enum PackageVersion {
    V2,
    V3,
}

impl<'de> Deserialize<'de> for PackageVersion {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PackageVersionVisitor;

        impl<'de> Visitor<'de> for PackageVersionVisitor {
            type Value = PackageVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("`2.0`, or `3.0`")
            }

            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    "2.0" => Ok(PackageVersion::V2),
                    "3.0" => Ok(PackageVersion::V3),
                    _ => Err(E::custom(format!(
                        "Deserialized value '{v}' does not match the expected value `2.0`, or `3.0`"
                    ))),
                }
            }
        }

        deserializer.deserialize_identifier(PackageVersionVisitor)
    }
}
