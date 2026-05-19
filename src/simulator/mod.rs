use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub enum KeyCode {
    Z,
}

impl KeyCode {
    /// 对应的 Windows 虚拟键码 (VK_*)，运行时通过 DD_todc 转成 DD 专用键码。
    pub fn vk_code(self) -> i32 {
        match self {
            KeyCode::Z => 0x5A,
        }
    }

    /// DD 官方虚拟键盘码表中的硬编码值，作为 DD_todc 调用失败时的兜底。
    /// Z 在 ZXCV 行 -> 501（注意：401 是 A，不是 Z）。
    pub fn dd_code_fallback(self) -> i32 {
        match self {
            KeyCode::Z => 501,
        }
    }
}

pub trait KeySimulator {
    fn key_down(&self, key: KeyCode) -> Result<()>;
    fn key_up(&self, key: KeyCode) -> Result<()>;

    fn key_press(&self, key: KeyCode) -> Result<()> {
        // 按官方 Python 样例：down 与 up 之间无需间隔，且即使 down 失败也要尝试 up，
        // 避免应用层任何分支让按键卡在按下状态。
        let down_res = self.key_down(key);
        let up_res = self.key_up(key);
        down_res.and(up_res)
    }

    fn name(&self) -> &str;
}

mod dd;

pub use dd::DDSimulator;

pub fn create_simulator() -> Result<Box<dyn KeySimulator>> {
    Ok(Box::new(DDSimulator::new()?))
}
