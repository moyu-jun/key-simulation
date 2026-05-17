use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub enum KeyCode {
    Z,
}

impl KeyCode {
    pub fn virtual_key(self) -> u16 {
        match self {
            KeyCode::Z => 0x5A,
        }
    }

    pub fn scan_code(self) -> u16 {
        match self {
            KeyCode::Z => 0x2C,
        }
    }

    pub fn unicode_char(self) -> u16 {
        match self {
            KeyCode::Z => 'z' as u16,
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

mod pynput_vk;
mod pynput_unicode;

pub use pynput_vk::PynputVkSimulator;
pub use pynput_unicode::PynputUnicodeSimulator;

pub fn create_simulator(method: &str) -> Box<dyn KeySimulator> {
    match method {
        "unicode" => Box::new(PynputUnicodeSimulator),
        _ => Box::new(PynputVkSimulator),
    }
}
