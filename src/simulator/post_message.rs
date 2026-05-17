use anyhow::{Result, Context};
use windows::Win32::Foundation::{HWND, WPARAM, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;

use super::{KeyCode, KeySimulator};

pub struct PostMessageSimulator;

impl KeySimulator for PostMessageSimulator {
    fn key_down(&self, key: KeyCode) -> Result<()> {
        let hwnd = unsafe { GetForegroundWindow() };
        if hwnd == HWND::default() {
            anyhow::bail!("无法获取前台窗口");
        }
        let vk = key.virtual_key() as usize;
        let scan = key.scan_code() as u32;
        let lparam = (1u32 | (scan << 16)) as isize;
        unsafe {
            PostMessageW(hwnd, WM_KEYDOWN, WPARAM(vk), LPARAM(lparam))
                .context("PostMessage WM_KEYDOWN 失败")?;
        }
        Ok(())
    }

    fn key_up(&self, key: KeyCode) -> Result<()> {
        let hwnd = unsafe { GetForegroundWindow() };
        if hwnd == HWND::default() {
            anyhow::bail!("无法获取前台窗口");
        }
        let vk = key.virtual_key() as usize;
        let scan = key.scan_code() as u32;
        let lparam = (1u32 | (scan << 16) | (3u32 << 30)) as isize;
        unsafe {
            PostMessageW(hwnd, WM_KEYUP, WPARAM(vk), LPARAM(lparam))
                .context("PostMessage WM_KEYUP 失败")?;
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "PostMessage"
    }
}
