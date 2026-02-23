use anyhow::anyhow;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tracing::{debug, info};

use crate::db::adapter::{DatabaseAdapter, BatchResult, ColumnInfo, FilterOperators, FilterValue, QueryFilter, TableSchema};

pub struct SqliteDatabase {
    conn: Arc<Mutex<rusqlite::Connection>>,
    readonly: bool,
}

impl SqliteDatabase {
    pub fn new(path: &Path, readonly: bool) -> anyhow::Result<Self> {
        let conn = if readonly {
            rusqlite::Connection::open_with_flags(path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)?
        } else {
            rusqlite::Connection::open(path)?
        };

        // Enable foreign keys
        conn.execute("PRAGMA foreign_keys = ON", [])?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
            readonly,
        })
    }


}

fn json_value_to_sqlite(value: &serde_json::Value) -> rusqlite::types::Value {
    match value {
        serde_json::Value::Null => rusqlite::types::Value::Null,
        serde_json::Value::Bool(b) => rusqlite::types::Value::Integer(if *b { 1 } else { 0 }),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                rusqlite::types::Value::Integer(i)
            } else if let Some(f) = n.as_f64() {
                rusqlite::types::Value::Real(f)
            } else {
                rusqlite::types::Value::Text(n.to_string())
            }
        }
        serde_json::Value::String(s) => rusqlite::types::Value::Text(s.clone()),
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
            rusqlite::types::Value::Text(value.to_string())
        }
    }
}

fn sqlite_row_to_json(row: &rusqlite::Row) -> anyhow::Result<serde_json::Value> {
    let mut map = serde_json::Map::new();
    
    for i in 0..row.as_ref().column_count() {
        let name = row.as_ref().column_name(i).unwrap_or("unknown");
        
        // Try to get value as String first
        let value = if let Ok(Some(text)) = row.get::<_, Option<String>>(i) {
            // Try to parse as JSON, fallback to string
            serde_json::from_str(&text).unwrap_or(serde_json::Value::String(text))
        } else if let Ok(Some(int)) = row.get::<_, Option<i64>>(i) {
            serde_json::Value::Number(int.into())
        } else if let Ok(Some(float)) = row.get::<_, Option<f64>>(i) {
            serde_json::Number::from_f64(float)
                .map(|n| serde_json::Value::Number(n))
                .unwrap_or(serde_json::Value::Null)
        } else if let Ok(Some(bool)) = row.get::<_, Option<bool>>(i) {
            serde_json::Value::Bool(bool)
        } else {
            serde_json::Value::Null
        };
        
        map.insert(name.to_string(), value);
    }
    
    Ok(serde_json::Value::Object(map))
}

fn build_where_clause(filter: &QueryFilter) -> anyhow::Result<(String, Vec<rusqlite::types::Value>)> {
    let mut conditions = Vec::new();
    let mut params = Vec::new();
    
    for (column, filter_value) in &filter.conditions {
        match filter_value {
            FilterValue::Direct(val) => {
                conditions.push(format!("{} = ?", column));
                params.push(json_value_to_sqlite(val));
            }
            FilterValue::Operator(ops) => {
                let (column_conds, mut param_vec) = build_operator_conditions(column, ops)?;
                conditions.push(column_conds);
                params.append(&mut param_vec);
            }
        }
    }
    
    if conditions.is_empty() {
        Ok(("1=1".to_string(), params))
    } else {
        Ok((format!("({})", conditions.join(") AND (")), params))
    }
}

