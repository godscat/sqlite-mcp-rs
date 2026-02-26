mod db;
mod tools;

use anyhow::anyhow;
use clap::Parser;
use db::SqliteDatabase;
use std::sync::Arc;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing::info;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[derive(Parser, Debug)]
#[command(name = "sqlite-mcp-rs")]
#[command(about = "SQLite MCP Server - Fast Rust implementation with JSON-based database operations", long_about = None)]
struct Args {
    /// SQLite database file path
    #[arg(short, long)]
    db_path: std::path::PathBuf,

    /// Read-only mode (default: false)
    #[arg(short, long, default_value = "false")]
    readonly: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    init_logging();

    info!("Starting SQLite MCP Server...");
    info!("Database path: {:?}", args.db_path);
    info!("Read-only mode: {}", args.readonly);

    let db: Arc<dyn db::DatabaseAdapter> = Arc::new(SqliteDatabase::new(&args.db_path, args.readonly)?);

    run_stdio_server(db).await?;

    Ok(())
}

fn init_logging() {
    let log_level = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "sqlite_mcp=info,tokio=info".to_string());
    
    let filter = EnvFilter::new(&log_level);

    FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .init();
}

async fn run_stdio_server(db: Arc<dyn db::DatabaseAdapter>) -> anyhow::Result<()> {
    let mut reader = BufReader::new(io::stdin());
    let mut writer = io::stdout();

    let mut line = String::new();

    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).await?;

        if bytes_read == 0 {
            break;
        }

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        tracing::debug!("Received: {}", line);

        let parsed: serde_json::Result<serde_json::Value> = serde_json::from_str(line);
        let request_id = parsed.as_ref().ok().and_then(|v| v.get("id").cloned());

        match handle_message(&db, line).await {
            Ok(response) => {
                if let Some(resp) = response {
                    let response_json = serde_json::to_string(&resp)?;
                    tracing::debug!("Sending: {}", response_json);
                    writer.write_all(response_json.as_bytes()).await?;
                    writer.write_all(b"\n").await?;
                    writer.flush().await?;
                }
            }
            Err(e) => {
                tracing::error!("Error handling message: {}", e);
                let error_response = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": request_id.unwrap_or_else(|| serde_json::Value::Null),
                    "error": {
                        "code": -32603,
                        "message": format!("Internal error: {}", e)
                    }
                });
                let error_json = serde_json::to_string(&error_response)?;
                writer.write_all(error_json.as_bytes()).await?;
                writer.write_all(b"\n").await?;
                writer.flush().await?;
            }
        }
    }

    Ok(())
}

async fn handle_message(
    db: &Arc<dyn db::DatabaseAdapter>,
    message: &str,
) -> anyhow::Result<Option<serde_json::Value>> {
    let json: serde_json::Value = serde_json::from_str(message)?;

    let method = json.get("method").and_then(|m| m.as_str());
    let id = json.get("id");

    match method {
        Some("initialize") => handle_initialize(id).map(Some),
        Some("initialized") => {
            info!("Client initialized");
            Ok(None)
        }
        Some("tools/list") => handle_tools_list(id).map(Some),
        Some("tools/call") => handle_tools_call(db, json.clone(), id).await.map(Some),
        Some("shutdown") => {
            info!("Shutdown requested");
            Ok(None)
        }
        Some(m) => {
            tracing::warn!("Unknown method: {}", m);
            Ok(None)
        }
        None => Ok(None),
    }
}

fn handle_initialize(id: Option<&serde_json::Value>) -> anyhow::Result<serde_json::Value> {
    Ok(serde_json::json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": {
            "protocolVersion": "2025-06-18",
            "capabilities": {
                "tools": {
                    "listChanged": false
                }
            },
            "serverInfo": {
                "name": "sqlite-mcp-rs",
                "version": "0.1.0"
            }
        }
    }))
}

