use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use super::dmmf::Document;

/// The message container that Prisma uses to communicate with its
/// generators and clients.
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: i32,
    pub method: String,
    pub params: Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: i32,
    #[serde(flatten)]
    pub data: JsonRpcResponseData,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum JsonRpcResponseData {
    Result(Value),
    Error { code: i32, message: String },
}

///
#[derive(Debug, Deserialize, Serialize)]
pub struct ManifestResponse {
    pub manifest: Manifest,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub pretty_name: String,
    pub default_output: String,
    pub denylist: Option<Vec<String>>,
    pub requires_generators: Option<Vec<String>>,
    pub requires_engines: Option<Vec<String>>,
}

///
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct GenerateRequest {
    pub generator: Generator,
    pub other_generators: Vec<Generator>,
    pub schema_path: String,
    // The version hash of the prisma engine
    pub version: String,
    pub dmmf: Document,
    pub datasources: Vec<Datasource>,
    pub datamodel: String,
    pub binary_paths: Option<BinaryPaths>,
    pub ast: Option<String>,
    pub postinstall: bool,
    pub no_engine: bool,
    pub allow_no_models: bool,
    pub env_paths: HashMap<String, Value>,
}

/// The prisma generator metadata as defined in the schema.prisma
/// generator blocks.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Generator {
    pub name: String,
    pub source_file_path: String,
    pub provider: PrismaValue,
    pub output: PrismaValue,
    // Custom config (not implemented)
    pub config: Value,
    pub pinned_binary_target: Option<String>,
    pub preview_features: Vec<String>,
    pub is_custom_output: bool,
}

///
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Datasource {
    pub name: String,
    pub source_file_path: String,
    pub provider: String,        // Provider,
    pub active_provider: String, // Provider,
    pub url: PrismaValue,
    // Custom config (not implemented)
    pub config: Option<Value>,
    pub schemas: Value,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[serde(deny_unknown_fields)]
pub enum Provider {
    SQLite,
    MySQL,
    Postgres,
}

/// Paths to prisma binaries, which may or may not be provided.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct BinaryPaths {
    pub migration_engine: HashMap<String, String>,
    pub query_engine: HashMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct BinaryTarget {
    pub from_env_var: Option<String>,
    pub value: String,
    pub native: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct PrismaValue {
    pub from_env_var: Option<String>,
    pub value: String,
}

impl JsonRpcRequest {
    pub fn parse(data: &'_ str) -> Self {
        serde_json::from_str(data).expect("Failed to parse Prisma request")
    }
}

impl JsonRpcResponseData {
    pub fn into_response(self, request_id: i32) -> JsonRpcResponse {
        JsonRpcResponse {
            jsonrpc: "2.0".into(),
            id: request_id,
            data: self,
        }
    }
}

impl JsonRpcResponse {
    pub fn into_bytes(self) -> Vec<u8> {
        let mut response_bytes =
            serde_json::to_vec(&self).expect("Failed to serialize Prisma response");

        response_bytes.push(b'\n');

        response_bytes
    }
}