fn build_operator_conditions(
    column: &str,
    ops: &FilterOperators,
) -> anyhow::Result<(String, Vec<rusqlite::types::Value>)> {
    let mut conditions = Vec::new();
    let mut params = Vec::new();
    
    if let Some(val) = &ops.eq {
        conditions.push(format!("{} = ?", column));
        params.push(json_value_to_sqlite(val));
    }
    if let Some(val) = &ops.ne {
        conditions.push(format!("{} != ?", column));
        params.push(json_value_to_sqlite(val));
    }
    if let Some(val) = &ops.gt {
        conditions.push(format!("{} > ?", column));
        params.push(json_value_to_sqlite(val));
    }
    if let Some(val) = &ops.gte {
        conditions.push(format!("{} >= ?", column));
        params.push(json_value_to_sqlite(val));
    }
    if let Some(val) = &ops.lt {
        conditions.push(format!("{} < ?", column));
        params.push(json_value_to_sqlite(val));
    }
    if let Some(val) = &ops.lte {
        conditions.push(format!("{} <= ?", column));
        params.push(json_value_to_sqlite(val));
    }
    if let Some(vals) = &ops.in_list {
        let placeholders: Vec<String> = vals.iter().map(|_| "?".to_string()).collect();
        conditions.push(format!("{} IN ({})", column, placeholders.join(", ")));
        for val in vals {
            params.push(json_value_to_sqlite(val));
        }
    }
    if let Some(pattern) = &ops.like {
        conditions.push(format!("{} LIKE ?", column));
        params.push(json_value_to_sqlite(&serde_json::Value::String(pattern.clone())));
    }
    
    if conditions.is_empty() {
        Ok(("1=1".to_string(), params))
    } else {
        Ok((format!("({})", conditions.join(") OR (")), params))
    }
}

#[async_trait::async_trait]
impl DatabaseAdapter for SqliteDatabase {
    async fn list_tables(&self) -> anyhow::Result<Vec<String>> {
        debug!("Listing tables in database");
        let conn = self.conn.lock().map_err(|e| anyhow!("Failed to lock connection: {}", e))?;
        
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name")?;
        
        let mut rows = stmt.query([])?;
        let mut tables = Vec::new();
        while let Some(row) = rows.next()? {
            tables.push(row.get::<_, String>(0)?);
        }
        
        debug!("Found {} tables", tables.len());
        Ok(tables)
    }

    async fn get_schema(&self, table: &str) -> anyhow::Result<TableSchema> {
        debug!("Getting schema for table '{}'", table);
        let conn = self.conn.lock().map_err(|e| anyhow!("Failed to lock connection: {}", e))?;
        
        let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", table))?;
        
        let mut rows = stmt.query([])?;
        let mut columns = Vec::new();
        let mut primary_keys = Vec::new();
        
        while let Some(row) = rows.next()? {
            let is_pk: i32 = row.get(5)?;
            if is_pk > 0 {
                let name: String = row.get(1)?;
                primary_keys.push(name.clone());
            }
            
            columns.push(ColumnInfo {
                name: row.get::<_, String>(1)?,
                data_type: row.get::<_, Option<String>>(2)?.unwrap_or_else(|| "ANY".to_string()),
                not_null: row.get::<_, i32>(3)? == 1,
                default_value: row.get::<_, Option<String>>(4)?,
                is_primary_key: is_pk > 0,
            });
        }
        
        let primary_key = if primary_keys.len() == 1 {
            primary_keys[0].clone()
        } else if primary_keys.is_empty() {
            "rowid".to_string()
        } else {
            return Err(anyhow!("Composite primary keys not supported"));
        };
        
        Ok(TableSchema {
            name: table.to_string(),
            columns,
            primary_key: Some(primary_key),
        })
    }

    async fn select(
        &self,
        table: &str,
        filters: Option<QueryFilter>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> anyhow::Result<Vec<serde_json::Value>> {
        debug!("Querying table '{}' with filters: {:?}", table, filters);
        let conn = self.conn.lock().map_err(|e| anyhow!("Failed to lock connection: {}", e))?;
        
        let mut sql = format!("SELECT * FROM {}", table);
        let mut params = Vec::new();
        
        if let Some(filter) = filters {
            let (where_clause, where_params) = build_where_clause(&filter)?;
            sql.push_str(&format!(" WHERE {}", where_clause));
            params = where_params;
        }
        
        if let Some(limit) = limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }
        if let Some(offset) = offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }
        
        debug!("Executing SQL: {}", sql);
        debug!("Params: {:?}", params);
        
