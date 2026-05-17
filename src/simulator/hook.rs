use anyhow::Result;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Foundation::HINSTANCE;

use super::{KeyCode, KeySimulator};

pub struct HookSimulator;

impl KeySimulator for HookSimulator {
    fn key_down(&self, key: KeyCode) -> Result<()> {
        unsafe {
            let hook = SetWindowsHookExW(
                WH_KEYBOARD_LL,
                None,
                HINSTANCE::default(),
                0,
            );
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
            SendInput(&[input], size_of::<INPUT>() as i32);
            if let Ok(h) = hook {
                let _ = UnhookWindowsHookEx(h);
            }
        }
        Ok(())
    }

    fn key_up(&self, key: KeyCode) -> Result<()> {
        unsafe {
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
            SendInput(&[input], size_of::<INPUT>() as i32);
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "Hook (WH_KEYBOARD_LL + ScanCode)"
    }
}
