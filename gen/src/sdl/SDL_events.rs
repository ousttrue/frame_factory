// this is generated.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_parens)]
use std::ffi::c_void;
extern crate va_list;
use super::*;

pub const SDL_RELEASED: i32 = 0;
pub const SDL_PRESSED: i32 = 1;
//SDL_TEXTEDITINGEVENT_TEXT_SIZE (32)
//SDL_TEXTINPUTEVENT_TEXT_SIZE (32)
//SDL_QUERY -1
pub const SDL_IGNORE: i32 = 0;
pub const SDL_DISABLE: i32 = 0;
pub const SDL_ENABLE: i32 = 1;
/* SDL_GetEventState(type)SDL_EventState(type,SDL_QUERY) */
pub const SDL_FIRSTEVENT: i32 = 0;
pub const SDL_QUIT: i32 = 0x100;
pub const SDL_APP_TERMINATING: i32 = 0x101;
pub const SDL_APP_LOWMEMORY: i32 = 0x102;
pub const SDL_APP_WILLENTERBACKGROUND: i32 = 0x103;
pub const SDL_APP_DIDENTERBACKGROUND: i32 = 0x104;
pub const SDL_APP_WILLENTERFOREGROUND: i32 = 0x105;
pub const SDL_APP_DIDENTERFOREGROUND: i32 = 0x106;
pub const SDL_LOCALECHANGED: i32 = 0x107;
pub const SDL_DISPLAYEVENT: i32 = 0x150;
pub const SDL_WINDOWEVENT: i32 = 0x200;
pub const SDL_SYSWMEVENT: i32 = 0x201;
pub const SDL_KEYDOWN: i32 = 0x300;
pub const SDL_KEYUP: i32 = 0x301;
pub const SDL_TEXTEDITING: i32 = 0x302;
pub const SDL_TEXTINPUT: i32 = 0x303;
pub const SDL_KEYMAPCHANGED: i32 = 0x304;
pub const SDL_MOUSEMOTION: i32 = 0x400;
pub const SDL_MOUSEBUTTONDOWN: i32 = 0x401;
pub const SDL_MOUSEBUTTONUP: i32 = 0x402;
pub const SDL_MOUSEWHEEL: i32 = 0x403;
pub const SDL_JOYAXISMOTION: i32 = 0x600;
pub const SDL_JOYBALLMOTION: i32 = 0x601;
pub const SDL_JOYHATMOTION: i32 = 0x602;
pub const SDL_JOYBUTTONDOWN: i32 = 0x603;
pub const SDL_JOYBUTTONUP: i32 = 0x604;
pub const SDL_JOYDEVICEADDED: i32 = 0x605;
pub const SDL_JOYDEVICEREMOVED: i32 = 0x606;
pub const SDL_CONTROLLERAXISMOTION: i32 = 0x650;
pub const SDL_CONTROLLERBUTTONDOWN: i32 = 0x651;
pub const SDL_CONTROLLERBUTTONUP: i32 = 0x652;
pub const SDL_CONTROLLERDEVICEADDED: i32 = 0x653;
pub const SDL_CONTROLLERDEVICEREMOVED: i32 = 0x654;
pub const SDL_CONTROLLERDEVICEREMAPPED: i32 = 0x655;
pub const SDL_CONTROLLERTOUCHPADDOWN: i32 = 0x656;
pub const SDL_CONTROLLERTOUCHPADMOTION: i32 = 0x657;
pub const SDL_CONTROLLERTOUCHPADUP: i32 = 0x658;
pub const SDL_CONTROLLERSENSORUPDATE: i32 = 0x659;
pub const SDL_FINGERDOWN: i32 = 0x700;
pub const SDL_FINGERUP: i32 = 0x701;
pub const SDL_FINGERMOTION: i32 = 0x702;
pub const SDL_DOLLARGESTURE: i32 = 0x800;
pub const SDL_DOLLARRECORD: i32 = 0x801;
pub const SDL_MULTIGESTURE: i32 = 0x802;
pub const SDL_CLIPBOARDUPDATE: i32 = 0x900;
pub const SDL_DROPFILE: i32 = 0x1000;
pub const SDL_DROPTEXT: i32 = 0x1001;
pub const SDL_DROPBEGIN: i32 = 0x1002;
pub const SDL_DROPCOMPLETE: i32 = 0x1003;
pub const SDL_AUDIODEVICEADDED: i32 = 0x1100;
pub const SDL_AUDIODEVICEREMOVED: i32 = 0x1101;
pub const SDL_SENSORUPDATE: i32 = 0x1200;
pub const SDL_RENDER_TARGETS_RESET: i32 = 0x2000;
pub const SDL_RENDER_DEVICE_RESET: i32 = 0x2001;
pub const SDL_USEREVENT: i32 = 0x8000;
pub const SDL_LASTEVENT: i32 = 0xffff;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SDL_CommonEvent {
    pub r#type: u32,
    pub timestamp: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SDL_DisplayEvent {
    pub r#type: u32,
    pub timestamp: u32,
    pub display: u32,
    pub event: u8,
    pub padding1: u8,
    pub padding2: u8,
    pub padding3: u8,
    pub data1: i32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SDL_WindowEvent {
    pub r#type: u32,
    pub timestamp: u32,
    pub windowID: u32,
    pub event: u8,
    pub padding1: u8,
    pub padding2: u8,
    pub padding3: u8,
    pub data1: i32,
    pub data2: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_KeyboardEvent {
    pub r#type: u32,
    pub timestamp: u32,
    pub windowID: u32,
    pub state: u8,
    pub repeat: u8,
    pub padding2: u8,
    pub padding3: u8,
    pub keysym: SDL_Keysym,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_TextEditingEvent {
    pub r#type: u32,
    pub timestamp: u32,
    pub windowID: u32,
    pub text: [i8; 32],
    pub start: i32,
    pub length: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_TextInputEvent {
    pub r#type: u32,
    pub timestamp: u32,
    pub windowID: u32,
    pub text: [i8; 32],
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SDL_MouseMotionEvent {
    pub r#type: u32,
    pub timestamp: u32,
    pub windowID: u32,
    pub which: u32,
    pub state: u32,
    pub x: i32,
    pub y: i32,
    pub xrel: i32,
    pub yrel: i32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SDL_MouseButtonEvent {
    pub r#type: u32,
    pub timestamp: u32,
    pub windowID: u32,
    pub which: u32,
    pub button: u8,
    pub state: u8,
    pub clicks: u8,
    pub padding1: u8,
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SDL_MouseWheelEvent {
    pub r#type: u32,
    pub timestamp: u32,
    pub windowID: u32,
    pub which: u32,
    pub x: i32,
    pub y: i32,
    pub direction: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SDL_JoyAxisEvent {
    pub r#type: u32,
    pub timestamp: u32,
    pub which: i32,
    pub axis: u8,
    pub padding1: u8,
    pub padding2: u8,
    pub padding3: u8,
    pub value: i16,
    pub padding4: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SDL_JoyBallEvent {
    pub r#type: u32,
    pub timestamp: u32,
    pub which: i32,
    pub ball: u8,
    pub padding1: u8,
    pub padding2: u8,
    pub padding3: u8,
    pub xrel: i16,
    pub yrel: i16,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SDL_JoyHatEvent {
    pub r#type: u32,
    pub timestamp: u32,
    pub which: i32,
    pub hat: u8,
    pub value: u8,
    pub padding1: u8,
    pub padding2: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SDL_JoyButtonEvent {
    pub r#type: u32,
    pub timestamp: u32,
    pub which: i32,
    pub button: u8,
    pub state: u8,
    pub padding1: u8,
    pub padding2: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SDL_JoyDeviceEvent {
    pub r#type: u32,
    pub timestamp: u32,
    pub which: i32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SDL_ControllerAxisEvent {
    pub r#type: u32,
    pub timestamp: u32,
    pub which: i32,
    pub axis: u8,
    pub padding1: u8,
    pub padding2: u8,
    pub padding3: u8,
    pub value: i16,
    pub padding4: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SDL_ControllerButtonEvent {
    pub r#type: u32,
    pub timestamp: u32,
    pub which: i32,
    pub button: u8,
    pub state: u8,
    pub padding1: u8,
    pub padding2: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SDL_ControllerDeviceEvent {
    pub r#type: u32,
    pub timestamp: u32,
    pub which: i32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SDL_ControllerTouchpadEvent {
    pub r#type: u32,
    pub timestamp: u32,
    pub which: i32,
    pub touchpad: i32,
    pub finger: i32,
    pub x: f32,
    pub y: f32,
    pub pressure: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_ControllerSensorEvent {
    pub r#type: u32,
    pub timestamp: u32,
    pub which: i32,
    pub sensor: i32,
    pub data: [f32; 3],
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SDL_AudioDeviceEvent {
    pub r#type: u32,
    pub timestamp: u32,
    pub which: u32,
    pub iscapture: u8,
    pub padding1: u8,
    pub padding2: u8,
    pub padding3: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SDL_TouchFingerEvent {
    pub r#type: u32,
    pub timestamp: u32,
    pub touchId: i64,
    pub fingerId: i64,
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub pressure: f32,
    pub windowID: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SDL_MultiGestureEvent {
    pub r#type: u32,
    pub timestamp: u32,
    pub touchId: i64,
    pub dTheta: f32,
    pub dDist: f32,
    pub x: f32,
    pub y: f32,
    pub numFingers: u16,
    pub padding: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SDL_DollarGestureEvent {
    pub r#type: u32,
    pub timestamp: u32,
    pub touchId: i64,
    pub gestureId: i64,
    pub numFingers: u32,
    pub error: f32,
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_DropEvent {
    pub r#type: u32,
    pub timestamp: u32,
    pub file: *mut i8,
    pub windowID: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_SensorEvent {
    pub r#type: u32,
    pub timestamp: u32,
    pub which: i32,
    pub data: [f32; 6],
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SDL_QuitEvent {
    pub r#type: u32,
    pub timestamp: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SDL_OSEvent {
    pub r#type: u32,
    pub timestamp: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_UserEvent {
    pub r#type: u32,
    pub timestamp: u32,
    pub windowID: u32,
    pub code: i32,
    pub data1: *mut c_void,
    pub data2: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_SysWMEvent {
    pub r#type: u32,
    pub timestamp: u32,
    pub msg: *mut SDL_SysWMmsg,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_Event {
    pub r#type: u32,
    pub common: SDL_CommonEvent,
    pub display: SDL_DisplayEvent,
    pub window: SDL_WindowEvent,
    pub key: SDL_KeyboardEvent,
    pub edit: SDL_TextEditingEvent,
    pub text: SDL_TextInputEvent,
    pub motion: SDL_MouseMotionEvent,
    pub button: SDL_MouseButtonEvent,
    pub wheel: SDL_MouseWheelEvent,
    pub jaxis: SDL_JoyAxisEvent,
    pub jball: SDL_JoyBallEvent,
    pub jhat: SDL_JoyHatEvent,
    pub jbutton: SDL_JoyButtonEvent,
    pub jdevice: SDL_JoyDeviceEvent,
    pub caxis: SDL_ControllerAxisEvent,
    pub cbutton: SDL_ControllerButtonEvent,
    pub cdevice: SDL_ControllerDeviceEvent,
    pub ctouchpad: SDL_ControllerTouchpadEvent,
    pub csensor: SDL_ControllerSensorEvent,
    pub adevice: SDL_AudioDeviceEvent,
    pub sensor: SDL_SensorEvent,
    pub quit: SDL_QuitEvent,
    pub user: SDL_UserEvent,
    pub syswm: SDL_SysWMEvent,
    pub tfinger: SDL_TouchFingerEvent,
    pub mgesture: SDL_MultiGestureEvent,
    pub dgesture: SDL_DollarGestureEvent,
    pub drop: SDL_DropEvent,
    pub padding: [u8; 56],
}
pub const SDL_ADDEVENT: i32 = 0;
pub const SDL_PEEKEVENT: i32 = 0x1;
pub const SDL_GETEVENT: i32 = 0x2;

#[link(name = "SDL2", kind = "static")]
extern "C" {

    pub fn SDL_PumpEvents() -> c_void;

    /// * events: 
    /// * numevents: 
    /// * action: 
    /// * minType: 
    /// * maxType: 
    pub fn SDL_PeepEvents(
        events: *mut SDL_Event,
        numevents: i32,
        action: i32,
        minType: u32,
        maxType: u32,
    ) -> i32;

    /// * type: 
    pub fn SDL_HasEvent(
        r#type: u32,
    ) -> i32;

    /// * minType: 
    /// * maxType: 
    pub fn SDL_HasEvents(
        minType: u32,
        maxType: u32,
    ) -> i32;

    /// * type: 
    pub fn SDL_FlushEvent(
        r#type: u32,
    ) -> c_void;

    /// * minType: 
    /// * maxType: 
    pub fn SDL_FlushEvents(
        minType: u32,
        maxType: u32,
    ) -> c_void;

    /// * event: 
    pub fn SDL_PollEvent(
        event: *mut SDL_Event,
    ) -> i32;

    /// * event: 
    pub fn SDL_WaitEvent(
        event: *mut SDL_Event,
    ) -> i32;

    /// * event: 
    /// * timeout: 
    pub fn SDL_WaitEventTimeout(
        event: *mut SDL_Event,
        timeout: i32,
    ) -> i32;

    /// * event: 
    pub fn SDL_PushEvent(
        event: *mut SDL_Event,
    ) -> i32;

    /// * filter: 
    /// * userdata: 
    pub fn SDL_SetEventFilter(
        filter: extern fn(*mut c_void,*mut SDL_Event,) -> i32,
        userdata: *mut c_void,
    ) -> c_void;

    /// * filter: 
    /// * userdata: 
    pub fn SDL_GetEventFilter(
        filter: *mut extern fn(*mut c_void,*mut SDL_Event,) -> i32,
        userdata: *mut *mut c_void,
    ) -> i32;

    /// * filter: 
    /// * userdata: 
    pub fn SDL_AddEventWatch(
        filter: extern fn(*mut c_void,*mut SDL_Event,) -> i32,
        userdata: *mut c_void,
    ) -> c_void;

    /// * filter: 
    /// * userdata: 
    pub fn SDL_DelEventWatch(
        filter: extern fn(*mut c_void,*mut SDL_Event,) -> i32,
        userdata: *mut c_void,
    ) -> c_void;

    /// * filter: 
    /// * userdata: 
    pub fn SDL_FilterEvents(
        filter: extern fn(*mut c_void,*mut SDL_Event,) -> i32,
        userdata: *mut c_void,
    ) -> c_void;

    /// * type: 
    /// * state: 
    pub fn SDL_EventState(
        r#type: u32,
        state: i32,
    ) -> u8;

    /// * numevents: 
    pub fn SDL_RegisterEvents(
        numevents: i32,
    ) -> u32;
}
