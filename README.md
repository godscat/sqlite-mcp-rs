# SQLite MCP Server

一个使用 Rust 实现的快速 SQLite MCP 服务器，支持基于 JSON 的数据库操作。

## 特性

- ✅ **纯 Rust 实现**：无需 Node.js 依赖，快速且高效
- ✅ **JSON 交互**：无需编写 SQL，所有操作都通过 JSON 完成
- ✅ **完整 CRUD 操作**：支持增删改查
- ✅ **批量操作**：支持批量插入、更新、删除（最多 100 条）
- ✅ **动态主键检测**：自动检测主键（优先 `id`，fallback 到 `rowid`）
- ✅ **只读模式**：可选的只读模式，保护数据安全
- ✅ **可扩展架构**：数据库抽象层，便于未来扩展其他数据库

## 安装

```bash
# 克隆仓库
git clone <repository-url>
cd sqlite-mcp

# 构建 release 版本
cargo build --release

# 可执行文件位于 target/release/sqlite-mcp
```

## 使用

### 启动服务器

```bash
# 读写模式
sqlite-mcp --db-path /path/to/database.db

# 只读模式
sqlite-mcp --db-path /path/to/database.db --readonly
```

### 命令行参数

- `--db-path <PATH>`：SQLite 数据库文件路径（必需）
- `--readonly`：只读模式（可选，默认：false）

## MCP 工具

### 1. list_tables
列出数据库中的所有表。

**输入：**
```json
{}
```

**输出：**
```json
{
  "tables": ["users", "products", "orders"]
}
```

### 2. get_table_schema
获取指定表的结构。

**输入：**
```json
{
  "table": "users"
}
```

**输出：**
```json
{
  "name": "users",
  "columns": [
    {"name": "id", "data_type": "INTEGER", "not_null": true, "is_primary_key": true},
    {"name": "name", "data_type": "TEXT", "not_null": false, "is_primary_key": false}
  ],
  "primary_key": "id"
}
```

### 3. query_records
查询表中的记录，支持过滤和分页。

**输入：**
```json
{
  "table": "users",
  "filters": {
    "age": {"$gte": 18},
    "name": {"$like": "%张%"}
  },
  "limit": 10,
  "offset": 0
}
```

**过滤操作符：**
- `$eq`：等于
- `$ne`：不等于
- `$gt`：大于
- `$gte`：大于等于
- `$lt`：小于
- `$lte`：小于等于
- `$in`：在列表中
- `$like`：模糊匹配

**输出：**
```json
{
  "records": [
    {"id": 1, "name": "张三", "age": 25},
    {"id": 2, "name": "张四", "age": 30}
  ],
  "total": 2
}
```

### 4. insert_record
插入一条新记录（只读模式下拒绝）。

**输入：**
```json
{
  "table": "users",
  "data": {
    "name": "李五",
    "age": 28
  }
}
```

**输出：**
```json
{
  "id": 101,
  "affected_rows": 1
}
```

### 5. update_record
更新一条记录（只读模式下拒绝）。

**输入：**
```json
{
  "table": "users",
  "id": 1,
  "data": {
    "age": 26
  }
}
```

**输出：**
```json
{
  "affected_rows": 1
}
```

### 6. delete_record
删除一条记录（只读模式下拒绝）。

**输入：**
```json
{
  "table": "users",
  "id": 1
}
```

**输出：**
```json
{
  "affected_rows": 1
}
```

### 7. batch_insert
批量插入记录（最多 100 条）。

**输入：**
```json
{
  "table": "users",
  "items": [
    {"name": "用户1", "age": 20},
    {"name": "用户2", "age": 21},
    {"name": "用户3", "age": 22}
  ],
  "batch_size": 50
}
```

**输出：**
```json
{
  "total": 3,
  "succeeded": 3,
  "failed": 0,
  "errors": [],
  "inserted_ids": [101, 102, 103]
}
```

### 8. batch_update
批量更新记录（最多 100 条）。

