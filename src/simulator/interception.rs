use anyhow::{Context, Result, bail};

use super::{KeyCode, KeySimulator};
use crate::log_msg;

const DLL_NAME: &str = "interception.dll";

type InterceptionContext = *mut std::ffi::c_void;
type InterceptionDevice = i32;

const INTERCEPTION_KEY_DOWN: u16 = 0x00;
const INTERCEPTION_KEY_UP: u16 = 0x01;
const INTERCEPTION_MAX_KEYBOARD: i32 = 10;

#[repr(C)]
struct InterceptionKeyStroke {
    code: u16,
    state: u16,
    information: u32,
}

type CreateContextFn = unsafe extern "C" fn() -> InterceptionContext;
type DestroyContextFn = unsafe extern "C" fn(InterceptionContext);
type GetHardwareIdFn = unsafe extern "C" fn(
    InterceptionContext,
    InterceptionDevice,
    *mut std::ffi::c_void,
    u32,
) -> u32;
type SendFn = unsafe extern "C" fn(
    InterceptionContext,
    InterceptionDevice,
    *const InterceptionKeyStroke,
    u32,
) -> i32;

pub struct InterceptionSimulator {
    context: InterceptionContext,
    device: InterceptionDevice,
    send: SendFn,
    destroy: DestroyContextFn,
    _lib: libloading::Library,
}

impl InterceptionSimulator {
    pub fn new() -> Result<Self> {
        log_msg!("[Interception] 定位 .exe 所在目录...");
        let dll_path = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.join(DLL_NAME)))
            .context("无法定位 .exe 所在目录")?;

        log_msg!("[Interception] DLL 完整路径: {}", dll_path.display());

        if !dll_path.exists() {
            bail!(
                "未找到 {}，请将 {} 放在 .exe 同目录下",
                dll_path.display(),
                DLL_NAME
            );
        }

        log_msg!("[Interception] 开始加载 DLL...");
        let lib = unsafe { libloading::Library::new(&dll_path) }
            .with_context(|| format!("加载 {} 失败", dll_path.display()))?;
        log_msg!("[Interception] DLL 加载成功");

        unsafe {
            log_msg!("[Interception] 解析导出函数...");
            let create: CreateContextFn = *lib
                .get(b"interception_create_context\0")
                .context("未找到 interception_create_context")?;
            let destroy: DestroyContextFn = *lib
                .get(b"interception_destroy_context\0")
                .context("未找到 interception_destroy_context")?;
            let get_hardware_id: GetHardwareIdFn = *lib
                .get(b"interception_get_hardware_id\0")
                .context("未找到 interception_get_hardware_id")?;
            let send: SendFn = *lib
                .get(b"interception_send\0")
                .context("未找到 interception_send")?;

            log_msg!("[Interception] 创建上下文...");
            let context = create();
            if context.is_null() {
                bail!(
                    "interception_create_context 返回 NULL，内核驱动未生效。请：\n  1. 从 https://github.com/oblitum/Interception/releases 下载安装包\n  2. 以管理员身份运行：install-interception.exe /install\n  3. 重启系统后再试"
                );
            }
            log_msg!("[Interception] 上下文创建成功");

            log_msg!(
                "[Interception] 探测已连接的键盘设备 (1..={}) ...",
                INTERCEPTION_MAX_KEYBOARD
            );
            let mut device: InterceptionDevice = 0;
            let mut buffer = [0u16; 256];
            for i in 1..=INTERCEPTION_MAX_KEYBOARD {
                let len = get_hardware_id(
                    context,
                    i,
                    buffer.as_mut_ptr() as *mut std::ffi::c_void,
                    (buffer.len() * 2) as u32,
                );
                if len > 0 && (len as usize) <= buffer.len() * 2 {
                    let chars = (len as usize) / 2;
                    let id = String::from_utf16_lossy(&buffer[..chars.saturating_sub(1)]);
                    log_msg!("[Interception] 设备 {} -> {}", i, id);
                    if device == 0 {
                        device = i;
                    }
                }
            }

            if device == 0 {
                destroy(context);
                bail!(
                    "未探测到任何键盘设备。可能原因：\n  1. Interception 驱动安装后未重启系统\n  2. 安装失败（杀软拦截）\n  3. 当前会话没有真实键盘连接（虚拟环境/远程桌面）"
                );
            }

            log_msg!("[Interception] 选用设备 {} 作为目标键盘", device);

            Ok(Self {
                context,
                device,
                send,
                destroy,
                _lib: lib,
            })
        }
    }
}

impl Drop for InterceptionSimulator {
    fn drop(&mut self) {
        unsafe { (self.destroy)(self.context) };
    }
}

impl KeySimulator for InterceptionSimulator {
    fn key_down(&self, key: KeyCode) -> Result<()> {
        let stroke = InterceptionKeyStroke {
            code: key.scan_code(),
            state: INTERCEPTION_KEY_DOWN,
            information: 0,
        };
        let ret = unsafe { (self.send)(self.context, self.device, &stroke, 1) };
        if ret != 1 {
            bail!("interception_send 按下返回 {}，期望 1", ret);
        }
        Ok(())
    }

    fn key_up(&self, key: KeyCode) -> Result<()> {
        let stroke = InterceptionKeyStroke {
            code: key.scan_code(),
            state: INTERCEPTION_KEY_UP,
            information: 0,
        };
        let ret = unsafe { (self.send)(self.context, self.device, &stroke, 1) };
        if ret != 1 {
            bail!("interception_send 释放返回 {}，期望 1", ret);
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "Interception 驱动 (interception.dll)"
    }
}
