use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub enum KeyCode {
    Z,
}

impl KeyCode {
    /// DD 驱动自定义键码
    /// Z 在 DD HID 版键码表中是 501
    pub fn dd_code(self) -> i32 {
        match self {
            KeyCode::Z => 501,
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

pub fn create_simulator(_method: &str) -> Result<Box<dyn KeySimulator>> {
    Ok(Box::new(DdSimulator::new()?))
}
