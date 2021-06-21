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
use super::SDL_version::SDL_version;
pub const SDL_SYSWM_UNKNOWN: i32 = 0;
pub const SDL_SYSWM_WINDOWS: i32 = 0x1;
pub const SDL_SYSWM_X11: i32 = 0x2;
pub const SDL_SYSWM_DIRECTFB: i32 = 0x3;
pub const SDL_SYSWM_COCOA: i32 = 0x4;
pub const SDL_SYSWM_UIKIT: i32 = 0x5;
pub const SDL_SYSWM_WAYLAND: i32 = 0x6;
pub const SDL_SYSWM_MIR: i32 = 0x7;
pub const SDL_SYSWM_WINRT: i32 = 0x8;
pub const SDL_SYSWM_ANDROID: i32 = 0x9;
pub const SDL_SYSWM_VIVANTE: i32 = 0xa;
pub const SDL_SYSWM_OS2: i32 = 0xb;
pub const SDL_SYSWM_HAIKU: i32 = 0xc;
pub const SDL_SYSWM_KMSDRM: i32 = 0xd;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_SysWMmsg {
    pub version: SDL_version,
    pub subsystem: i32,
    pub msg: anonymous_0,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union anonymous_0 {
    pub win: anonymous_1,
    pub dummy: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct anonymous_1 {
    pub hwnd: *mut c_void,
    pub msg: u32,
    pub wParam: u64,
    pub lParam: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_SysWMinfo {
    pub version: SDL_version,
    pub subsystem: i32,
    pub info: anonymous_2,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union anonymous_2 {
    pub win: anonymous_3,
    pub dummy: [u8; 64],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct anonymous_3 {
    pub window: *mut c_void,
    pub hdc: *mut c_void,
    pub hinstance: *mut c_void,
}

#[link(name = "SDL2", kind = "static")]
extern "C" {

    /// * window: 
    /// * info: 
    pub fn SDL_GetWindowWMInfo_REAL(
        window: *mut SDL_Window,
        info: *mut SDL_SysWMinfo,
    ) -> i32;
}
