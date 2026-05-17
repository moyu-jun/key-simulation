use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub enum KeyCode {
    Z,
}

impl KeyCode {
    /// DD 驱动自定义键码
    /// Z 在 DD 键码表中是 404（第 4 行第 4 列）
    pub fn dd_code(self) -> i32 {
        match self {
            KeyCode::Z => 404,
        }
    }
}

pub trait KeySimulator {
    fn key_down(&self, key: KeyCode) -> Result<()>;
    fn key_up(&self, key: KeyCode) -> Result<()>;

    fn key_press(&self, key: KeyCode) -> Result<()> {
        self.key_down(key)?;
        std::thread::sleep(std::time::Duration::from_millis(50));
        self.key_up(key)
    }

    fn name(&self) -> &str;
}

mod dd;

pub use dd::DdSimulator;

pub fn create_simulator(_method: &str) -> Box<dyn KeySimulator> {
    match DdSimulator::new() {
        Ok(s) => Box::new(s),
        Err(e) => {
            eprintln!("[错误] 初始化 DD 驱动失败: {:?}", e);
            std::process::exit(1);
        }
    }
}
