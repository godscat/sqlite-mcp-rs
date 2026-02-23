# MCP 服务器配置指南

## 问题解决

如果您遇到 "SyntaxError: Unexpected token" 错误，这是因为调试日志干扰了 JSON 响应的解析。

## 解决方案

### 1. 重启服务器

服务器已经更新，现在：
- JSON 响应只输出到 **stdout**
- 日志信息只输出到 **stderr**
- 默认日志级别为 **INFO**（不输出调试信息）

### 2. 启动服务器（推荐方式）

```bash
# 方式 1：重定向 stderr 到日志文件
./target/release/sqlite-mcp.exe --db-path your_database.db 2>> server.log

# 方式 2：完全禁用日志输出
RUST_LOG=off ./target/release/sqlite-mcp.exe --db-path your_database.db

# 方式 3：只显示错误日志
RUST_LOG=error ./target/release/sqlite-mcp.exe --db-path your_database.db
```

### 3. Claude Desktop 配置

在 Claude Desktop 配置文件中添加环境变量：

**Windows 配置示例**：
```json
{
  "mcpServers": {
    "sqlite": {
      "command": "E:/Workspace/mcp-servers/sqlite-mcp/target/release/sqlite-mcp.exe",
      "args": ["--db-path", "E:/Workspace/mcp-servers/sqlite-mcp/test/test.db"],
      "env": {
        "RUST_LOG": "error"
      }
    }
  }
}
```

**Linux/macOS 配置示例**：
```json
{
  "mcpServers": {
    "sqlite": {
      "command": "/path/to/sqlite-mcp",
      "args": ["--db-path", "/path/to/database.db"],
      "env": {
        "RUST_LOG": "error"
      }
    }
  }
}
```

### 4. 验证服务器工作正常

手动测试服务器响应：

```bash
# 测试初始化（应该只返回 JSON，没有其他输出）
echo '{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2025-06-18", "capabilities": {}}}' | \
  ./target/release/sqlite-mcp.exe --db-path test/test.db 2>/dev/null

# 预期输出（应该是纯 JSON）：
# {"id":1,"jsonrpc":"2.0","result":{"capabilities":{"tools":{"listChanged":false},"protocolVersion":"2025-06-18","serverInfo":{"serverInfo":{"name":"sqlite-mcp","version":"0.1.0"}}}
```

## 环境变量说明

| 变量 | 值 | 说明 |
|--------|-----|------|
| `RUST_LOG` | `off` | 完全禁用日志 |
| `RUST_LOG` | `error` | 只显示错误日志 |
| `RUST_LOG` | `warn` | 显示警告和错误日志 |
| `RUST_LOG` | `info` | 显示信息、警告和错误日志（默认） |
| `RUST_LOG` | `debug` | 显示所有日志（包括调试信息） |

## 故障排除

### 问题：仍然遇到 JSON 解析错误

**解决方案 1**：确保使用最新版本
```bash
cargo build --release
```

**解决方案 2**：完全禁用日志
```bash
RUST_LOG=off ./target/release/sqlite-mcp.exe --db-path your_database.db
```

**解决方案 3**：重定向 stderr
```bash
./target/release/sqlite-mcp.exe --db-path your_database.db 2>/dev/null
```

### 问题：服务器无法连接到数据库

**解决方案**：
1. 确认数据库文件存在
2. 检查文件路径是否正确
3. 确保有读取权限

### 问题：MCP 客户端显示连接超时

**解决方案**：
1. 检查服务器是否正常启动
2. 查看日志文件了解错误信息
3. 确认端口或进程没有被占用

## 最佳实践

1. **生产环境**：使用 `RUST_LOG=error` 只记录重要错误
2. **开发环境**：使用 `RUST_LOG=debug` 获取详细调试信息
3. **部署时**：将 stderr 重定向到日志文件用于监控

## 技术细节

- **JSON 响应**：输出到 stdout，确保 MCP 客户端能正确解析
- **日志信息**：输出到 stderr，避免干扰 JSON 解析
- **默认级别**：INFO，平衡了信息量和性能

## 更新日志

### v0.1.1（当前版本）
- ✅ 修复了 JSON 响应包含调试日志的问题
- ✅ 将日志输出重定向到 stderr
- ✅ 调整默认日志级别为 INFO
- ✅ 支持通过环境变量控制日志级别