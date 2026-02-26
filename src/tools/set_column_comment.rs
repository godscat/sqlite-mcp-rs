use crate::db::DatabaseAdapter;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SetColumnCommentInput {
    pub table: String,
    pub column: String,
    pub desc: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SetColumnCommentOutput {
    pub table: String,
    pub column: String,
    pub desc: String,
}

pub async fn execute(
    db: &std::sync::Arc<dyn DatabaseAdapter>,
    args: &serde_json::Value,
) -> anyhow::Result<String> {
    let input: SetColumnCommentInput = serde_json::from_value(args.clone())?;
    db.set_column_comment(&input.table, &input.column, &input.desc).await?;
    let output = SetColumnCommentOutput {
        table: input.table,
        column: input.column,
        desc: input.desc,
    };
    serde_json::to_string_pretty(&output).map_err(anyhow::Error::from)
}
