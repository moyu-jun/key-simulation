use anyhow::Result;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

use super::{KeyCode, KeySimulator};

pub struct KeybdEventSimulator;

impl KeySimulator for KeybdEventSimulator {
    fn key_down(&self, key: KeyCode) -> Result<()> {
        unsafe {
            keybd_event(key.virtual_key() as u8, key.scan_code() as u8, KEYBD_EVENT_FLAGS(0), 0);
        }
        Ok(())
    }

    fn key_up(&self, key: KeyCode) -> Result<()> {
        unsafe {
            keybd_event(key.virtual_key() as u8, key.scan_code() as u8, KEYEVENTF_KEYUP, 0);
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "keybd_event"
    }
}
