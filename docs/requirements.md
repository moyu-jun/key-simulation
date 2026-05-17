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

## 验收标准

1. F10 热键能在游戏运行时正常触发和停止模拟
2. 至少有一种模拟方案能被目标游戏正确识别为真实按键输入
3. 模拟间隔稳定在 1000ms 左右
4. 程序运行稳定，无内存泄漏或崩溃
5. `cargo build --release` 生成可独立运行的 `.exe` 文件
6. 双击 `.exe` 自动触发 UAC 提权，以管理员身份运行
