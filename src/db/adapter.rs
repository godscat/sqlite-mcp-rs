use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait DatabaseAdapter: Send + Sync {
    async fn list_tables(&self) -> Result<Vec<String>>;
    async fn get_schema(&self, table: &str) -> Result<TableSchema>;
    async fn select(
        &self,
        table: &str,
        filters: Option<QueryFilter>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<serde_json::Value>>;
    async fn insert(&self, table: &str, data: serde_json::Value) -> Result<i64>;
    async fn update(
        &self,
        table: &str,
        pk_value: i64,
        data: serde_json::Value,
    ) -> Result<usize>;
    async fn delete(&self, table: &str, pk_value: i64) -> Result<usize>;
    async fn batch_insert(
        &self,
        table: &str,
        items: Vec<serde_json::Value>,
        batch_size: usize,
    ) -> Result<BatchResult>;
    async fn batch_update(
        &self,
        table: &str,
        updates: Vec<(i64, serde_json::Value)>,
        batch_size: usize,
    ) -> Result<BatchResult>;
    async fn batch_delete(&self, table: &str, ids: Vec<i64>) -> Result<usize>;

    #[allow(dead_code)]
    async fn is_readonly(&self) -> bool;
}

#[derive(Debug, Clone, Serialize)]
pub struct TableSchema {
    pub name: String,
    pub columns: Vec<ColumnInfo>,
    pub primary_key: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
    pub not_null: bool,
    pub default_value: Option<String>,
    pub is_primary_key: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct BatchResult {
    pub total: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub errors: Vec<String>,
    pub inserted_ids: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct QueryFilter {
    #[serde(flatten)]
    pub conditions: std::collections::HashMap<String, FilterValue>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum FilterValue {
    Direct(serde_json::Value),
    Operator(FilterOperators),
}

impl<'de> serde::Deserialize<'de> for FilterValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        if let serde_json::Value::Object(obj) = &value {
            // 检查是否有任何 $ 操作符
            if obj.keys().any(|k| k.starts_with('$')) {
                // 尝试解析为 FilterOperators
                if let Ok(ops) = serde_json::from_value::<FilterOperators>(value.clone()) {
                    return Ok(FilterValue::Operator(ops));
                }
            }
        }

        Ok(FilterValue::Direct(value))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FilterOperators {
    #[serde(rename = "$eq")]
    #[serde(default)]
    pub eq: Option<serde_json::Value>,
    #[serde(rename = "$ne")]
    #[serde(default)]
    pub ne: Option<serde_json::Value>,
    #[serde(rename = "$gt")]
    #[serde(default)]
    pub gt: Option<serde_json::Value>,
    #[serde(rename = "$gte")]
    #[serde(default)]
    pub gte: Option<serde_json::Value>,
    #[serde(rename = "$lt")]
    #[serde(default)]
    pub lt: Option<serde_json::Value>,
    #[serde(rename = "$lte")]
    #[serde(default)]
    pub lte: Option<serde_json::Value>,
    #[serde(rename = "$in")]
    #[serde(default)]
    pub in_list: Option<Vec<serde_json::Value>>,
    #[serde(rename = "$like")]
    #[serde(default)]
    pub like: Option<String>,
}

impl schemars::JsonSchema for FilterValue {
    fn schema_name() -> String {
        "FilterValue".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        gen.subschema_for::<serde_json::Value>()
    }
}
