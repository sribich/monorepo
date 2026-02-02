//! Prisma DMMF model parsing.
//!
//! The definitions in this file come from both manual parsing of the prisma
//! generator requests and from the prisma source.
//!
//! The official DMMF structure can be found here:
//!
//!   https://github.com/prisma/prisma/blob/main/packages/generator-helper/src/dmmf.ts
//!
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub datamodel: Datamodel,
    pub schema: Schema,
    pub mappings: Mappings,
}

/// All types that exist within the prisma datamodel.
///
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Datamodel {
    pub enums: Vec<DatamodelEnum>,
    pub models: Vec<Model>,
    pub types: Vec<Model>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatamodelEnum {
    pub name: String,
    pub values: Vec<EnumValue>,
    pub db_name: Option<String>,
    pub documentation: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EnumValue {
    pub name: String,
    pub db_name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub name: String,
    pub db_name: Option<String>,
    pub fields: Vec<Field>,
    pub primary_key: Option<PrimaryKey>,
    pub unique_fields: Vec<Vec<String>>,
    pub unique_indexes: Vec<UniqueIndex>,
    pub is_generated: Option<bool>,
    pub documentation: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub kind: FieldKind,
    pub name: String,
    /// Describes the data type in the same the way it is defined in the Prisma schema:
    /// BigInt, Boolean, Bytes, DateTime, Decimal, Float, Int, JSON, String, $ModelName
    pub r#type: String,
    pub is_list: bool,
    pub is_required: bool,
    pub is_unique: bool,
    pub is_id: bool,
    pub is_read_only: bool,
    pub is_generated: Option<bool>, // Does not exist on 'type' but does on 'model'
    pub is_updated_at: Option<bool>, // Does not exist on 'type' but does on 'model'
    pub db_name: Option<String>,
    pub has_default_value: bool,
    pub default: Option<Value>,
    pub relation_name: Option<String>,
    pub relation_to_fields: Option<Vec<String>>,
    pub relation_from_fields: Option<Vec<String>>,
    pub relation_on_delete: Option<String>,
    pub documentation: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FieldKind {
    Object,
    Scalar,
    Enum,
    Unsupported,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FieldNamespace {
    Prisma,
    Model,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FieldLocation {
    Scalar,
    InputObjectTypes,
    OutputObjectTypes,
    EnumTypes,
    FieldRefTypes,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryKey {
    pub name: Option<String>,
    pub fields: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UniqueIndex {
    pub name: Option<String>,
    pub fields: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub root_query_type: Option<String>,
    pub root_mutation_type: Option<String>,
    pub input_object_types: InputTypes,
    pub output_object_types: OutputTypes,
    pub enum_types: EnumTypes,
    pub field_ref_types: FieldRefTypes,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InputTypes {
    pub model: Option<Vec<InputType>>,
    pub prisma: Vec<InputType>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputTypes {
    pub model: Vec<OutputType>,
    pub prisma: Vec<OutputType>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumTypes {
    pub model: Option<Vec<SchemaEnum>>,
    pub prisma: Vec<SchemaEnum>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldRefTypes {
    pub model: Option<Vec<FieldRefType>>,
    pub prisma: Vec<FieldRefType>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaEnum {
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InputType {
    pub name: String,
    pub constraints: Constraints,
    pub fields: Vec<SchemaArg>,
    pub meta: Option<InputMeta>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InputMeta {
    source: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaArg {
    pub name: String,
    pub comment: Option<String>,
    pub input_types: Vec<InputTypeRef>,
    pub is_nullable: bool,
    pub is_required: bool,
    pub deprecation: Option<Deprecation>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum InputFieldLocation {
    Scalar,
    InputObjectTypes,
    EnumTypes,
    FieldRefTypes,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InputTypeRef {
    pub is_list: bool,
    pub r#type: String,
    pub location: InputFieldLocation,
    pub namespace: Option<FieldNamespace>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Constraints {
    pub max_num_fields: Option<i32>,
    pub min_num_fields: Option<i32>,
    pub fields: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputType {
    pub name: String,
    pub fields: Vec<SchemaField>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaField {
    pub name: String,
    pub is_nullable: Option<bool>,
    pub output_type: OutputTypeRef,
    pub args: Vec<SchemaArg>,
    pub deprecation: Option<Deprecation>,
    pub documentation: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OutputFieldLocation {
    Scalar,
    OutputObjectTypes,
    EnumTypes,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputTypeRef {
    pub is_list: bool,
    pub r#type: String,
    pub location: OutputFieldLocation,
    pub namespace: Option<FieldNamespace>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldRefType {
    pub name: String,
    pub allow_types: Vec<FieldRefAllowType>,
    pub fields: Vec<SchemaArg>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FieldRefFieldLocation {
    Scalar,
    EnumTypes,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldRefAllowType {
    pub is_list: bool,
    pub r#type: String,
    pub location: FieldRefFieldLocation,
    pub namespace: Option<FieldNamespace>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Mappings {
    pub model_operations: Vec<ModelMapping>,
    pub other_operations: OtherOperationMappings,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelMapping {
    model: String,
    plural: Option<String>,
    find_unique: Option<String>,
    find_unique_or_throw: Option<String>,
    find_first: Option<String>,
    find_first_or_throw: Option<String>,
    find_many: Option<String>,
    create: Option<String>,
    create_many: Option<String>,
    update: Option<String>,
    update_many: Option<String>,
    upsert: Option<String>,
    delete: Option<String>,
    delete_many: Option<String>,
    aggregate: Option<String>,
    group_by: Option<String>,
    count: Option<String>,
    find_raw: Option<String>,
    aggregate_raw: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherOperationMappings {
    pub read: Vec<String>,
    pub write: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Deprecation {
    since_version: String,
    reason: String,
    planned_removal_version: Option<String>,
}
