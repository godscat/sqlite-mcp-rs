## 1. 准备阶段

- [x] 1.1 阅读并理解 `src/tools/get_schema.rs` 现有实现
- [x] 1.2 确认 `sql/sqlite_assist_table_ddl.sql` 文件存在且格式正确
- [x] 1.3 查看现有的 `TableSchema` 和 `Column` 结构体定义（通常在 `src/db/adapter.rs`）

## 2. 数据结构扩展

- [x] 2.1 在 `TableSchema` 结构体中添加可选的 `desc` 字段（Option<String>）
- [x] 2.2 在 `Column` 结构体中添加可选的 `desc` 字段（Option<String>）
- [x] 2.3 确保添加的字段支持 serde 序列化和反序列化

## 3. 辅助表管理功能

- [x] 3.1 创建 `ensure_auxiliary_tables_exist` 函数：检测辅助表是否存在，不存在则创建
- [x] 3.2 实现 DDL 读取功能：从 `sql/sqlite_assist_table_ddl.sql` 读取并执行 DDL 语句
- [x] 3.3 使用事务包装辅助表创建操作，确保原子性
- [x] 3.4 添加日志记录辅助表创建操作

## 4. 默认注释初始化

- [x] 4.1 创建 `initialize_default_table_comment` 函数：为指定表插入默认注释到 _table_comment
- [x] 4.2 创建 `initialize_default_column_comments` 函数：为指定表的所有列插入默认注释到 _table_column_comment
- [x] 4.3 使用 INSERT OR IGNORE 确保幂等性（不覆盖已存在的注释）
- [x] 4.4 添加日志记录默认注释插入操作

## 5. 注释查询功能

- [x] 5.1 创建 `get_table_comment` 函数：从 _table_comment 查询表注释
- [x] 5.2 创建 `get_column_comments` 函数：从 _table_column_comment 查询列注释
- [x] 5.3 处理查询结果为空的情况（返回 None 或使用默认值）

## 6. get_schema 工具集成

- [x] 6.1 在 `get_table_schema` 函数开始处调用辅助表创建逻辑
- [x] 6.2 在获取表结构后调用默认注释初始化逻辑
- [x] 6.3 调用注释查询函数获取表和列的注释
- [x] 6.4 将注释信息填充到 TableSchema 和 Column 对象的 desc 字段
- [x] 6.5 更新返回的 JSON 序列化，确保 desc 字段包含在输出中

## 7. 错误处理和日志

- [x] 7.1 添加 DDL 执行失败的错误处理和详细错误信息
- [x] 7.2 添加辅助表操作失败的错误处理
- [x] 7.3 添加调试日志记录关键操作（表创建、注释初始化、查询等）
- [x] 7.4 确保所有数据库操作都包含适当的错误上下文信息

## 8. 测试验证

- [x] 8.1 测试场景1：首次调用 get_table_schema，验证辅助表自动创建
- [x] 8.2 测试场景2：首次查询表，验证默认注释自动初始化
- [x] 8.3 测试场景3：查询已存在的表，验证注释正确返回
- [x] 8.4 测试场景4：重复调用 get_table_schema，验证幂等性（不影响现有数据）
- [x] 8.5 测试场景5：验证返回的 JSON 格式包含 desc 字段
- [x] 8.6 测试场景6：手动修改注释后查询，验证返回更新后的注释
- [x] 8.7 运行 `cargo check` 确保代码编译通过
- [x] 8.8 运行 `cargo test` 确保所有测试通过（如果有测试）

## 9. 代码质量

- [x] 9.1 检查代码风格符合项目规范（参考 AGENTS.md）
- [x] 9.2 确保所有函数都有适当的错误处理
- [x] 9.3 添加必要的文档注释（如果需要）
- [x] 9.4 确保代码遵循 Rust 最佳实践和安全性原则