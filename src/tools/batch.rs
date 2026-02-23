use crate::db::DatabaseAdapter;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BatchInsertInput {
    pub table: String,
    pub items: Vec<serde_json::Value>,
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
}

fn default_batch_size() -> usize {
    50
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BatchInsertOutput {
    pub total: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub errors: Vec<String>,
    pub inserted_ids: Option<Vec<i64>>,
}

pub async fn insert_execute(
    db: &std::sync::Arc<dyn DatabaseAdapter>,
    args: &serde_json::Value,
) -> anyhow::Result<String> {
    let input: BatchInsertInput = serde_json::from_value(args.clone())?;

    if input.items.len() > 100 {
        return Err(anyhow::anyhow!("Maximum of 100 items allowed per batch"));
    }

    let result = db.batch_insert(&input.table, input.items, input.batch_size).await?;
    let output = BatchInsertOutput {
        total: result.total,
        succeeded: result.succeeded,
        failed: result.failed,
        errors: result.errors,
        inserted_ids: result.inserted_ids,
    };
    serde_json::to_string_pretty(&output).map_err(anyhow::Error::from)
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BatchUpdateInput {
    pub table: String,
    pub updates: Vec<BatchUpdateItem>,
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BatchUpdateItem {
    pub id: i64,
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BatchUpdateOutput {
    pub total: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub errors: Vec<String>,
}

pub async fn update_execute(
    db: &std::sync::Arc<dyn DatabaseAdapter>,
    args: &serde_json::Value,
) -> anyhow::Result<String> {
    let input: BatchUpdateInput = serde_json::from_value(args.clone())?;

    if input.updates.len() > 100 {
        return Err(anyhow::anyhow!("Maximum of 100 updates allowed per batch"));
    }

    let updates: Vec<(i64, serde_json::Value)> = input
        .updates
        .into_iter()
        .map(|item| (item.id, item.data))
        .collect();

    let result = db.batch_update(&input.table, updates, input.batch_size).await?;
    let output = BatchUpdateOutput {
        total: result.total,
        succeeded: result.succeeded,
        failed: result.failed,
        errors: result.errors,
    };
    serde_json::to_string_pretty(&output).map_err(anyhow::Error::from)
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BatchDeleteInput {
    pub table: String,
    pub ids: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BatchDeleteOutput {
    pub affected_rows: u64,
}

pub async fn delete_execute(
    db: &std::sync::Arc<dyn DatabaseAdapter>,
    args: &serde_json::Value,
) -> anyhow::Result<String> {
    let input: BatchDeleteInput = serde_json::from_value(args.clone())?;

    let affected = db.batch_delete(&input.table, input.ids).await?;
    let output = BatchDeleteOutput {
        affected_rows: affected as u64,
    };
    serde_json::to_string_pretty(&output).map_err(anyhow::Error::from)
}
