## Context

当前 SQLite MCP 服务器中，query_records 工具的 filters 参数在 handle_tools_list 返回的 JSON Schema 中只有简单的 "type": "object" 和 "description": "Filter conditions (optional)" 描述。用户不知道支持哪些操作符、如何正确构造 filter 对象、以及多个条件之间的逻辑关系。这导致用户在使用 query_records 工具时遇到困难。

现有代码已经实现了 filter 功能（$eq, $gt, $gte, $lt, $lte, $ne, $like 操作符），但 MCP 工具定义的 JSON Schema 缺少这些细节说明。

## Goals / Non-Goals

**Goals:**
- 完善 src/main.rs 中 handle_tools_list 返回的 query_records 工具的 filters 参数 inputSchema
- 在 inputSchema 中明确列出所有支持的操作符
- 提供 filters 参数的完整 JSON Schema 结构示例
- 说明多个条件的默认逻辑关系（AND）
- 在 README 中添加统一的 filter 参数使用指南和示例

**Non-Goals:**
- 修改 filter 功能的代码实现
- 改变现有的 filter 操作符行为
- 添加新的操作符或逻辑运算符（如 OR）
- 实现 filter 功能的变更

## Decisions

**Decision 1: JSON Schema 结构选择**
- 在 query_records 的 inputSchema 中，为 filters 参数添加详细的 properties 定义
- 使用嵌套的 JSON Schema 来表示操作符和值的结构
- 理由：这是 MCP 工具定义的标准方式，客户端可以自动生成 UI

**Decision 2: 文档位置选择**
- 在 README.md 中添加统一的 filter 使用指南
- 在 src/main.rs 的 handle_tools_list 中完善 JSON Schema
- 理由：MCP 客户端直接读取 handle_tools_list 的 JSON Schema，README 提供额外说明

**Decision 3: 示例覆盖范围**
- 为每个操作符提供至少一个示例
- 提供实际使用场景的复杂示例（组合多个条件）
- 理由：全面覆盖，帮助用户理解不同场景下的用法

## Risks / Trade-offs

**Risk: 文档与实现可能不一致**
- Mitigation: 文档更新前先验证当前代码实际支持的操作符
- Mitigation: 未来添加新操作符时同步更新文档

**Risk: 文档过多导致信息过载**
- Mitigation: 将详细示例放在 README 统一指南中，工具文档只保留关键信息

**Trade-off: 维护成本**
- 更新代码时需要同步更新文档
- 但文档化减少了用户支持成本和误解

## Migration Plan

无需迁移计划 - 这是纯文档变更，不影响运行时行为。

## Open Questions

无
