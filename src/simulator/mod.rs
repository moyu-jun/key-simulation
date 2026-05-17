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

mod send_input;
mod scan_code;
mod keybd_event;

pub use send_input::SendInputSimulator;
pub use scan_code::ScanCodeSimulator;
pub use keybd_event::KeybdEventSimulator;

pub fn create_simulator(method: &str) -> Box<dyn KeySimulator> {
    match method {
        "scan_code" => Box::new(ScanCodeSimulator),
        "keybd_event" => Box::new(KeybdEventSimulator),
        _ => Box::new(SendInputSimulator),
    }
}
