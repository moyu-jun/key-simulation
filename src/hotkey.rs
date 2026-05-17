use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;

const VK_F10: i32 = 0x79;

pub struct HotkeyListener {
    was_pressed: bool,
}

impl HotkeyListener {
    pub fn new() -> Self {
        Self { was_pressed: false }
    }

    pub fn is_triggered(&mut self) -> bool {
        let pressed = unsafe { GetAsyncKeyState(VK_F10) } & (0x8000u16 as i16) != 0;
        if pressed && !self.was_pressed {
            self.was_pressed = true;
            return true;
        }
        if !pressed {
            self.was_pressed = false;
        }
        false
    }
}
