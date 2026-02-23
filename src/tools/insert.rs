use crate::db::DatabaseAdapter;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct InsertRecordInput {
    pub table: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct InsertRecordOutput {
    pub id: i64,
    pub affected_rows: u64,
}

pub async fn execute(
    db: &std::sync::Arc<dyn DatabaseAdapter>,
    args: &serde_json::Value,
) -> anyhow::Result<String> {
    let input: InsertRecordInput = serde_json::from_value(args.clone())?;
    let id = db.insert(&input.table, input.data).await?;
    let output = InsertRecordOutput {
        id,
        affected_rows: 1,
    };
    serde_json::to_string_pretty(&output).map_err(anyhow::Error::from)
}
