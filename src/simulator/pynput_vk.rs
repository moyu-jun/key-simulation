use anyhow::Result;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

use super::{KeyCode, KeySimulator};

/// 模拟 pynput 的 press(Key.z) 路径：
/// 使用虚拟键码 + MapVirtualKey 动态获取扫描码
pub struct PynputVkSimulator;

impl KeySimulator for PynputVkSimulator {
    fn key_down(&self, key: KeyCode) -> Result<()> {
        let vk = key.virtual_key();
        let scan = unsafe { MapVirtualKeyW(vk as u32, MAP_VIRTUAL_KEY_TYPE(0)) } as u16;
        let input = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(vk),
                    wScan: scan,
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
        let vk = key.virtual_key();
        let scan = unsafe { MapVirtualKeyW(vk as u32, MAP_VIRTUAL_KEY_TYPE(0)) } as u16;
        let input = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(vk),
                    wScan: scan,
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
        "pynput_vk (Key.z 路径)"
    }
}
