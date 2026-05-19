mod hotkey;
mod logger;
mod simulator;

use std::io::{self, Write};
use std::time::{Duration, Instant};
use simulator::{KeyCode, create_simulator};
use hotkey::HotkeyListener;

fn pause_before_exit() {
    let _ = writeln!(io::stdout(), "\n按回车键退出...");
    let _ = io::stdout().flush();
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);
}

fn main() {
    logger::init();

    log_msg!("初始化按键模拟器...");
    let sim = match create_simulator() {
        Ok(s) => {
            log_msg!("模拟器初始化成功: {}", s.name());
            s
        }
        Err(e) => {
            log_msg!("[致命] 模拟器初始化失败: {:?}", e);
            pause_before_exit();
            std::process::exit(1);
        }
    };

    let mut listener = HotkeyListener::new();
    let mut active = false;
    let mut last_press = Instant::now();
    let interval = Duration::from_millis(1000);

    log_msg!("=== 按键模拟工具 ===");
    log_msg!("当前方案: {}", sim.name());
    log_msg!("按 F10 启动/停止模拟（每 1000ms 模拟 Z 键）");
    log_msg!("按 Ctrl+C 退出程序");

    loop {
        if listener.is_triggered() {
            active = !active;
            if active {
                log_msg!("[启动] 模拟已开启 ({})", sim.name());
                last_press = Instant::now() - interval;
            } else {
                log_msg!("[停止] 模拟已关闭");
            }
        }

        if active && last_press.elapsed() >= interval {
            if let Err(e) = sim.key_press(KeyCode::Z) {
                log_msg!("[错误] 模拟按键失败: {}", e);
            }
            last_press = Instant::now();
        }

        std::thread::sleep(Duration::from_millis(10));
    }
}
