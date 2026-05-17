# 项目需求文档

## 概述

本项目是一个面向游戏场景的按键模拟工具，使用 Rust 编写。项目的核心目标是探索和验证多种按键模拟方案，找到能够在游戏环境中稳定工作的实现方式。

## 功能需求

### 1. 全局热键检测

- 监听全局热键 **F10** 作为开关
- F10 按下时启动按键模拟，再次按下时停止（切换模式）
- 热键检测需在后台持续运行，不依赖窗口焦点

### 2. 按键模拟

- 触发后每隔 **1000ms** 模拟一次 **Z 键**按下
- 模拟目标为当前前台窗口（游戏窗口）
- 需要模拟完整的按键事件（按下 + 释放）

### 3. 游戏反屏蔽

- 游戏通常会屏蔽常规的按键模拟方式（如 `SendInput`、`PostMessage`）
- 需要探索更底层或更隐蔽的模拟方案以绕过检测

## 架构设计

### 统一接口规范

所有按键模拟方案必须实现统一的 trait 接口，对外暴露相同的调用方式。上层业务逻辑（热键监听、定时触发）仅依赖 trait，不感知具体实现。这样可以：

- 运行时动态切换模拟方案
- 新增方案时无需修改上层代码
- 便于对比测试不同方案的效果

```rust
pub trait KeySimulator {
    /// 模拟按键按下
    fn key_down(&self, key: KeyCode) -> Result<()>;

    /// 模拟按键释放
    fn key_up(&self, key: KeyCode) -> Result<()>;

    /// 模拟完整按键（按下 + 释放）
    fn key_press(&self, key: KeyCode) -> Result<()> {
        self.key_down(key)?;
        self.key_up(key)
    }

    /// 返回方案名称，用于日志和调试
    fn name(&self) -> &str;
}
```

上层调用示例：

```rust
fn run_simulation(simulator: &dyn KeySimulator, key: KeyCode, interval: Duration) {
    loop {
        simulator.key_press(key).unwrap();
        std::thread::sleep(interval);
    }
}
```

### 技术探索方向

本项目将逐一实现并测试以下按键模拟方案，所有方案均实现 `KeySimulator` trait：

| 方案 | 层级 | 说明 |
|------|------|------|
| `SendInput` | Win32 API | 最常见的模拟方式，易被游戏屏蔽 |
| `keybd_event` | Win32 API | 旧版 API，部分游戏仍可识别 |
| `PostMessage` / `SendMessage` | 窗口消息 | 向目标窗口发送 WM_KEYDOWN/WM_KEYUP |
| 扫描码模拟 | 硬件级 | 使用硬件扫描码而非虚拟键码 |
| `SetWindowsHookEx` | 钩子注入 | 通过键盘钩子注入按键事件 |
| 驱动级模拟 | 内核层 | 通过虚拟键盘驱动发送输入（如 Interception） |

## 构建与分发

### 编译为 .exe

- 使用 `cargo build --release` 生成 Release 版本的 `.exe` 可执行文件
- 最终测试以 `.exe` 文件直接运行为准

### 管理员权限

程序启动时需要管理员权限（部分模拟方案需要提升权限才能注入输入到游戏进程）。通过 Windows 应用程序清单（manifest）实现自动 UAC 提权：

- 项目根目录放置 `app.manifest` 文件，声明 `requireAdministrator`
- 通过 `build.rs` 在编译时嵌入 manifest 到 `.exe` 中
- 双击运行时自动弹出 UAC 提权对话框

### 构建命令

```bash
cargo build --release    # 生成 target/release/key-simulation.exe
```

## 项目结构规划

```
src/
├── main.rs              # 入口，热键监听与主循环
├── hotkey.rs            # 全局热键检测
├── simulator/           # 按键模拟方案
│   ├── mod.rs           # KeySimulator trait 定义 + 方案注册
│   ├── send_input.rs    # SendInput 实现
│   ├── keybd_event.rs   # keybd_event 实现
│   ├── post_message.rs  # PostMessage 实现
│   ├── scan_code.rs     # 扫描码实现
│   ├── hook.rs          # SetWindowsHookEx 实现
│   └── driver.rs        # 驱动级实现
├── config.rs            # 配置（按键、间隔、方案选择）
app.manifest             # Windows UAC 管理员权限清单
build.rs                 # 编译脚本，嵌入 manifest
```

## 测试记录

| 方案 | 记事本 | 游戏 | 结论 |
|------|--------|------|------|
| `SendInput`（虚拟键码） | 正常生效 | 无效 | 被游戏屏蔽 |
| `ScanCode`（硬件扫描码） | 正常生效 | 无效 | 被游戏屏蔽 |
| `keybd_event`（旧版 API） | 正常生效 | 无效 | 被游戏屏蔽 |
| `PostMessage`（窗口消息） | 正常生效 | 无效 | 被游戏屏蔽 |
| `Hook`（WH_KEYBOARD_LL + ScanCode） | 正常生效 | 无效 | 被游戏屏蔽 |
| `Interception`（驱动级） | 无效 | 无效 | DLL 已加载但未生效（见下方分析） |

测试条件：管理员权限启动，Windows 10。

结论：所有用户态方案均被游戏屏蔽。Interception 方案 DLL 可加载但输入未生效，原因分析如下。

### Interception 不生效原因分析

**核心问题：仅有 DLL 不够，必须安装内核驱动。**

`interception.dll` 只是用户态接口库，实际的键盘输入注入由内核驱动 `keyboard.sys`（Interception 驱动）完成。如果驱动未安装：
- `interception_create_context()` 会返回一个看似有效的句柄
- `interception_send()` 调用不会报错，但实际无数据发送到输入栈
- 程序表现为"能启动但无效果"

### 解决步骤

1. 从 https://github.com/oblitum/Interception/releases 下载 **Interception 安装包**
2. 以管理员身份运行：`install-interception.exe /install`
3. **重启系统**（驱动需要重启后生效）
4. 重启后再运行 `key-simulation.exe --method interception`

## 验收标准

1. F10 热键能在游戏运行时正常触发和停止模拟
2. 至少有一种模拟方案能被目标游戏正确识别为真实按键输入
3. 模拟间隔稳定在 1000ms 左右
4. 程序运行稳定，无内存泄漏或崩溃
5. `cargo build --release` 生成可独立运行的 `.exe` 文件
6. 双击 `.exe` 自动触发 UAC 提权，以管理员身份运行
