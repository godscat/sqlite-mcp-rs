use crate::db::DatabaseAdapter;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdateRecordInput {
    pub table: String,
    pub id: i64,
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdateRecordOutput {
    pub affected_rows: u64,
}

pub async fn execute(
    db: &std::sync::Arc<dyn DatabaseAdapter>,
    args: &serde_json::Value,
) -> anyhow::Result<String> {
    let input: UpdateRecordInput = serde_json::from_value(args.clone())?;
    let affected = db.update(&input.table, input.id, input.data).await?;
    let output = UpdateRecordOutput {
        affected_rows: affected as u64,
    };
    serde_json::to_string_pretty(&output).map_err(anyhow::Error::from)
}
