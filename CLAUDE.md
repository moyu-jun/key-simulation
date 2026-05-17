# CLAUDE.md

本文件为 Claude Code (claude.ai/code) 在此仓库中工作时提供指导。

## 项目

使用 Rust（edition 2024）编写的按键模拟工具。

## 构建与测试命令

```bash
cargo build              # 编译
cargo run                # 运行二进制文件
cargo test               # 运行所有测试
cargo test test_name     # 运行匹配模式的测试
cargo fmt                # 格式化代码
cargo clippy             # 代码检查
cargo check              # 快速编译检查（无代码生成）
```

## 约定

- Rust edition 2024
- 提交前使用 `cargo fmt` 格式化
- 使用 `cargo clippy -- -D warnings` 将 lint 警告视为错误
- 错误处理：应用层使用 `anyhow`，库风格的类型化错误使用 `thiserror`
- 函数参数优先使用 `&str` 而非 `String`；需要所有权的构造函数使用 `impl Into<String>`

