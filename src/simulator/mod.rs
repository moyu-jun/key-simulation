use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub enum KeyCode {
    Z,
}

impl KeyCode {
    /// PS/2 Set 1 硬件扫描码（Interception 使用此编码）
    pub fn scan_code(self) -> u16 {
        match self {
            KeyCode::Z => 0x2C,
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

mod interception;

pub use interception::InterceptionSimulator;

pub fn create_simulator(_method: &str) -> Result<Box<dyn KeySimulator>> {
    Ok(Box::new(InterceptionSimulator::new()?))
}
