use anyhow::Result;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

use super::{KeyCode, KeySimulator};

pub struct ScanCodeSimulator;

impl KeySimulator for ScanCodeSimulator {
    fn key_down(&self, key: KeyCode) -> Result<()> {
        let input = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: key.scan_code(),
                    dwFlags: KEYEVENTF_SCANCODE,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        unsafe { SendInput(&[input], size_of::<INPUT>() as i32) };
        Ok(())
    }

    fn key_up(&self, key: KeyCode) -> Result<()> {
        let input = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: key.scan_code(),
                    dwFlags: KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        unsafe { SendInput(&[input], size_of::<INPUT>() as i32) };
        Ok(())
    }

    fn name(&self) -> &str {
        "ScanCode"
    }
}
