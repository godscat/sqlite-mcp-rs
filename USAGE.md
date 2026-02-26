# API 参考

完整的 JSON-RPC 2.0 API 文档。

## 协议概述

服务器通过标准输入/输出接收 JSON-RPC 2.0 格式的请求。

### 初始化

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

### 获取工具列表

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/list"
}
```

## 工具 API

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

获取指定表的结构，包含表和列的描述信息。

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

**响应:**
```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"name\":\"users\",\"desc\":\"用户表\",\"columns\":[{\"name\":\"id\",\"desc\":\"主键ID\",\"data_type\":\"INTEGER\",\"not_null\":true,\"default_value\":null,\"is_primary_key\":true},{\"name\":\"name\",\"desc\":\"用户名称\",\"data_type\":\"TEXT\",\"not_null\":false,\"default_value\":null,\"is_primary_key\":false}],\"primary_key\":\"id\"}"
      }
    ]
  }
}
```

> `desc` 字段包含表和列的描述信息。首次查询时自动创建辅助表 `_table_comment` 和 `_table_column_comment`。

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

**过滤操作符:**
| 操作符 | 描述 |
|--------|------|
| `$eq` | 等于 |
| `$ne` | 不等于 |
| `$gt` | 大于 |
| `$gte` | 大于等于 |
| `$lt` | 小于 |
| `$lte` | 小于等于 |
| `$in` | 在列表中 |
| `$like` | 模糊匹配 |

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

## 类型映射

| SQLite | JSON |
|--------|------|
| INTEGER | number |
| REAL | number |
| TEXT | string |
| BLOB | string (base64) |
| NULL | null |

## 依赖项

- `rusqlite` - SQLite 数据库访问
- `tokio` - 异步运行时
- `serde` - JSON 序列化
- `schemars` - JSON Schema 生成
- `anyhow` - 错误处理
- `tracing` - 日志记录
- `clap` - 命令行参数解析
- `async-trait` - 异步 trait 支持