**输入：**
```json
{
  "table": "users",
  "updates": [
    {"id": 1, "data": {"age": 30}},
    {"id": 2, "data": {"age": 31}}
  ],
  "batch_size": 50
}
```

**输出：**
```json
{
  "total": 2,
  "succeeded": 2,
  "failed": 0,
  "errors": []
}
```

### 9. batch_delete
批量删除记录。

**输入：**
```json
{
  "table": "users",
  "ids": [1, 2, 3]
}
```

**输出：**
```json
{
  "affected_rows": 3
}
```

## Filter Parameter Usage

`query_records` 工具的 `filters` 参数允许灵活的查询条件。

### 基本语法

```json
{
  "filters": {
    "column_name": {
      "$operator": "value"
    }
  }
}
```

### 支持的操作符

| 操作符 | 描述 | 示例 |
|--------|------|------|
| `$eq` | 等于 | `{"age": {"$eq": 25}}` |
| `$ne` | 不等于 | `{"status": {"$ne": "deleted"}}` |
| `$gt` | 大于 | `{"price": {"$gt": 100}}` |
| `$gte` | 大于等于 | `{"age": {"$gte": 18}}` |
| `$lt` | 小于 | `{"quantity": {"$lt": 10}}` |
| `$lte` | 小于等于 | `{"score": {"$lte": 90}}` |
| `$in` | 在列表中 | `{"category": {"$in": ["book", "movie"]}}` |
| `$like` | 模糊匹配（使用 % 通配符） | `{"name": {"$like": "%张%"}}` |

### 逻辑关系

- **多列条件**：使用 AND 逻辑（所有条件必须满足）
- **同列多操作符**：使用 OR 逻辑（任一条件满足即可）

### 使用示例

#### 1. 简单等于查询

查询年龄等于 25 的用户：

```json
{
  "table": "users",
  "filters": {
    "age": {"$eq": 25}
  }
}
```

#### 2. 范围查询

查询年龄在 18 到 30 之间的用户：

```json
{
  "table": "users",
  "filters": {
    "age": {"$gte": 18, "$lte": 30}
  }
}
```

#### 3. 模糊匹配

查询名字包含"张"的用户：

```json
{
  "table": "users",
  "filters": {
    "name": {"$like": "%张%"}
  }
}
```

#### 4. 多条件组合（AND 逻辑）

查询年龄大于等于 18 并且名字包含"张"的用户：

```json
{
  "table": "users",
  "filters": {
    "age": {"$gte": 18},
    "name": {"$like": "%张%"}
  }
}
```

**注意**：不同列的条件使用 AND 逻辑，表示所有条件都必须满足。

#### 5. 同列多操作符（OR 逻辑）

查询年龄小于 18 或大于 65 的用户：

```json
{
  "table": "users",
  "filters": {
    "age": {"$lt": 18, "$gt": 65}
  }
}
```

**注意**：同一列的多个操作符使用 OR 逻辑，表示任一条件满足即可。

#### 6. 列表匹配

查询分类为 book 或 movie 的产品：

```json
{
  "table": "products",
  "filters": {
    "category": {"$in": ["book", "movie"]}
  }
}
```

#### 7. 复杂组合

查询价格为 100 以上、分类为 book 且名称包含"教程"的产品：

```json
{
  "table": "products",
  "filters": {
    "price": {"$gte": 100},
    "category": {"$eq": "book"},
    "name": {"$like": "%教程%"}
  }
}
```

### 分页

结合 `limit` 和 `offset` 实现分页：

```json
{
  "table": "users",
  "filters": {
    "age": {"$gte": 18}
  },
  "limit": 10,
  "offset": 0
}
```

## Claude Desktop 集成

在 Claude Desktop 的配置文件中添加以下内容：

**Windows：**
```json
{
  "mcpServers": {
    "sqlite": {
      "command": "E:\\\\path\\\\to\\\\sqlite-mcp.exe",
      "args": [
        "--db-path",
        "C:\\\\path\\\\to\\\\database.db"
      ]
    }
  }
}
```

