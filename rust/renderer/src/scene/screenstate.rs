use std::ops::Deref;

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
        value as u32 & flag as u32 != 0
    }
}

#[repr(C)]
pub struct ScreenState {
    pub Width: i16,
    pub Height: i16,
    pub ElapsedSeconds: f32,
    pub DeltaSeconds: f32,
    pub MouseX: i16,
    pub MouseY: i16,
    pub MouseFlag: MouseButtonFlags,
}