        let mut stmt = conn.prepare(&sql)?;
        let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p as &dyn rusqlite::ToSql).collect();
        
        let mut rows = stmt.query(param_refs.as_slice())?;
        let mut results = Vec::new();
        while let Some(row) = rows.next()? {
            results.push(sqlite_row_to_json(row)?);
        }
        
        debug!("Query returned {} rows", results.len());
        Ok(results)
    }

    async fn insert(&self, table: &str, data: serde_json::Value) -> anyhow::Result<i64> {
        if self.readonly {
            return Err(anyhow!("Cannot insert in read-only mode"));
        }
        
        let conn = self.conn.lock().map_err(|e| anyhow!("Failed to lock connection: {}", e))?;
        
        if let serde_json::Value::Object(obj) = data {
            let columns: Vec<&String> = obj.keys().collect();
            let placeholders: Vec<&str> = columns.iter().map(|_| "?").collect();
            let sql = format!(
                "INSERT INTO {} ({}) VALUES ({})",
                table,
                columns.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "),
                placeholders.join(", ")
            );
            
            let mut params = Vec::new();
            for val in obj.values() {
                params.push(json_value_to_sqlite(val));
            }
            
            debug!("Inserting into table '{}'", table);
            debug!("SQL: {}", sql);
            
            let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p as &dyn rusqlite::ToSql).collect();
            conn.execute(&sql, param_refs.as_slice())?;
            let id = conn.last_insert_rowid();
            debug!("Inserted row with ID: {}", id);
            
            Ok(id)
        } else {
            Err(anyhow!("Data must be a JSON object"))
        }
    }

    async fn update(
        &self,
        table: &str,
        pk_value: i64,
        data: serde_json::Value,
    ) -> anyhow::Result<usize> {
        if self.readonly {
            return Err(anyhow!("Cannot update in read-only mode"));
        }
        
        let conn = self.conn.lock().map_err(|e| anyhow!("Failed to lock connection: {}", e))?;
        
        // Get primary key column
        let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", table))?;
        let mut rows = stmt.query([])?;
        let mut primary_keys = Vec::new();
        
        while let Some(row) = rows.next()? {
            let is_pk: i32 = row.get(5)?;
            if is_pk > 0 {
                let name: String = row.get(1)?;
                primary_keys.push(name);
            }
        }
        
        let pk_column = if primary_keys.len() == 1 {
            primary_keys[0].clone()
        } else {
            "rowid".to_string()
        };
        
        if let serde_json::Value::Object(obj) = data {
            let sets: Vec<String> = obj.keys().map(|k| format!("{} = ?", k)).collect();
            let sql = format!(
                "UPDATE {} SET {} WHERE {} = ?",
                table,
                sets.join(", "),
                pk_column
            );
            
            let mut params = Vec::new();
            for val in obj.values() {
                params.push(json_value_to_sqlite(val));
            }
            params.push(rusqlite::types::Value::Integer(pk_value));
            
            debug!(
                "Updating table '{}' where {} = {}",
                table, pk_column, pk_value
            );
            debug!("SQL: {}", sql);
            
            let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p as &dyn rusqlite::ToSql).collect();
            let affected = conn.execute(&sql, param_refs.as_slice())?;
            debug!("Updated {} rows", affected);
            
            Ok(affected)
        } else {
            Err(anyhow!("Data must be a JSON object"))
        }
    }

    async fn delete(&self, table: &str, pk_value: i64) -> anyhow::Result<usize> {
        if self.readonly {
            return Err(anyhow!("Cannot delete in read-only mode"));
        }
        
        let conn = self.conn.lock().map_err(|e| anyhow!("Failed to lock connection: {}", e))?;
        
        // Get primary key column
        let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", table))?;
        let mut rows = stmt.query([])?;
        let mut primary_keys = Vec::new();
        
        while let Some(row) = rows.next()? {
            let is_pk: i32 = row.get(5)?;
            if is_pk > 0 {
                let name: String = row.get(1)?;
                primary_keys.push(name);
            }
        }
        
        let pk_column = if primary_keys.len() == 1 {
            primary_keys[0].clone()
        } else {
            "rowid".to_string()
        };
        
        let sql = format!("DELETE FROM {} WHERE {} = ?", table, pk_column);
        debug!("Deleting from table '{}' where {} = {}", table, pk_column, pk_value);
        debug!("SQL: {}", sql);
        
        let pk_param = rusqlite::types::Value::Integer(pk_value);
        let param_refs: Vec<&dyn rusqlite::ToSql> = vec![&pk_param];
        let affected = conn.execute(&sql, param_refs.as_slice())?;
        debug!("Deleted {} rows", affected);
        
        Ok(affected)
    }

    async fn batch_insert(
        &self,
        table: &str,
        items: Vec<serde_json::Value>,
        batch_size: usize,
    ) -> anyhow::Result<BatchResult> {
        if self.readonly {
            return Err(anyhow!("Cannot insert in read-only mode"));
        }
        
        if items.is_empty() {
            return Ok(BatchResult {
                total: 0,
                succeeded: 0,
                failed: 0,
                errors: Vec::new(),
                inserted_ids: None,
            });
        }
        
        let total = items.len();
        let mut succeeded = 0;
        let mut failed = 0;
        let mut errors = Vec::new();
        let mut inserted_ids = Vec::new();
        
        info!(
            "Starting batch insert of {} items into table '{}' with batch size {}",
            total, table, batch_size
        );
        
        for batch in items.chunks(batch_size) {
            let conn = self.conn.lock().map_err(|e| anyhow!("Failed to lock connection: {}", e))?;
            let tx = conn.unchecked_transaction()?;
            let batch_succeeded = batch.len();
            let mut batch_errors = Vec::new();
            
            for item in batch {
                if let serde_json::Value::Object(obj) = item {
                    let columns: Vec<&String> = obj.keys().collect();
                    let placeholders: Vec<&str> = columns.iter().map(|_| "?").collect();
                    let sql = format!(
                        "INSERT INTO {} ({}) VALUES ({})",
                        table,
                        columns.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "),
                        placeholders.join(", ")
                    );
                    
                    let mut params = Vec::new();
                    for val in obj.values() {
                        params.push(json_value_to_sqlite(val));
                    }
                    
                    let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p as &dyn rusqlite::ToSql).collect();
                    match tx.execute(&sql, param_refs.as_slice()) {
                        Ok(_) => {
                            inserted_ids.push(tx.last_insert_rowid());
                        }
                        Err(e) => {
                            batch_errors.push(format!("Insert failed: {}", e));
                            failed += 1;
                        }
                    }
                }
            }
            
            if batch_errors.is_empty() {
                tx.commit()?;
                succeeded += batch_succeeded;
            } else {
                tx.rollback()?;
                errors.extend(batch_errors);
            }
        }
        
        info!(
            "Batch insert completed: {} succeeded, {} failed",
            succeeded, failed
        );
        
        Ok(BatchResult {
            total,
            succeeded,
            failed,
            errors,
            inserted_ids: Some(inserted_ids),
        })
    }

    async fn batch_update(
        &self,
        table: &str,
        updates: Vec<(i64, serde_json::Value)>,
        batch_size: usize,
    ) -> anyhow::Result<BatchResult> {
        if self.readonly {
            return Err(anyhow!("Cannot update in read-only mode"));
        }
        
        if updates.is_empty() {
            return Ok(BatchResult {
                total: 0,
                succeeded: 0,
                failed: 0,
                errors: Vec::new(),
                inserted_ids: None,
            });
        }
        
        let total = updates.len();
        let mut succeeded = 0;
        let mut failed = 0;
        let mut errors = Vec::new();
        
        info!(
            "Starting batch update of {} items in table '{}' with batch size {}",
            total, table, batch_size
        );
        
        // Get primary key column first
        let pk_column = {
            let conn = self.conn.lock().map_err(|e| anyhow!("Failed to lock connection: {}", e))?;
            let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", table))?;
            let mut rows = stmt.query([])?;
            let mut primary_keys = Vec::new();
            
            while let Some(row) = rows.next()? {
                let is_pk: i32 = row.get(5)?;
                if is_pk > 0 {
                    let name: String = row.get(1)?;
                    primary_keys.push(name);
                }
            }
            
            if primary_keys.len() == 1 {
                primary_keys[0].clone()
            } else {
                "rowid".to_string()
            }
        }; // conn is dropped here, releasing the lock
        
        for batch in updates.chunks(batch_size) {
            let conn = self.conn.lock().map_err(|e| anyhow!("Failed to lock connection: {}", e))?;
            let tx = conn.unchecked_transaction()?;
            let batch_succeeded = batch.len();
            let mut batch_errors = Vec::new();
            
            for (pk_value, data) in batch {
                if let serde_json::Value::Object(obj) = data {
                    let sets: Vec<String> = obj.keys().map(|k| format!("{} = ?", k)).collect();
                    let sql = format!(
                        "UPDATE {} SET {} WHERE {} = ?",
                        table,
                        sets.join(", "),
                        pk_column
                    );
                    
                    let mut params = Vec::new();
                    for val in obj.values() {
                        params.push(json_value_to_sqlite(val));
                    }
                    params.push(rusqlite::types::Value::Integer(*pk_value));
                    
                    let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p as &dyn rusqlite::ToSql).collect();
                    match tx.execute(&sql, param_refs.as_slice()) {
                        Ok(_) => {}
                        Err(e) => {
                            batch_errors.push(format!("Update failed for ID {}: {}", pk_value, e));
                            failed += 1;
                        }
                    }
                }
            }
            
            if batch_errors.is_empty() {
                tx.commit()?;
                succeeded += batch_succeeded;
            } else {
                tx.rollback()?;
                errors.extend(batch_errors);
            }
        }
        
        info!(
            "Batch update completed: {} succeeded, {} failed",
            succeeded, failed
        );
        
        Ok(BatchResult {
            total,
            succeeded,
            failed,
            errors,
            inserted_ids: None,
        })
    }

    async fn batch_delete(&self, table: &str, ids: Vec<i64>) -> anyhow::Result<usize> {
        if self.readonly {
            return Err(anyhow!("Cannot delete in read-only mode"));
        }
        
        if ids.is_empty() {
            return Ok(0);
        }
        
        info!(
            "Starting batch delete of {} items in table '{}'",
            ids.len(), table
        );
        
        // Get primary key column first
        let pk_column = {
            let conn = self.conn.lock().map_err(|e| anyhow!("Failed to lock connection: {}", e))?;
            let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", table))?;
            let mut rows = stmt.query([])?;
            let mut primary_keys = Vec::new();
            
            while let Some(row) = rows.next()? {
                let is_pk: i32 = row.get(5)?;
                if is_pk > 0 {
                    let name: String = row.get(1)?;
                    primary_keys.push(name);
                }
            }
            
            if primary_keys.len() == 1 {
                primary_keys[0].clone()
            } else {
                "rowid".to_string()
            }
        }; // conn is dropped here, releasing the lock
        
        let placeholders: Vec<String> = ids.iter().map(|_| "?".to_string()).collect();
        let sql = format!(
            "DELETE FROM {} WHERE {} IN ({})",
            table,
            pk_column,
            placeholders.join(", ")
        );
        
        debug!("Batch delete SQL: {}", sql);
        
        let params: Vec<rusqlite::types::Value> = ids
            .iter()
            .map(|id| rusqlite::types::Value::Integer(*id))
            .collect();
        
        let conn = self.conn.lock().map_err(|e| anyhow!("Failed to lock connection: {}", e))?;
        let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p as &dyn rusqlite::ToSql).collect();
        let affected = conn.execute(&sql, param_refs.as_slice())?;
        info!("Batch deleted {} rows", affected);
        
        Ok(affected)
    }

    async fn is_readonly(&self) -> bool {
        self.readonly
    }
}