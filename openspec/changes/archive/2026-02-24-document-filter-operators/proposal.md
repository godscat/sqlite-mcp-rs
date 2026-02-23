## Why

当前 filter 参数的文档缺少关键信息，用户不知道可以使用哪些操作符以及正确的格式，导致使用困难。完善的文档能降低学习成本，提高开发效率。

## What Changes

- 完善 src/main.rs 中 handle_tools_list 返回的 query_records 工具的 filters 参数 inputSchema
- 在 inputSchema 中明确列出支持的操作符（$eq, $gt, $gte, $lt, $lte, $ne, $like）
- 提供 filters 参数的完整 JSON Schema 结构
- 说明多个条件的默认逻辑关系（AND）
- 在 README.md 中添加 filter 参数使用指南

## Capabilities

### New Capabilities
- `filter-documentation`: 统一的 filter 参数文档规范和使用指南

### Modified Capabilities
- `query-records`: 完善 query_records 工具的 MCP 工具定义中的 filters 参数说明

## Impact

受影响的文件：
- src/main.rs (完善 handle_tools_list 中 query_records 的 filters 参数 inputSchema)
- README.md (添加 filter 参数使用指南)

不涉及代码逻辑变更，仅更新 MCP 工具定义的 JSON Schema。
