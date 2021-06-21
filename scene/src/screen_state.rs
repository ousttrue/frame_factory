#[repr(u32)]
#[derive(Clone, Copy)]
pub enum MouseButtonFlags {
    None = 0,
    LeftDown = 0x01,
    RightDown = 0x02,
    MiddleDown = 0x04,
    WheelPlus = 0x08,
    WheelMinus = 0x10,
    CursorUpdate = 0x20,
}

impl Default for MouseButtonFlags {
    fn default() -> Self {
        MouseButtonFlags::None
    }
}

impl MouseButtonFlags {
    pub fn has(self, flag: MouseButtonFlags) -> bool {
        let value = self;
        (value as u32 & flag as u32) != 0
    }
}

#[repr(C)]
#[derive(Default)]
pub struct ScreenState {
    pub width: i16,
    pub height: i16,
    // pub elapsed_seconds: f32,
    // pub delta_seconds: f32,
    pub mouse_x: i16,
    pub mouse_y: i16,
    pub mouse_flag: MouseButtonFlags,
}

impl ScreenState {
    pub fn crop(&self, x: i16, y: i16, w: i16, h: i16) -> ScreenState {
        ScreenState {
            width: w,
            height: h,
            mouse_x: self.mouse_x - x,
            mouse_y: self.mouse_y - y,
            ..*self
        }
    }
}
