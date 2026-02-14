use crate::domain::SettingKind;

pub mod existing_path;

pub trait Kind {
    const KIND: &'static str;

    fn parse(data: &prisma_client::model::setting::Data) -> SettingKind;

    fn name(&self) -> String;

    fn as_value(&self) -> String;
    fn as_constraints(&self) -> Option<String>;

    fn as_kind(&self) -> SettingKind;

    fn from_parts(kind: String, value: String) -> Self;
}
