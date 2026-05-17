use anyhow::{Context, Result, bail};

use super::{KeyCode, KeySimulator};
use crate::log_msg;

const DLL_NAME: &str = "ddhid.63340.dll";

type DdKeyFn = unsafe extern "system" fn(i32, i32) -> i32;
type DdBtnFn = unsafe extern "system" fn(i32) -> i32;

const DD_KEY_DOWN: i32 = 1;
const DD_KEY_UP: i32 = 2;

pub struct DdSimulator {
    _lib: libloading::Library,
    dd_key: DdKeyFn,
}

impl DdSimulator {
    pub fn new() -> Result<Self> {
        log_msg!("[DD] 定位 .exe 所在目录...");
        let dll_path = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.join(DLL_NAME)))
            .context("无法定位 .exe 所在目录")?;

        log_msg!("[DD] DLL 完整路径: {}", dll_path.display());

        if !dll_path.exists() {
            bail!(
                "未找到 {}，请将 {} 放在 .exe 同目录下",
                dll_path.display(),
                DLL_NAME
            );
        }

        log_msg!("[DD] 开始加载 DLL...");
        let lib = unsafe { libloading::Library::new(&dll_path) }
            .with_context(|| format!("加载 {} 失败", dll_path.display()))?;
        log_msg!("[DD] DLL 加载成功");

        unsafe {
            log_msg!("[DD] 查找 DD_btn 函数...");
            let dd_btn: DdBtnFn = *lib
                .get(b"DD_btn\0")
                .with_context(|| format!("{} 中未找到 DD_btn 函数", DLL_NAME))?;

            log_msg!("[DD] 查找 DD_key 函数...");
            let dd_key: DdKeyFn = *lib
                .get(b"DD_key\0")
                .with_context(|| format!("{} 中未找到 DD_key 函数", DLL_NAME))?;

            log_msg!("[DD] 调用 DD_btn(0) 初始化驱动...");
            let init_ret = dd_btn(0);
            log_msg!("[DD] DD_btn(0) 返回值: {}", init_ret);

            if init_ret != 1 {
                bail!(
                    "DD 驱动初始化失败 (DD_btn(0) 返回 {})。可能原因：\n  1. DD 免费版需要联网验证，请检查网络\n  2. 驱动加载失败，请确认是管理员权限\n  3. 杀毒软件拦截",
                    init_ret
                );
            }

            log_msg!("[DD] 驱动初始化成功");
            Ok(Self {
                _lib: lib,
                dd_key,
            })
        }
    }
}

impl KeySimulator for DdSimulator {
    fn key_down(&self, key: KeyCode) -> Result<()> {
        let dd_code = key.dd_code();
        let ret = unsafe { (self.dd_key)(dd_code, DD_KEY_DOWN) };
        if ret != 1 {
            bail!("DD_key 按下返回错误: {}", ret);
        }
        Ok(())
    }

    fn key_up(&self, key: KeyCode) -> Result<()> {
        let dd_code = key.dd_code();
        let ret = unsafe { (self.dd_key)(dd_code, DD_KEY_UP) };
        if ret != 1 {
            bail!("DD_key 释放返回错误: {}", ret);
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "DD HID 驱动 (ddhid.63340.dll)"
    }
}
