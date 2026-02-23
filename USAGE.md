# SQLite MCP Server 使用指南

## 项目概述

这是一个使用 Rust 实现的 SQLite MCP (Model Context Protocol) 服务器，提供基于 JSON 的数据库操作接口。

## 快速开始

### 1. 构建项目

```bash
# 调试版本
cargo build

# 发布版本
cargo build --release
```

### 2. 运行服务器

```bash
# 基本用法
cargo run -- --db-path your_database.db

# 只读模式
cargo run -- --db-path your_database.db --readonly

# 使用发布版本
./target/release/sqlite-mcp --db-path your_database.db
```

### 3. 与服务器交互

服务器通过标准输入/输出接收 JSON-RPC 2.0 格式的请求。

#### 初始化服务器
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "protocolVersion": "2025-06-18",
    "capabilities": {}
  }
}
```

#### 获取可用工具列表
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/list"
}
```

## 可用工具

### 1. list_tables
列出数据库中的所有表。

**请求:**
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "list_tables",
    "arguments": {}
  }
}
```

**响应:**
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"tables\":[\"users\",\"products\"]}"
      }
    ]
  }
}
```

### 2. get_table_schema
获取指定表的结构。

**请求:**
```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "method": "tools/call",
  "params": {
    "name": "get_table_schema",
    "arguments": {
      "table": "users"
    }
  }
}
```

### 3. query_records
查询表中的记录，支持过滤和分页。

**请求:**
```json
{
  "jsonrpc": "2.0",
  "id": 5,
  "method": "tools/call",
  "params": {
    "name": "query_records",
    "arguments": {
      "table": "users",
      "filters": {
        "age": {"$gt": 25}
      },
      "limit": 10,
      "offset": 0
    }
  }
}
```

**支持的过滤操作:**
- `$eq`: 等于
- `$ne`: 不等于
- `$gt`: 大于
- `$gte`: 大于等于
- `$lt`: 小于
- `$lte`: 小于等于
- `$in`: 在列表中
- `$like`: 模糊匹配

### 4. insert_record
插入新记录。

**请求:**
```json
{
  "jsonrpc": "2.0",
  "id": 6,
  "method": "tools/call",
  "params": {
    "name": "insert_record",
    "arguments": {
      "table": "users",
      "data": {
        "name": "Alice",
        "email": "alice@example.com",
        "age": 30
      }
    }
  }
}
```

### 5. update_record
更新现有记录。

**请求:**
```json
{
  "jsonrpc": "2.0",
  "id": 7,
  "method": "tools/call",
  "params": {
    "name": "update_record",
    "arguments": {
      "table": "users",
      "id": 1,
      "data": {
        "age": 31
      }
    }
  }
}
```

### 6. delete_record
删除记录。

**请求:**
```json
{
  "jsonrpc": "2.0",
  "id": 8,
  "method": "tools/call",
  "params": {
    "name": "delete_record",
    "arguments": {
      "table": "users",
      "id": 1
    }
  }
}
```

### 7. batch_insert
批量插入记录（最多100条）。

**请求:**
```json
{
  "jsonrpc": "2.0",
  "id": 9,
  "method": "tools/call",
  "params": {
    "name": "batch_insert",
    "arguments": {
      "table": "users",
      "items": [
        {"name": "Bob", "email": "bob@example.com", "age": 25},
        {"name": "Charlie", "email": "charlie@example.com", "age": 35}
      ],
      "batch_size": 50
    }
  }
}
```

### 8. batch_update
批量更新记录。

**请求:**
```json
{
  "jsonrpc": "2.0",
  "id": 10,
  "method": "tools/call",
  "params": {
    "name": "batch_update",
    "arguments": {
      "table": "users",
      "updates": [
        {"id": 1, "data": {"age": 31}},
        {"id": 2, "data": {"age": 26}}
      ],
      "batch_size": 50
    }
  }
}
```

### 9. batch_delete
批量删除记录。

**请求:**
```json
{
  "jsonrpc": "2.0",
  "id": 11,
  "method": "tools/call",
  "params": {
    "name": "batch_delete",
    "arguments": {
      "table": "users",
      "ids": [1, 2, 3]
    }
  }
}
```

## 测试

### 运行测试脚本
```bash
python test_simple.py
```

### 手动测试
1. 创建测试数据库（需要安装 sqlite3）：
```bash
sqlite3 test.db < test_data.sql
```

2. 启动服务器：
```bash
cargo run -- --db-path test.db
```

3. 发送 JSON-RPC 请求测试各个功能。

## 部署

### 作为独立服务器运行
```bash
# 构建发布版本
cargo build --release

# 运行服务器
./target/release/sqlite-mcp --db-path /path/to/database.db
```

### Claude Desktop 配置
```json
{
  "mcpServers": {
    "sqlite": {
      "command": "/path/to/sqlite-mcp",
      "args": ["--db-path", "/path/to/database.db"]
    }
  }
}
```

## 技术特性

- **高性能**: 使用 Rust 实现，内存安全且快速
- **异步支持**: 基于 tokio 运行时，支持并发操作
- **事务支持**: 批量操作使用事务确保数据一致性
- **错误处理**: 完善的错误处理和日志记录
- **类型安全**: 使用 serde 进行 JSON 序列化/反序列化
- **SQL 注入防护**: 使用参数化查询防止 SQL 注入攻击

## 依赖项

- `rusqlite`: SQLite 数据库访问
- `tokio`: 异步运行时
- `serde`: JSON 序列化/反序列化
- `schemars`: JSON Schema 生成
- `anyhow`: 错误处理
- `tracing`: 日志记录
- `clap`: 命令行参数解析
- `async-trait`: 异步 trait 支持

## 许可证

MIT License