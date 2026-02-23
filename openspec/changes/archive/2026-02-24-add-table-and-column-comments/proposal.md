## Why

SQLite 默认不支持表和列的 COMMENT 语法，这导致数据库元数据缺乏文档说明，增加了代码理解难度和维护成本。通过引入辅助表机制，可以提供类似 MySQL 的表注释和列注释功能，增强数据库文档化能力，提升开发者体验。

## What Changes

- 新增辅助表 `_table_comment` 用于存储表注释
- 新增辅助表 `_table_column_comment` 用于存储列注释
- 修改 `get_table_schema` 工具：
  - 检测并自动创建辅助表（如果不存在）
  - 自动初始化表和列的默认注释（使用表名/列名作为描述）
  - 查询并返回表注释和列注释信息
  - 扩展 JSON 响应格式，添加 `desc` 字段到表和列对象

## Capabilities

### New Capabilities
- `table-column-comments`: 提供表注释和列注释的存储、查询和管理功能，增强数据库元数据的文档化能力

### Modified Capabilities
- `get-table-schema`: 修改返回的 JSON 结构，添加 `desc` 字段到表对象和列对象，包含从辅助表查询的注释信息

## Impact

**受影响的代码**:
- `src/tools/get_schema.rs` - 修改 `execute` 函数以支持注释查询
- `src/db/adapter.rs` - 可能需要扩展 `TableSchema` 结构以包含注释字段
- 新增 `src/db/comments.rs` 或在现有文件中添加注释相关函数

**API 变更**:
- `get_table_schema` 工具的返回 JSON 格式扩展：
  - 顶层添加 `desc` 字段（表注释）
  - 每个列对象添加 `desc` 字段（列注释）

**数据库变更**:
- 创建两个辅助表（如果不存在）
- 自动插入默认注释数据

**依赖变更**:
- 无新增外部依赖
- 使用现有的 rusqlite 和 anyhow