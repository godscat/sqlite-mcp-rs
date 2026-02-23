use crate::db::{adapter::TableSchema, DatabaseAdapter};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetTableSchemaInput {
    pub table: String,
}

pub async fn execute(
    db: &std::sync::Arc<dyn DatabaseAdapter>,
    args: &serde_json::Value,
) -> anyhow::Result<String> {
    let input: GetTableSchemaInput = serde_json::from_value(args.clone())?;
    let schema: TableSchema = db.get_schema(&input.table).await?;
    serde_json::to_string_pretty(&schema).map_err(anyhow::Error::from)
}
