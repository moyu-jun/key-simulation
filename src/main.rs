mod hotkey;
mod simulator;

use std::time::{Duration, Instant};
use simulator::{KeyCode, create_simulator};
use hotkey::HotkeyListener;

fn main() {
    let method = std::env::args()
        .skip_while(|a| a != "--method")
        .nth(1)
        .unwrap_or_else(|| "post_message".to_string());

    let sim = create_simulator(&method);
    let mut listener = HotkeyListener::new();
    let mut active = false;
    let mut last_press = Instant::now();
    let interval = Duration::from_millis(1000);

    println!("=== 按键模拟工具 ===");
    println!("当前方案: {}", sim.name());
    println!("可用方案: post_message, hook, interception");
    println!("按 F10 启动/停止模拟（每 1000ms 模拟 Z 键）");
    println!("按 Ctrl+C 退出程序");

    loop {
        if listener.is_triggered() {
            active = !active;
            if active {
                println!("[启动] 模拟已开启 ({})", sim.name());
                last_press = Instant::now() - interval;
            } else {
                println!("[停止] 模拟已关闭");
            }
        }

        if active && last_press.elapsed() >= interval {
            if let Err(e) = sim.key_press(KeyCode::Z) {
                eprintln!("模拟按键失败: {}", e);
            }
            last_press = Instant::now();
        }

        std::thread::sleep(Duration::from_millis(10));
    }
}
