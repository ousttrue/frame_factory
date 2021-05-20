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

impl MouseButtonFlags {
    pub fn has(self, flag: MouseButtonFlags) -> bool {
        let value = self;
        (value as u32 & flag as u32) != 0
    }
}

#[repr(C)]
pub struct ScreenState {
    pub width: i16,
    pub height: i16,
    pub elapsed_seconds: f32,
    pub delta_seconds: f32,
    pub mouse_x: i16,
    pub mouse_y: i16,
    pub mouse_flag: MouseButtonFlags,
}
