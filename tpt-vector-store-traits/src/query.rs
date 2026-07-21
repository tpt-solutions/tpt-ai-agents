use serde::{Deserialize, Serialize};

/// Vector search query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    pub vector: alloc::vec::Vec<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Filter>,
    #[serde(default = "default_limit")]
    pub limit: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_payload: Option<bool>,
}

fn default_limit() -> usize {
    10
}

/// Filter for vector queries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    pub must: alloc::vec::Vec<FilterCondition>,
}

/// A single filter condition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterCondition {
    pub key: alloc::string::String,
    pub op: FilterOp,
    pub value: FilterValue,
}

/// Filter operator.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterOp {
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
    Contains,
}

/// Filter value.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilterValue {
    String(alloc::string::String),
    Number(f64),
    Bool(bool),
}
