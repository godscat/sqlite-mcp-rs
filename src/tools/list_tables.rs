use crate::db::DatabaseAdapter;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListTablesOutput {
    pub tables: Vec<String>,
}

pub async fn execute(
    db: &std::sync::Arc<dyn DatabaseAdapter>,
    _args: &serde_json::Value,
) -> anyhow::Result<String> {
    let tables = db.list_tables().await?;
    let output = ListTablesOutput { tables };
    serde_json::to_string_pretty(&output).map_err(anyhow::Error::from)
}
