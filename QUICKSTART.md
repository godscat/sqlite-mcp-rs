# 快速开始

5 分钟配置 SQLite MCP 服务器。

## 步骤 1: 构建

```bash
cargo build --release
```

## 步骤 2: 创建测试数据库（可选）

```bash
cd test
sqlite3 test.db < test_data.sql
cd ..
```

## 步骤 3: 验证服务器

```bash
python verify_server.py
```

看到 "✅ 服务器配置正确" 表示成功。

## 步骤 4: 配置 Claude Desktop

编辑配置文件：

- **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`
- **macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Linux**: `~/.config/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "sqlite": {
"command": "/path/to/sqlite-mcp-rs",
      "args": ["--db-path", "/path/to/database.db"],
      "env": {
        "RUST_LOG": "error"
      }
    }
  }
}

## 步骤 5: 重启 Claude Desktop

完全退出后重新启动。

## 只读模式

```json
{
  "mcpServers": {
    "sqlite-readonly": {
      "command": "/path/to/sqlite-mcp-rs",
      "args": ["--db-path", "/path/to/database.db", "--readonly"],
      "env": { "RUST_LOG": "error" }
    }
  }
}
```

## 测试功能

在 Claude Desktop 中：

- "列出数据库中的所有表"
- "查询 users 表中所有用户"

## 故障排除

详见 [TROUBLESHOOTING.md](TROUBLESHOOTING.md)