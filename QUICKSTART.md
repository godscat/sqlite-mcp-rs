# SQLite MCP 服务器 - 快速开始

## 🚀 5 分钟快速配置

### 步骤 1: 构建项目（如果还没有）

```bash
cargo build --release
```

### 步骤 2: 创建测试数据库（可选）

```bash
cd test
sqlite3 test.db < test_data.sql
cd ..
```

### 步骤 3: 验证服务器工作正常

```bash
python verify_server.py
```

如果看到 "✅ 服务器配置正确，可以与 Claude Desktop 集成！"，说明一切就绪！

### 步骤 4: 配置 Claude Desktop

编辑配置文件：

**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`
**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
**Linux**: `~/.config/Claude/claude_desktop_config.json`

添加以下配置：

```json
{
  "mcpServers": {
    "sqlite": {
      "command": "E:/Workspace/mcp-servers/sqlite-mcp/target/release/sqlite-mcp.exe",
      "args": [
        "--db-path",
        "E:/Workspace/mcp-servers/sqlite-mcp/test/test.db"
      ],
      "env": {
        "RUST_LOG": "error"
      }
    }
  }
}
```

**重要**：
- 将路径替换为您实际的可执行文件路径
- 将 `RUST_LOG` 设置为 `error` 以避免日志干扰
- 确保数据库文件路径正确

### 步骤 5: 重启 Claude Desktop

1. 完全退出 Claude Desktop
2. 重新启动 Claude Desktop
3. 在聊天中测试 SQLite 功能

## 🎯 测试 SQLite 功能

在 Claude Desktop 中，您可以：

1. **列出表**："列出数据库中的所有表"
2. **查询数据**："查询 users 表中所有用户"
3. **插入数据**："插入一个新用户到 users 表"
4. **更新数据**："更新 ID 为 1 的用户的年龄"
5. **删除数据**："删除 ID 为 2 的用户"

## 🔧 高级配置

### 使用生产数据库

将配置中的数据库路径更改为您的实际数据库：

```json
{
  "mcpServers": {
    "sqlite": {
      "command": "/path/to/sqlite-mcp",
      "args": ["--db-path", "/path/to/your/database.db"],
      "env": {
        "RUST_LOG": "error"
      }
    }
  }
}
```

### 只读模式（安全配置）

```json
{
  "mcpServers": {
    "sqlite-readonly": {
      "command": "/path/to/sqlite-mcp",
      "args": [
        "--db-path",
        "/path/to/your/database.db",
        "--readonly"
      ],
      "env": {
        "RUST_LOG": "error"
      }
    }
  }
}
```

## 📋 故障排除

### 问题：Claude Desktop 中看不到 SQLite 工具

**解决方案**：
1. 检查配置文件路径是否正确
2. 验证可执行文件路径存在
3. 查看 Claude Desktop 的日志了解错误
4. 重启 Claude Desktop

### 问题：连接超时或无响应

**解决方案**：
1. 手动运行服务器验证是否工作：
   ```bash
   ./target/release/sqlite-mcp.exe --db-path your_database.db
   ```
2. 发送测试请求验证响应
3. 检查防火墙设置

### 问题：JSON 解析错误

**解决方案**：
1. 确保使用最新版本：`cargo build --release`
2. 设置 `RUST_LOG=error` 减少日志输出
3. 参考 [TROUBLESHOOTING.md](TROUBLESHOOTING.md) 获取详细信息

## 📚 更多文档

- [README.md](README.md) - 项目概述和基本功能
- [USAGE.md](USAGE.md) - 详细 API 文档
- [TROUBLESHOOTING.md](TROUBLESHOOTING.md) - 故障排除指南
- [AGENTS.md](AGENTS.md) - 开发者指南

## 🎉 完成！

现在您已经成功配置了 SQLite MCP 服务器，可以在 Claude Desktop 中使用所有数据库功能了！

如果您有任何问题，请参考故障排除文档或提交 Issue。