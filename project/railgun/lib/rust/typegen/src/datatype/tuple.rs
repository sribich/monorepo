use super::DataType;

#[derive(Debug, Clone)]
pub struct TupleMeta {
    pub elements: Vec<DataType>,
}
