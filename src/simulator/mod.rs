use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub enum KeyCode {
    Z,
}

impl KeyCode {
    /// DD 驱动专用按键编码（区别于 PS/2 扫描码与 Windows VK 码）
    pub fn dd_code(self) -> i32 {
        match self {
            KeyCode::Z => 401,
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

pub use dd::DDSimulator;

pub fn create_simulator() -> Result<Box<dyn KeySimulator>> {
    Ok(Box::new(DDSimulator::new()?))
}
