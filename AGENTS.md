# AGENTS.md - SQLite MCP 服务器开发指南

这是一个使用 Rust 实现的 SQLite MCP (Model Context Protocol) 服务器，提供基于 JSON 的数据库操作接口。

## 构建和测试命令

### 核心命令
```bash
# 检查代码
cargo check

# 构建 debug 版本
cargo build

# 构建 release 版本  
cargo build --release

# 运行测试
cargo test

# 运行可执行文件
cargo run -- --db-path test.db

# 只读模式运行
cargo run -- --db-path test.db --readonly
```

### 测试数据
```bash
# 创建测试数据库（需要 sqlite3 命令）
./test.sh

# 或者手动执行 SQL
sqlite3 test.db < test.sql
```

## 项目结构

```
src/
├── main.rs              # 主入口和 MCP 协议处理
├── db/                  # 数据库抽象层
│   ├── mod.rs          # 模块声明
│   ├── adapter.rs      # DatabaseAdapter trait 定义
│   └── sqlite.rs       # SQLite 具体实现（包含表/列注释功能）
└── tools/               # MCP 工具实现
    ├── mod.rs          # 模块声明
    ├── list_tables.rs  # 列表表工具
    ├── get_schema.rs   # 获取表结构工具（返回包含注释的 schema）
    ├── query.rs        # 查询记录工具
    ├── insert.rs       # 插入记录工具
    ├── update.rs       # 更新记录工具
    ├── delete.rs       # 删除记录工具
    └── batch.rs        # 批量操作工具

辅助表（首次使用时自动创建）：
- _table_comment        # 存储表描述信息
- _table_column_comment # 存储列描述信息
```

## 代码风格指南

### 1. 导入规范
```rust
// 标准库导入在前
use std::path::Path;
use std::sync::{Arc, Mutex};

// 第三方库导入在中间
use anyhow::anyhow;
use serde::{Deserialize, Serialize};

// 本地模块导入在后
use crate::db::adapter::{DatabaseAdapter, TableSchema};
```

### 2. 错误处理
- 统一使用 `anyhow::Result<T>` 作为返回类型
- 使用 `anyhow!` 宏创建错误
- 数据库操作错误要包含上下文信息

```rust
// 好的示例
let conn = self.conn.lock()
    .map_err(|e| anyhow!("Failed to lock connection: {}", e))?;

// 避免直接使用 unwrap()，除非确定不会失败
```

### 3. 命名约定
- **结构体**: PascalCase (例如: `SqliteDatabase`, `TableSchema`)
- **函数/方法**: snake_case (例如: `list_tables`, `get_schema`)
- **常量**: SCREAMING_SNAKE_CASE
- **模块**: snake_case，保持与文件名一致

### 4. 类型设计
```rust
// 使用 serde 序列化/反序列化
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct QueryRecordsInput {
    pub table: String,
    #[serde(default)]
    pub filters: Option<QueryFilter>,
}

// 使用 async_trait 定义异步 trait
#[async_trait::async_trait]
pub trait DatabaseAdapter: Send + Sync {
    async fn list_tables(&self) -> Result<Vec<String>>;
}
```

### 5. 日志记录
```rust
// 使用 tracing 进行日志记录
use tracing::{debug, info, warn, error};

debug!("Querying table '{}' with filters: {:?}", table, filters);
info!("Starting batch insert of {} items", total);
warn!("Unknown method: {}", method);
error!("Error handling message: {}", e);
```

### 6. 异步编程
```rust
// 所有数据库操作都应该是异步的
#[async_trait::async_trait]
impl DatabaseAdapter for SqliteDatabase {
    async fn select(&self, table: &str, ...) -> Result<Vec<serde_json::Value>> {
        // 使用 tokio::spawn 或类似异步运行时
        let conn = self.conn.lock().map_err(|e| anyhow!("Failed to lock connection: {}", e))?;
        // ... 同步数据库操作
    }
}
```

### 7. JSON 处理
```rust
// 使用 serde_json 进行 JSON 处理
use serde_json::{json, Map, Value};

// 优先使用 from_value 和 to_value
let input: QueryRecordsInput = serde_json::from_value(args.clone())?;
let output = serde_json::to_string_pretty(&output)?;

// 手动构建 JSON 时使用 json! 宏
let response = json!({
    "jsonrpc": "2.0",
    "id": id,
    "result": {
        "content": [{"type": "text", "text": result}]
    }
});
```

