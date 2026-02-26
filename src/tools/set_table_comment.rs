use crate::db::DatabaseAdapter;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SetTableCommentInput {
    pub table: String,
    pub desc: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SetTableCommentOutput {
    pub table: String,
    pub desc: String,
}

pub async fn execute(
    db: &std::sync::Arc<dyn DatabaseAdapter>,
    args: &serde_json::Value,
) -> anyhow::Result<String> {
    let input: SetTableCommentInput = serde_json::from_value(args.clone())?;
    db.set_table_comment(&input.table, &input.desc).await?;
    let output = SetTableCommentOutput {
        table: input.table,
        desc: input.desc,
    };
    serde_json::to_string_pretty(&output).map_err(anyhow::Error::from)
}