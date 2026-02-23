use crate::db::DatabaseAdapter;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeleteRecordInput {
    pub table: String,
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeleteRecordOutput {
    pub affected_rows: u64,
}

pub async fn execute(
    db: &std::sync::Arc<dyn DatabaseAdapter>,
    args: &serde_json::Value,
) -> anyhow::Result<String> {
    let input: DeleteRecordInput = serde_json::from_value(args.clone())?;
    let affected = db.delete(&input.table, input.id).await?;
    let output = DeleteRecordOutput {
        affected_rows: affected as u64,
    };
    serde_json::to_string_pretty(&output).map_err(anyhow::Error::from)
}