**Linux/macOS：**
```json
{
  "mcpServers": {
    "sqlite": {
      "command": "/path/to/sqlite-mcp",
      "args": [
        "--db-path",
        "/path/to/database.db"
      ]
    }
  }
}
```

只读模式示例：
```json
{
  "mcpServers": {
    "sqlite-readonly": {
      "command": "/path/to/sqlite-mcp",
      "args": [
        "--db-path",
        "/path/to/database.db",
        "--readonly"
      ]
    }
  }
}
```

配置文件位置：
- **macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Windows**: `%APPDATA%/Claude/claude_desktop_config.json`
- **Linux**: `~/.config/Claude/claude_desktop_config.json`

## 类型映射

SQLite 类型与 JSON 的自动映射：

| SQLite | JSON |
|--------|-------|
| INTEGER | number |
| REAL | number |
| TEXT | string |
| BLOB | string (base64 编码) |
| NULL | null |

## 扩展性

该项目使用了数据库抽象层，便于未来扩展支持其他数据库：

```rust
trait DatabaseAdapter: Send + Sync {
    fn list_tables(&self) -> Result<Vec<String>>;
    fn get_schema(&self, table: &str) -> Result<TableSchema>;
    fn select(&self, table: &str, filters: Option<QueryFilter>, limit: Option<usize>, offset: Option<usize>) -> Result<Vec<serde_json::Value>>;
    fn insert(&self, table: &str, data: serde_json::Value) -> Result<i64>;
    fn update(&self, table: &str, pk_column: &str, pk_value: i64, data: serde_json::Value) -> Result<usize>;
    fn delete(&self, table: &str, pk_column: &str, pk_value: i64) -> Result<usize>;
    fn batch_insert(&self, table: &str, items: Vec<serde_json::Value>, batch_size: usize) -> Result<BatchResult>;
    fn batch_update(&self, table: &str, updates: Vec<(i64, serde_json::Value)>, pk_column: &str, batch_size: usize) -> Result<BatchResult>;
    fn batch_delete(&self, table: &str, ids: Vec<i64>, pk_column: &str) -> Result<usize>;
    fn is_readonly(&self) -> bool;
}
```

要支持新的数据库类型（如 PostgreSQL、MySQL），只需实现 `DatabaseAdapter` trait 并更新 main.rs 中的实例化逻辑。

## 日志

默认日志级别为 DEBUG，记录所有 SQL 查询和参数。日志输出到 stderr。

## 开发

```bash
# 检查代码
cargo check

# 运行测试
cargo test

# 构建 debug 版本
cargo build
```

## 日志和调试

### 日志级别控制

服务器支持通过环境变量控制日志输出：

```bash
# 完全禁用日志（推荐用于生产环境）
RUST_LOG=off ./target/release/sqlite-mcp.exe --db-path database.db

# 只显示错误
RUST_LOG=error ./target/release/sqlite-mcp.exe --db-path database.db

# 显示信息和警告（默认）
RUST_LOG=info ./target/release/sqlite-mcp.exe --db-path database.db

# 显示所有调试信息
RUST_LOG=debug ./target/release/sqlite-mcp.exe --db-path database.db
```

### 输出重定向

- JSON 响应：输出到 stdout
- 日志信息：输出到 stderr

可以将日志重定向到文件：

```bash
./target/release/sqlite-mcp.exe --db-path database.db 2>> server.log
```

### 故障排除

如果遇到 JSON 解析错误，请确保：

1. 使用最新版本：`cargo build --release`
2. 在 Claude Desktop 配置中设置 `RUST_LOG=error`
3. 查看详细故障排除指南：[TROUBLESHOOTING.md](TROUBLESHOOTING.md)

## 许可证

MIT

## 贡献

欢迎提交 Issue 和 Pull Request！

## 致谢

- [rmcp](https://github.com/modelcontextprotocol/rust-sdk) - Rust MCP SDK
- [rusqlite](https://github.com/rusqlite/rusqlite) - Rust SQLite 绑定
