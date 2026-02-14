use super::kind::Kind;
use crate::domain::entity::setting::Setting;

pub mod data_path;

pub trait SettingMeta {
    type Value: Kind;

    /// The name of the setting
    const NAME: &'static str;

    const DISPLAY_NAME: &'static str;

    const DESCRIPTION: &'static str;

    fn into_aggregate(&self) -> Setting;

    fn from_parts(name: String, kind: String, value: String) -> Self;

    // async fn init(db: &PrismaClient) -> Result<Self::Value>;

    // async fn update(db: &PrismaClient, value: Self::Value) -> Result<()>;

    // fn from_kind(kind: &SettingKind) -> Result<Self>
    // where
    //     Self: Sized;
    //
    //     async fn value(db: &PrismaClient) -> Result<Self::Value>;
    //
    //     fn kind(&self) -> SettingKind;
}
