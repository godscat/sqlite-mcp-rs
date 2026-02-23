# SQLite MCP 服务器测试

本目录包含用于测试 SQLite MCP 服务器的文件。

## 文件说明

### demo.py
完整的 Python 演示脚本，展示如何与 SQLite MCP 服务器交互。包含：
- 创建测试数据库
- 测试各种 MCP 操作（初始化、列表表、查询、插入等）
- 演示复杂过滤查询

使用方法：
```bash
cd test
python demo.py
```

### test_data.sql
测试数据库的 SQL 脚本，包含示例表和数据：
- users 表：用户信息
- products 表：产品信息

使用方法：
```bash
# 创建测试数据库
sqlite3 test.db < test_data.sql
```

### test_simple.py
简单的测试脚本，用于验证服务器基本功能。

## 手动测试

1. **创建测试数据库**：
   ```bash
   cd test
   sqlite3 test.db < test_data.sql
   ```

2. **启动服务器**：
   ```bash
   cargo run -- --db-path test/test.db
   ```

3. **发送测试请求**（示例）：
   ```json
   {"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2025-06-18", "capabilities": {}}}
   ```

## 测试用例

### 基本操作测试
- 初始化服务器
- 列出可用工具
- 列出数据库表
- 获取表结构
- 查询记录

### 数据操作测试
- 插入新记录
- 更新现有记录
- 删除记录
- 批量操作

### 高级功能测试
- 复杂过滤查询
- 分页查询
- 只读模式测试

## 预期结果

所有测试应该返回有效的 JSON-RPC 响应，包含相应的数据或确认信息。

## 故障排除

1. **确保 sqlite3 可用**：如果没有 sqlite3 命令，需要先安装 SQLite
2. **数据库路径**：确保数据库文件路径正确
3. **权限问题**：确保有读写数据库文件的权限