#!/usr/bin/env python3
"""
快速验证 SQLite MCP 服务器是否正常工作
"""

import subprocess
import json
import sys
import os


def test_server():
    """测试服务器是否输出纯 JSON"""
    print("测试 SQLite MCP 服务器...")
    print("=" * 50)

    # 测试初始化请求
    request = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {"protocolVersion": "2025-06-18", "capabilities": {}},
    }

    request_str = json.dumps(request)
    print(f"发送请求: {request_str}")
    print()

    try:
        # 运行服务器，捕获 stdout
        result = subprocess.run(
            ["./target/release/sqlite-mcp.exe", "--db-path", "test/test.db"],
            input=request_str + "\n",
            stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL,  # 忽略 stderr 输出
            text=True,
        )

        stdout = result.stdout.strip() if result.stdout else ""
        stderr = ""

        print(f"接收到的 stdout:")
        print(stdout)
        print()

        if stderr:
            print(f"警告: stderr 有输出: {stderr}")
            print()

        # 尝试解析响应
        try:
            response = json.loads(stdout)
            print("✅ 成功！响应是有效的 JSON")
            print()
            print(f"响应内容:")
            print(json.dumps(response, indent=2, ensure_ascii=False))
            print()

            # 验证必要字段
            if "jsonrpc" in response and "result" in response:
                print("✅ 响应包含必需的字段")
                print()
                print("服务器配置正确，可以与 Claude Desktop 集成！")
                return True
            else:
                print("❌ 响应缺少必需的字段")
                return False

        except json.JSONDecodeError as e:
            print(f"❌ 响应不是有效的 JSON")
            print(f"错误: {e}")
            print()
            print("请确保使用最新版本的服务器，并查看 TROUBLESHOOTING.md")
            return False

    except FileNotFoundError:
        print("❌ 找不到服务器可执行文件")
        print("请先运行: cargo build --release")
        return False
    except Exception as e:
        print(f"❌ 测试失败: {e}")
        return False


if __name__ == "__main__":
    os.chdir(os.path.dirname(os.path.abspath(__file__)))

    # 检查测试数据库
    if not os.path.exists("test/test.db"):
        print("创建测试数据库...")
        result = subprocess.run(
            ["D:/dev/sqlite/sqlite3.exe", "test/test.db", "read test/test_data.sql"],
            capture_output=True,
            text=True,
            shell=True,
        )
        if result.returncode != 0:
            print(f"创建数据库失败: {result.stderr}")
            sys.exit(1)
        print("✅ 测试数据库创建成功")
        print()

    # 测试服务器
    success = test_server()
    sys.exit(0 if success else 1)
