#!/usr/bin/env python3
"""
SQLite MCP Server 演示脚本
这个脚本演示如何与 SQLite MCP 服务器交互
"""

import subprocess
import json
import sys
import os


def create_test_database():
    """创建测试数据库"""
    print("创建测试数据库...")
    if os.path.exists("test.db"):
        os.remove("test.db")

    # 使用 sqlite3 创建数据库
    result = subprocess.run(
        ["D:/dev/sqlite/sqlite3.exe", "test.db", "<", "test_data.sql"],
        capture_output=True,
        text=True,
        shell=True,
    )

    if result.returncode == 0:
        print("✅ 测试数据库创建成功")
    else:
        print("❌ 测试数据库创建失败")
        print(result.stderr)
        return False

    return True


def run_mcp_test():
    """运行 MCP 服务器测试"""
    print("\n=== SQLite MCP 服务器演示 ===\n")

    # 创建测试数据库
    if not create_test_database():
        print("无法创建测试数据库，退出...")
        return

    # 测试请求
    test_cases = [
        {
            "name": "初始化服务器",
            "request": {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {"protocolVersion": "2025-06-18", "capabilities": {}},
            },
        },
        {
            "name": "列出数据库表",
            "request": {
                "jsonrpc": "2.0",
                "id": 2,
                "method": "tools/call",
                "params": {"name": "list_tables", "arguments": {}},
            },
        },
        {
            "name": "查询所有用户",
            "request": {
                "jsonrpc": "2.0",
                "id": 3,
                "method": "tools/call",
                "params": {"name": "query_records", "arguments": {"table": "users"}},
            },
        },
        {
            "name": "插入新用户",
            "request": {
                "jsonrpc": "2.0",
                "id": 4,
                "method": "tools/call",
                "params": {
                    "name": "insert_record",
                    "arguments": {
                        "table": "users",
                        "data": {"name": "Eve", "email": "eve@example.com", "age": 26},
                    },
                },
            },
        },
        {
            "name": "查询年龄大于25的用户",
            "request": {
                "jsonrpc": "2.0",
                "id": 5,
                "method": "tools/call",
                "params": {
                    "name": "query_records",
                    "arguments": {"table": "users", "filters": {"age": {"$gt": 25}}},
                },
            },
        },
    ]

    print("开始测试 MCP 服务器功能...\n")

    for i, test_case in enumerate(test_cases, 1):
        print(f"测试 {i}: {test_case['name']}")
        print(f"请求: {json.dumps(test_case['request'], ensure_ascii=False, indent=2)}")

        proc = None
        try:
            # 运行服务器并发送请求
            proc = subprocess.Popen(
                ["cargo", "run", "--", "--db-path", "test.db"],
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
            )

            request_str = json.dumps(test_case["request"]) + "\n"
            stdout, stderr = proc.communicate(input=request_str, timeout=10)

            if proc.returncode == 0 and stdout.strip():
                try:
                    response = json.loads(stdout.strip())
                    print(f"响应: {json.dumps(response, ensure_ascii=False, indent=2)}")
                except json.JSONDecodeError:
                    print(f"响应: {stdout}")
            else:
                print(f"错误: {stderr}")

        except subprocess.TimeoutExpired:
            if proc:
                proc.kill()
            print("❌ 请求超时")
        except Exception as e:
            print(f"❌ 测试失败: {e}")

        print("-" * 60)

    print("\n✅ 演示完成！")
    print("\n要手动运行服务器，请使用:")
    print("  cargo run -- --db-path test.db")
    print("\n然后发送 JSON-RPC 请求到标准输入。")


if __name__ == "__main__":
    run_mcp_test()
