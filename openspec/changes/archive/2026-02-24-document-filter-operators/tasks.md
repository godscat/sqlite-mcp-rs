## 1. 准备工作

- [x] 1.1 验证当前代码中 filters 参数支持的操作符列表
- [x] 1.2 检查多个条件的逻辑实现（确认是 AND 逻辑）
- [x] 1.3 查看 src/main.rs 中 handle_tools_list 的当前实现

## 2. 完善 query_records 的 filters JSON Schema

- [x] 2.1 查看 src/main.rs 中 query_records 的当前 inputSchema
- [x] 2.2 设计 filters 参数的完整 JSON Schema 结构
- [x] 2.3 在 inputSchema 中添加每个操作符的详细定义
- [x] 2.4 在 filters 描述中说明多个条件的 AND 逻辑关系

## 3. 更新 README 文档

- [x] 3.1 在 README.md 中添加 "Filter Parameter Usage" 章节
- [x] 3.2 编写支持的操作符列表和说明
- [x] 3.3 提供 filters 参数的实际使用示例
- [x] 3.4 添加简单示例（$eq 操作符）
- [x] 3.5 添加范围示例（$gte 和 $lt 操作符）
- [x] 3.6 添加模式匹配示例（$like 操作符）
- [x] 3.7 添加多条件组合示例

## 4. 验证和审查

- [x] 4.1 检查 src/main.rs 中的 JSON Schema 格式是否正确
- [x] 4.2 确保 README 中的示例与 JSON Schema 一致
- [x] 4.3 测试 filters 参数是否能正确解析
