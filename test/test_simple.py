#!/usr/bin/env python3
"""
SQLite MCP Server Test Script
This script demonstrates how to interact with the SQLite MCP server
"""

import subprocess
import json
import sys
import os


def run_test():
    """Test the SQLite MCP server functionality"""
    print("=== SQLite MCP Server Test ===\n")

    # Check if we're in the right directory
    if not os.path.exists("Cargo.toml"):
        print("Error: Please run this script from the sqlite-mcp project directory")
        sys.exit(1)

    print("1. Testing server startup...")

    # Test simple commands
    commands = [
        {
            "name": "Help command",
            "cmd": ["cargo", "run", "--", "--help"],
            "timeout": 10,
        },
        {"name": "Version check", "cmd": ["cargo", "version"], "timeout": 5},
    ]

    for test in commands:
        print(f"\n{test['name']}:")
        try:
            result = subprocess.run(
                test["cmd"], capture_output=True, text=True, timeout=test["timeout"]
            )

            if result.returncode == 0:
                print(f"✓ {test['name']} executed successfully")
                if result.stdout:
                    print(f"  Output: {result.stdout[:200]}...")
            else:
                print(f"✗ {test['name']} failed")
                print(f"  Error: {result.stderr[:200]}...")

        except subprocess.TimeoutExpired:
            print(f"✗ {test['name']} timed out")
        except Exception as e:
            print(f"✗ {test['name']} error: {e}")

    print("\n2. Testing server functionality...")
    print("To test the MCP server interactively:")
    print("  1. Start the server: cargo run -- --db-path test.db")
    print("  2. Send JSON-RPC requests to stdin")
    print("\nExample requests:")

    example_requests = [
        {
            "description": "Initialize server",
            "request": {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {"protocolVersion": "2025-06-18", "capabilities": {}},
            },
        },
        {
            "description": "List available tools",
            "request": {"jsonrpc": "2.0", "id": 2, "method": "tools/list"},
        },
        {
            "description": "List database tables",
            "request": {
                "jsonrpc": "2.0",
                "id": 3,
                "method": "tools/call",
                "params": {"name": "list_tables", "arguments": {}},
            },
        },
    ]

    for example in example_requests:
        print(f"\n{example['description']}:")
        print(json.dumps(example["request"], indent=2))

    print("\n=== Test completed ===")
    print("\nNext steps:")
    print("1. Install sqlite3 to create test databases")
    print("2. Run: cargo build --release")
    print("3. Test with: cargo run -- --db-path your_database.db")


if __name__ == "__main__":
    run_test()
