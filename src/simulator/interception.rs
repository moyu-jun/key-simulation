use anyhow::{Result, bail};
use std::ffi::c_void;

use super::{KeyCode, KeySimulator};

const INTERCEPTION_KEY_DOWN: u16 = 0x00;
const INTERCEPTION_KEY_UP: u16 = 0x01;

#[repr(C)]
struct KeyStroke {
    code: u16,
    state: u16,
    information: u32,
}

type Context = *mut c_void;
type Device = i32;

type CreateContextFn = unsafe extern "C" fn() -> Context;
type DestroyContextFn = unsafe extern "C" fn(Context);
type SetFilterFn = unsafe extern "C" fn(Context, unsafe extern "C" fn(Device) -> i32, i32);
type SendFn = unsafe extern "C" fn(Context, Device, *const c_void, u32) -> i32;

unsafe extern "C" fn is_keyboard(device: Device) -> i32 {
    if (1..=10).contains(&device) { 1 } else { 0 }
}

pub struct InterceptionSimulator {
    ctx: Context,
    device: Device,
    _lib: libloading::Library,
    send_fn: SendFn,
    destroy_fn: DestroyContextFn,
}

impl InterceptionSimulator {
    pub fn new() -> Self {
        let lib = unsafe { libloading::Library::new("interception.dll") }
            .expect("无法加载 interception.dll，请确保已安装 Interception 驱动");

        unsafe {
            let create: CreateContextFn = *lib.get(b"interception_create_context\0").unwrap();
            let set_filter: SetFilterFn = *lib.get(b"interception_set_filter\0").unwrap();
            let send_fn: SendFn = *lib.get(b"interception_send\0").unwrap();
            let destroy_fn: DestroyContextFn =
                *lib.get(b"interception_destroy_context\0").unwrap();

            let ctx = create();
            set_filter(ctx, is_keyboard, 0xFFFF_u16 as i32);

            Self {
                ctx,
                device: 1,
                _lib: lib,
                send_fn,
                destroy_fn,
            }
        }
    }
}

impl Drop for InterceptionSimulator {
    fn drop(&mut self) {
        unsafe { (self.destroy_fn)(self.ctx) };
    }
}

impl KeySimulator for InterceptionSimulator {
    fn key_down(&self, key: KeyCode) -> Result<()> {
        let stroke = KeyStroke {
            code: key.scan_code(),
            state: INTERCEPTION_KEY_DOWN,
            information: 0,
        };
        let ret = unsafe {
            (self.send_fn)(
                self.ctx,
                self.device,
                &stroke as *const KeyStroke as *const c_void,
                1,
            )
        };
        if ret == 0 {
            bail!("interception_send 失败");
        }
        Ok(())
    }

    fn key_up(&self, key: KeyCode) -> Result<()> {
        let stroke = KeyStroke {
            code: key.scan_code(),
            state: INTERCEPTION_KEY_UP,
            information: 0,
        };
        let ret = unsafe {
            (self.send_fn)(
                self.ctx,
                self.device,
                &stroke as *const KeyStroke as *const c_void,
                1,
            )
        };
        if ret == 0 {
            bail!("interception_send 失败");
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "Interception (驱动级)"
    }
}