## MCP 协议规范

### 1. 标准响应格式
```rust
// 成功响应
Ok(json!({
    "jsonrpc": "2.0",
    "id": id,
    "result": result_data
}))

// 错误响应  
json!({
    "jsonrpc": "2.0", 
    "id": serde_json::Value::Null,
    "error": {
        "code": -32603,
        "message": format!("Internal error: {}", e)
    }
})
```

### 2. 工具定义
```rust
// 在 main.rs 的 handle_tools_list 中定义
{
    "name": "tool_name",
    "title": "Human Readable Name", 
    "description": "Tool description",
    "inputSchema": {
        "type": "object",
        "properties": {
            "param1": {
                "type": "string",
                "description": "Parameter description"
            }
        },
        "required": ["param1"]
    }
}
```

### 3. 工具实现模式
```rust
// tools/tool_name.rs 标准模式
use crate::db::DatabaseAdapter;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ToolInput {
    pub required_param: String,
    #[serde(default)]
    pub optional_param: Option<Type>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]  
pub struct ToolOutput {
    pub result_field: Type,
}

pub async fn execute(
    db: &std::sync::Arc<dyn DatabaseAdapter>,
    args: &serde_json::Value,
) -> anyhow::Result<String> {
    let input: ToolInput = serde_json::from_value(args.clone())?;
    // 业务逻辑
    let output = ToolOutput { result_field };
    serde_json::to_string_pretty(&output).map_err(anyhow::Error::from)
}
```

## 数据库操作最佳实践

### 1. SQL 构建安全
```rust
// 使用参数化查询，避免 SQL 注入
let sql = format!("SELECT * FROM {} WHERE {} = ?", table, pk_column);
let params = vec![rusqlite::types::Value::Integer(pk_value)];
```

### 2. 事务处理
```rust
// 批量操作使用事务
let tx = conn.unchecked_transaction()?;
// 执行操作
if errors.is_empty() {
    tx.commit()?;
} else {
    tx.rollback()?;
}
```

### 3. 连接管理
```rust
// 使用 Arc<Mutex<Connection>> 管理连接
let conn = self.conn.lock()
    .map_err(|e| anyhow!("Failed to lock connection: {}", e))?;
```

## 扩展新功能

### 1. 添加新工具
1. 在 `src/tools/` 创建新文件
2. 定义输入/输出结构体
3. 实现 `execute` 函数
4. 在 `tools/mod.rs` 中导出
5. 在 `main.rs` 的 `handle_tools_call` 中添加匹配分支
6. 在 `handle_tools_list` 中定义工具模式

### 2. 添加新数据库支持
1. 实现 `DatabaseAdapter` trait
2. 在 `db/mod.rs` 中添加模块
3. 在 `main.rs` 中添加实例化逻辑

## 性能注意事项

1. **批处理操作** - 使用事务提高性能
2. **连接池** - 当前使用单一连接，未来可考虑连接池
3. **内存使用** - 大结果集考虑分页处理
4. **日志级别** - 生产环境调整日志级别避免性能影响

## 安全注意事项

1. **SQL 注入** - 始终使用参数化查询
2. **输入验证** - 使用 serde 进行 JSON 反序列化验证
3. **权限控制** - 只读模式限制写操作
4. **错误信息** - 避免在错误信息中泄露敏感数据

## 调试技巧

```rust
// 启用详细日志
RUST_LOG=debug cargo run -- --db-path test.db

// SQL 查询调试
debug!("SQL: {}", sql);
debug!("Params: {:?}", params);

// JSON 调试  
debug!("Request: {}", serde_json::to_string_pretty(&json)?);
```

## 测试策略

目前项目没有单元测试，建议添加：

1. **单元测试** - 测试核心数据库操作
2. **集成测试** - 测试 MCP 协议交互
3. **性能测试** - 测试批量操作性能

## 部署和发布

```bash
# 构建生产版本
cargo build --release

# 可执行文件位置
# Linux/macOS: target/release/sqlite-mcp-rs  
# Windows: target/release/sqlite-mcp-rs.exe

# Claude Desktop 配置示例
{
  "mcpServers": {
    "sqlite": {
      "command": "/path/to/sqlite-mcp-rs",
      "args": ["--db-path", "/path/to/database.db"]
    }
  }
}
```