// this is generated.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]        
use std::ffi::c_void;
extern crate va_list;
use super::*;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_Keysym {
    pub scancode: i32,
    pub sym: i32,
    pub r#mod: u16,
    pub unused: u32,
}

#[link(name = "SDL2", kind = "static")]
extern "C" {

    pub fn SDL_GetKeyboardFocus() -> *mut SDL_Window;

    /// * numkeys: 
    pub fn SDL_GetKeyboardState(
        numkeys: *mut i32,
    ) -> *mut u8;

    pub fn SDL_GetModState() -> i32;

    /// * modstate: 
    pub fn SDL_SetModState(
        modstate: i32,
    ) -> c_void;

    /// * scancode: 
    pub fn SDL_GetKeyFromScancode(
        scancode: i32,
    ) -> i32;

    /// * key: 
    pub fn SDL_GetScancodeFromKey(
        key: i32,
    ) -> i32;

    /// * scancode: 
    pub fn SDL_GetScancodeName(
        scancode: i32,
    ) -> *mut i8;

    /// * name: 
    pub fn SDL_GetScancodeFromName(
        name: *const i8,
    ) -> i32;

    /// * key: 
    pub fn SDL_GetKeyName(
        key: i32,
    ) -> *mut i8;

    /// * name: 
    pub fn SDL_GetKeyFromName(
        name: *const i8,
    ) -> i32;

    pub fn SDL_StartTextInput() -> c_void;

    pub fn SDL_IsTextInputActive() -> i32;

    pub fn SDL_StopTextInput() -> c_void;

    /// * rect: 
    pub fn SDL_SetTextInputRect(
        rect: *mut SDL_Rect,
    ) -> c_void;

    pub fn SDL_HasScreenKeyboardSupport() -> i32;

    /// * window: 
    pub fn SDL_IsScreenKeyboardShown(
        window: *mut SDL_Window,
    ) -> i32;
}
