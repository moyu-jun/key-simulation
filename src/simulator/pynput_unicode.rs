use anyhow::Result;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

use super::{KeyCode, KeySimulator};

/// 模拟 pynput 的 press('z') 路径：
/// 使用 KEYEVENTF_UNICODE 标志，wVk=0，wScan=字符的 Unicode 值
pub struct PynputUnicodeSimulator;

impl KeySimulator for PynputUnicodeSimulator {
    fn key_down(&self, key: KeyCode) -> Result<()> {
        let input = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: key.unicode_char(),
                    dwFlags: KEYEVENTF_UNICODE,
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
                    wScan: key.unicode_char(),
                    dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        unsafe { SendInput(&[input], size_of::<INPUT>() as i32) };
        Ok(())
    }

    fn name(&self) -> &str {
        "pynput_unicode (press('z') 路径)"
    }
}