fn handle_tools_list(id: Option<&serde_json::Value>) -> anyhow::Result<serde_json::Value> {
    Ok(serde_json::json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": {
            "tools": [
                {
                    "name": "list_tables",
                    "title": "List Tables",
                    "description": "List all tables in the database",
                    "inputSchema": {
                        "type": "object",
                        "properties": {}
                    }
                },
                {
                    "name": "get_table_schema",
                    "title": "Get Table Schema",
                    "description": "Get the schema of a specific table",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "table": {
                                "type": "string",
                                "description": "Table name"
                            }
                        },
                        "required": ["table"]
                    }
                },
                {
                    "name": "query_records",
                    "title": "Query Records",
                    "description": "Query records from a table with optional filters",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "table": {
                                "type": "string",
                                "description": "Table name"
                            },
                            "filters": {
                                "type": "object",
                                "description": "Filter conditions. Multiple columns are combined with AND. Supported operators: $eq, $ne, $gt, $gte, $lt, $lte, $in, $like",
                                "additionalProperties": {
                                    "type": "object",
                                    "properties": {
                                        "$eq": {
                                            "description": "Equals"
                                        },
                                        "$ne": {
                                            "description": "Not equals"
                                        },
                                        "$gt": {
                                            "description": "Greater than"
                                        },
                                        "$gte": {
                                            "description": "Greater than or equal"
                                        },
                                        "$lt": {
                                            "description": "Less than"
                                        },
                                        "$lte": {
                                            "description": "Less than or equal"
                                        },
                                        "$in": {
                                            "type": "array",
                                            "description": "Value in list"
                                        },
                                        "$like": {
                                            "type": "string",
                                            "description": "Pattern matching (use % for wildcard)"
                                        }
                                    }
                                }
                            },
                            "limit": {
                                "type": "integer",
                                "description": "Maximum number of results (optional)"
                            },
                            "offset": {
                                "type": "integer",
                                "description": "Offset for pagination (optional)"
                            }
                        },
                        "required": ["table"]
                    }
                },
                {
                    "name": "insert_record",
                    "title": "Insert Record",
                    "description": "Insert a new record into a table",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "table": {
                                "type": "string",
                                "description": "Table name"
                            },
                            "data": {
                                "type": "object",
                                "description": "Record data as JSON object"
                            }
                        },
                        "required": ["table", "data"]
                    }
                },
                {
                    "name": "update_record",
                    "title": "Update Record",
                    "description": "Update an existing record by ID",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "table": {
                                "type": "string",
                                "description": "Table name"
                            },
                            "id": {
                                "type": "integer",
                                "description": "Record ID"
                            },
                            "data": {
                                "type": "object",
                                "description": "Updated data as JSON object"
                            }
                        },
                        "required": ["table", "id", "data"]
                    }
                },
                {
                    "name": "delete_record",
                    "title": "Delete Record",
                    "description": "Delete a record by ID",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "table": {
                                "type": "string",
                                "description": "Table name"
                            },
                            "id": {
                                "type": "integer",
                                "description": "Record ID"
                            }
                        },
                        "required": ["table", "id"]
                    }
                },
                {
                    "name": "batch_insert",
                    "title": "Batch Insert",
                    "description": "Insert multiple records (max 100 items)",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "table": {
                                "type": "string",
                                "description": "Table name"
                            },
                            "items": {
                                "type": "array",
                                "description": "Array of records to insert",
                                "items": {
                                    "type": "object"
                                },
                                "maxItems": 100
                            },
                            "batch_size": {
                                "type": "integer",
                                "description": "Number of records per transaction (default: 50)",
                                "default": 50
                            }
                        },
                        "required": ["table", "items"]
                    }
                },
                {
                    "name": "batch_update",
                    "title": "Batch Update",
                    "description": "Update multiple records (max 100 items)",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "table": {
                                "type": "string",
                                "description": "Table name"
                            },
                            "updates": {
                                "type": "array",
                                "description": "Array of {id, data} objects",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "id": {"type": "integer"},
                                        "data": {"type": "object"}
                                    },
                                    "required": ["id", "data"]
                                },
                                "maxItems": 100
                            },
                            "batch_size": {
                                "type": "integer",
                                "description": "Number of records per transaction (default: 50)",
                                "default": 50
                            }
                        },
                        "required": ["table", "updates"]
                    }
                },
                {
                    "name": "batch_delete",
                    "title": "Batch Delete",
                    "description": "Delete multiple records by IDs",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "table": {
                                "type": "string",
                                "description": "Table name"
                            },
                            "ids": {
                                "type": "array",
                                "description": "Array of record IDs to delete",
                                "items": {
                                    "type": "integer"
                                }
                            }
                        },
                        "required": ["table", "ids"]
                    }
                },
                {
                    "name": "set_table_comment",
                    "title": "Set Table Comment",
                    "description": "Set or update the description of a table",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "table": {
                                "type": "string",
                                "description": "Table name"
                            },
                            "desc": {
                                "type": "string",
                                "description": "Table description"
                            }
                        },
                        "required": ["table", "desc"]
                    }
                },
                {
                    "name": "set_column_comment",
                    "title": "Set Column Comment",
                    "description": "Set or update the description of a column",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "table": {
                                "type": "string",
                                "description": "Table name"
                            },
                            "column": {
                                "type": "string",
                                "description": "Column name"
                            },
                            "desc": {
                                "type": "string",
                                "description": "Column description"
                            }
                        },
                        "required": ["table", "column", "desc"]
                    }
                }
            ]
        }
    }))
}

async fn handle_tools_call(
    db: &Arc<dyn db::DatabaseAdapter>,
    json: serde_json::Value,
    id: Option<&serde_json::Value>,
) -> anyhow::Result<serde_json::Value> {
    let params = json.get("params");
    let tool_name = params
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .ok_or_else(|| anyhow!("Missing tool name"))?;

    let empty_args = serde_json::json!({});
    let arguments = params.and_then(|p| p.get("arguments")).unwrap_or(&empty_args);

    let result = match tool_name {
        "list_tables" => tools::list_tables::execute(db, arguments).await?,
        "get_table_schema" => tools::get_schema::execute(db, arguments).await?,
        "query_records" => tools::query::execute(db, arguments).await?,
        "insert_record" => tools::insert::execute(db, arguments).await?,
        "update_record" => tools::update::execute(db, arguments).await?,
        "delete_record" => tools::delete::execute(db, arguments).await?,
        "batch_insert" => tools::batch::insert_execute(db, arguments).await?,
        "batch_update" => tools::batch::update_execute(db, arguments).await?,
        "batch_delete" => tools::batch::delete_execute(db, arguments).await?,
        "set_table_comment" => tools::set_table_comment::execute(db, arguments).await?,
        "set_column_comment" => tools::set_column_comment::execute(db, arguments).await?,
        _ => return Err(anyhow!("Unknown tool: {}", tool_name)),
    };

    Ok(serde_json::json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": {
            "content": [
                {
                    "type": "text",
                    "text": result
                }
            ]
        }
    }))
}
