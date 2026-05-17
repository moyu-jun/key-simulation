use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::OnceLock;

static LOG_PATH: OnceLock<Mutex<PathBuf>> = OnceLock::new();

pub fn init() {
    let path = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join("key-simulation.log")))
        .unwrap_or_else(|| PathBuf::from("key-simulation.log"));

    let _ = std::fs::write(&path, "");
    let _ = LOG_PATH.set(Mutex::new(path));

    log("=== 程序启动 ===");
}

pub fn log(msg: &str) {
    let line = format!("[{}] {}\n", timestamp(), msg);

    print!("{}", line);
    let _ = std::io::stdout().flush();

    if let Some(lock) = LOG_PATH.get() {
        if let Ok(path) = lock.lock() {
            if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(&*path) {
                let _ = f.write_all(line.as_bytes());
            }
        }
    }
}

fn timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let h = (secs / 3600) % 24;
    let m = (secs / 60) % 60;
    let s = secs % 60;
    format!("{:02}:{:02}:{:02}", h, m, s)
}

#[macro_export]
macro_rules! log_msg {
    ($($arg:tt)*) => {
        $crate::logger::log(&format!($($arg)*))
    };
}
