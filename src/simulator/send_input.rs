use anyhow::Result;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

use super::{KeyCode, KeySimulator};

pub struct SendInputSimulator;

impl KeySimulator for SendInputSimulator {
    fn key_down(&self, key: KeyCode) -> Result<()> {
        let input = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(key.virtual_key()),
                    wScan: key.scan_code(),
                    dwFlags: KEYBD_EVENT_FLAGS(0),
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
                    wVk: VIRTUAL_KEY(key.virtual_key()),
                    wScan: key.scan_code(),
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        unsafe { SendInput(&[input], size_of::<INPUT>() as i32) };
        Ok(())
    }

    fn name(&self) -> &str {
        "SendInput"
    }
}
