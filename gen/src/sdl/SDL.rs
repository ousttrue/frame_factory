// this is generated.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]        
use std::ffi::c_void;
extern crate va_list;
use super::*;
pub const SDL_INIT_TIMER: u32 = 0x00000001;
pub const SDL_INIT_AUDIO: u32 = 0x00000010;
pub const SDL_INIT_VIDEO: u32 = 0x00000020;
pub const SDL_INIT_JOYSTICK: u32 = 0x00000200;
pub const SDL_INIT_HAPTIC: u32 = 0x00001000;
pub const SDL_INIT_GAMECONTROLLER: u32 = 0x00002000;
pub const SDL_INIT_EVENTS: u32 = 0x00004000;
pub const SDL_INIT_SENSOR: u32 = 0x00008000;
pub const SDL_INIT_NOPARACHUTE: u32 = 0x00100000;
//SDL_INIT_EVERYTHING (SDL_INIT_TIMER|SDL_INIT_AUDIO|SDL_INIT_VIDEO|SDL_INIT_EVENTS|SDL_INIT_JOYSTICK|SDL_INIT_HAPTIC|SDL_INIT_GAMECONTROLLER|SDL_INIT_SENSOR)

#[link(name = "SDL2", kind = "static")]
extern "C" {

    /// * flags: 
    pub fn SDL_Init(
        flags: u32,
    ) -> i32;

    /// * flags: 
    pub fn SDL_InitSubSystem(
        flags: u32,
    ) -> i32;

    /// * flags: 
    pub fn SDL_QuitSubSystem(
        flags: u32,
    ) -> c_void;

    /// * flags: 
    pub fn SDL_WasInit(
        flags: u32,
    ) -> u32;

    pub fn SDL_Quit() -> c_void;
}
