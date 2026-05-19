use anyhow::{Context, Result, bail};

use super::{KeyCode, KeySimulator};
use crate::log_msg;

const DLL_NAME: &str = "dd63330.dll";

const DD_KEY_DOWN: i32 = 1;
const DD_KEY_UP: i32 = 2;

type DDBtnFn = unsafe extern "system" fn(i32) -> i32;
type DDKeyFn = unsafe extern "system" fn(i32, i32) -> i32;
type DDTodcFn = unsafe extern "system" fn(i32) -> i32;

pub struct DDSimulator {
    key_fn: DDKeyFn,
    z_dd_code: i32,
    _lib: libloading::Library,
}

impl DDSimulator {
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
        log_msg!("[DD] DLL 加载成功（驱动 dd63330.sys 应已被自动加载到内核）");

        unsafe {
            log_msg!("[DD] 解析导出函数...");
            let btn_fn: DDBtnFn = *lib.get(b"DD_btn\0").context("未找到导出函数 DD_btn")?;
            let key_fn: DDKeyFn = *lib.get(b"DD_key\0").context("未找到导出函数 DD_key")?;

            // DD_btn(0) 是官方约定的初始化/通路自检（参见 ddxoft 官方 Python 样例 DD.py）：
            //   返回 1 -> DLL ↔ 内核驱动通路正常，可以发按键
            //   返回 0 -> 通路异常，即使 .sys 在内核也不能用，直接判定失败
            log_msg!("[DD] 调用 DD_btn(0) 初始化自检...");
            let probe = btn_fn(0);
            log_msg!("[DD] DD_btn(0) 返回 {}", probe);
            if probe != 1 {
                bail!(
                    "DD_btn(0) 返回 {}，DLL 与内核驱动通路异常。常见原因：\n  1. dd63330.dll 与系统位数不匹配（应使用 64 位 DLL 配合 64 位 .exe）\n  2. 当前系统已加载了另一份 DD 驱动副本，导致版本冲突\n  3. 该版本 DLL 需要联网授权但未连通\n  4. 进程未真正以管理员权限运行",
                    probe
                );
            }

            // 优先用 DD_todc(VK) 让 DLL 自己告诉我们 DD 码，避免硬编码错位
            let z_dd_code = match lib.get::<DDTodcFn>(b"DD_todc\0") {
                Ok(sym) => {
                    let todc_fn: DDTodcFn = *sym;
                    let vk = KeyCode::Z.vk_code();
                    let code = todc_fn(vk);
                    if code > 0 {
                        log_msg!("[DD] DD_todc(VK_Z=0x{:X}) -> {} (使用此值)", vk, code);
                        code
                    } else {
                        let fallback = KeyCode::Z.dd_code_fallback();
                        log_msg!(
                            "[DD] DD_todc 返回 {}，回退到硬编码 {}",
                            code,
                            fallback
                        );
                        fallback
                    }
                }
                Err(_) => {
                    let fallback = KeyCode::Z.dd_code_fallback();
                    log_msg!("[DD] DLL 未导出 DD_todc，使用硬编码 {}", fallback);
                    fallback
                }
            };

            Ok(Self {
                key_fn,
                z_dd_code,
                _lib: lib,
            })
        }
    }

    fn dd_code_for(&self, key: KeyCode) -> i32 {
        match key {
            KeyCode::Z => self.z_dd_code,
        }
    }
}

impl KeySimulator for DDSimulator {
    fn key_down(&self, key: KeyCode) -> Result<()> {
        let code = self.dd_code_for(key);
        let ret = unsafe { (self.key_fn)(code, DD_KEY_DOWN) };
        if ret != 1 {
            bail!(
                "DD_key({}, 1) 按下返回 {}，期望 1。可能原因：DD 码与按键不匹配 / 驱动通信失败 / 缺少管理员权限",
                code,
                ret
            );
        }
        Ok(())
    }

    fn key_up(&self, key: KeyCode) -> Result<()> {
        let code = self.dd_code_for(key);
        let ret = unsafe { (self.key_fn)(code, DD_KEY_UP) };
        if ret != 1 {
            bail!(
                "DD_key({}, 2) 释放返回 {}，期望 1",
                code,
                ret
            );
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "DD 驱动 (dd63330.dll)"
    }
}
