use crate::db::{adapter::QueryFilter, DatabaseAdapter};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct QueryRecordsInput {
    pub table: String,
    #[serde(default)]
    pub filters: Option<QueryFilter>,
    #[serde(default)]
    pub limit: Option<usize>,
    #[serde(default)]
    pub offset: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct QueryRecordsOutput {
    pub records: Vec<serde_json::Value>,
    pub total: usize,
}

pub async fn execute(
    db: &std::sync::Arc<dyn DatabaseAdapter>,
    args: &serde_json::Value,
) -> anyhow::Result<String> {
    let input: QueryRecordsInput = serde_json::from_value(args.clone())?;
    let records = db.select(&input.table, input.filters, input.limit, input.offset).await?;
    let total = records.len();
    let output = QueryRecordsOutput {
        records,
        total,
    };
    serde_json::to_string_pretty(&output).map_err(anyhow::Error::from)
}
