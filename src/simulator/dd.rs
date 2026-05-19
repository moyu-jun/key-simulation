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
        log_msg!("[DD] DLL 加载成功");

        unsafe {
            log_msg!("[DD] 解析导出函数...");
            let btn_fn: DDBtnFn = *lib.get(b"DD_btn\0").context("未找到导出函数 DD_btn")?;
            let key_fn: DDKeyFn = *lib.get(b"DD_key\0").context("未找到导出函数 DD_key")?;

            // DD_todc 可选，仅用于打印 VK 到 DD code 的映射，便于排查 dd_code 是否正确
            if let Ok(todc_sym) = lib.get::<DDTodcFn>(b"DD_todc\0") {
                let todc_fn: DDTodcFn = *todc_sym;
                let vk_z: i32 = 0x5A;
                let dd_z = todc_fn(vk_z);
                log_msg!(
                    "[DD] DD_todc(VK_Z=0x5A) -> {} (硬编码值: {})",
                    dd_z,
                    KeyCode::Z.dd_code()
                );
            }

            // DD_btn(0) 在官方约定里是 no-op，仅用作"DLL 可调用"的最小验证；
            // 其返回值不能作为驱动是否生效的判据，真正生效与否要看 DD_key 的返回值。
            log_msg!("[DD] 调用 DD_btn(0) 探测 DLL...");
            let probe = btn_fn(0);
            log_msg!("[DD] DD_btn(0) 返回 {} (no-op，仅参考)", probe);

            Ok(Self {
                key_fn,
                _lib: lib,
            })
        }
    }
}

impl KeySimulator for DDSimulator {
    fn key_down(&self, key: KeyCode) -> Result<()> {
        let ret = unsafe { (self.key_fn)(key.dd_code(), DD_KEY_DOWN) };
        if ret != 1 {
            bail!(
                "DD_key 按下返回 {}，期望 1。可能原因：缺少管理员权限 / 驱动未加载 / 杀软拦截",
                ret
            );
        }
        Ok(())
    }

    fn key_up(&self, key: KeyCode) -> Result<()> {
        let ret = unsafe { (self.key_fn)(key.dd_code(), DD_KEY_UP) };
        if ret != 1 {
            bail!(
                "DD_key 释放返回 {}，期望 1。可能原因：缺少管理员权限 / 驱动未加载 / 杀软拦截",
                ret
            );
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "DD 驱动 (dd63330.dll)"
    }
}
