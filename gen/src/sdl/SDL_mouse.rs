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

pub const fn SDL_BUTTON(X: i32) -> i32{(1<<((X)-1))}
pub const SDL_BUTTON_LEFT: i32 = 1;
pub const SDL_BUTTON_MIDDLE: i32 = 2;
pub const SDL_BUTTON_RIGHT: i32 = 3;
pub const SDL_BUTTON_X1: i32 = 4;
pub const SDL_BUTTON_X2: i32 = 5;
//SDL_BUTTON_LMASK SDL_BUTTON(SDL_BUTTON_LEFT)
//SDL_BUTTON_MMASK SDL_BUTTON(SDL_BUTTON_MIDDLE)
//SDL_BUTTON_RMASK SDL_BUTTON(SDL_BUTTON_RIGHT)
//SDL_BUTTON_X1MASK SDL_BUTTON(SDL_BUTTON_X1)
//SDL_BUTTON_X2MASK SDL_BUTTON(SDL_BUTTON_X2)
pub type SDL_Cursor = c_void;
pub const SDL_SYSTEM_CURSOR_ARROW: i32 = 0;
pub const SDL_SYSTEM_CURSOR_IBEAM: i32 = 0x1;
pub const SDL_SYSTEM_CURSOR_WAIT: i32 = 0x2;
pub const SDL_SYSTEM_CURSOR_CROSSHAIR: i32 = 0x3;
pub const SDL_SYSTEM_CURSOR_WAITARROW: i32 = 0x4;
pub const SDL_SYSTEM_CURSOR_SIZENWSE: i32 = 0x5;
pub const SDL_SYSTEM_CURSOR_SIZENESW: i32 = 0x6;
pub const SDL_SYSTEM_CURSOR_SIZEWE: i32 = 0x7;
pub const SDL_SYSTEM_CURSOR_SIZENS: i32 = 0x8;
pub const SDL_SYSTEM_CURSOR_SIZEALL: i32 = 0x9;
pub const SDL_SYSTEM_CURSOR_NO: i32 = 0xa;
pub const SDL_SYSTEM_CURSOR_HAND: i32 = 0xb;
pub const SDL_NUM_SYSTEM_CURSORS: i32 = 0xc;
pub const SDL_MOUSEWHEEL_NORMAL: i32 = 0;
pub const SDL_MOUSEWHEEL_FLIPPED: i32 = 0x1;

#[link(name = "SDL2", kind = "static")]
extern "C" {

    pub fn SDL_GetMouseFocus() -> *mut SDL_Window;

    /// * x: 
    /// * y: 
    pub fn SDL_GetMouseState(
        x: *mut i32,
        y: *mut i32,
    ) -> u32;

    /// * x: 
    /// * y: 
    pub fn SDL_GetGlobalMouseState(
        x: *mut i32,
        y: *mut i32,
    ) -> u32;

    /// * x: 
    /// * y: 
    pub fn SDL_GetRelativeMouseState(
        x: *mut i32,
        y: *mut i32,
    ) -> u32;

    /// * window: 
    /// * x: 
    /// * y: 
    pub fn SDL_WarpMouseInWindow(
        window: *mut SDL_Window,
        x: i32,
        y: i32,
    ) -> c_void;

    /// * x: 
    /// * y: 
    pub fn SDL_WarpMouseGlobal(
        x: i32,
        y: i32,
    ) -> i32;

    /// * enabled: 
    pub fn SDL_SetRelativeMouseMode(
        enabled: i32,
    ) -> i32;

    /// * enabled: 
    pub fn SDL_CaptureMouse(
        enabled: i32,
    ) -> i32;

    pub fn SDL_GetRelativeMouseMode() -> i32;

    /// * data: 
    /// * mask: 
    /// * w: 
    /// * h: 
    /// * hot_x: 
    /// * hot_y: 
    pub fn SDL_CreateCursor(
        data: *const u8,
        mask: *const u8,
        w: i32,
        h: i32,
        hot_x: i32,
        hot_y: i32,
    ) -> *mut SDL_Cursor;

    /// * surface: 
    /// * hot_x: 
    /// * hot_y: 
    pub fn SDL_CreateColorCursor(
        surface: *mut SDL_Surface,
        hot_x: i32,
        hot_y: i32,
    ) -> *mut SDL_Cursor;

    /// * id: 
    pub fn SDL_CreateSystemCursor(
        id: i32,
    ) -> *mut SDL_Cursor;

    /// * cursor: 
    pub fn SDL_SetCursor(
        cursor: *mut SDL_Cursor,
    ) -> c_void;

    pub fn SDL_GetCursor() -> *mut SDL_Cursor;

    pub fn SDL_GetDefaultCursor() -> *mut SDL_Cursor;

    /// * cursor: 
    pub fn SDL_FreeCursor(
        cursor: *mut SDL_Cursor,
    ) -> c_void;

    /// * toggle: 
    pub fn SDL_ShowCursor(
        toggle: i32,
    ) -> i32;
}
