

bitflags! {
    pub struct MouseButtonFlags: u32 {
        const None = 0;
        const LeftDown = 0x01;
        const RightDown = 0x02;
        const MiddleDown = 0x04;
        const WheelPlus = 0x08;
        const WheelMinus = 0x10;
        const CursorUpdate = 0x20;
    }
}

impl Default for MouseButtonFlags {
    fn default() -> Self {
        MouseButtonFlags::None
    }
}

// impl MouseButtonFlags {
//     pub fn has(self, flag: MouseButtonFlags) -> bool {
//         let value = self;
//         (value as u32 & flag as u32) != 0
//     }
// }

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
    pub fn new(
        width: i16,
        height: i16,
        mouse_x: i16,
        mouse_y: i16,
        button_left: bool,
        button_middle: bool,
        button_right: bool,
        wheel: i32,
    ) -> ScreenState {
        let mut mouse_flag = Default::default();
        if button_left {
            mouse_flag |= MouseButtonFlags::LeftDown;
        }
        if button_middle {
            mouse_flag |= MouseButtonFlags::MiddleDown;
        }
        if button_right {
            mouse_flag |= MouseButtonFlags::RightDown;
        }
        if wheel > 0 {
            mouse_flag |= MouseButtonFlags::WheelPlus;
        } else if wheel < 0 {
            mouse_flag |= MouseButtonFlags::WheelMinus;
        }

        ScreenState {
            width,
            height,
            mouse_x,
            mouse_y,
            mouse_flag,
        }
    }

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
