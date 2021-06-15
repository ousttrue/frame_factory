// this is generated.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]        
use std::ffi::c_void;
extern crate va_list;

pub const SDL_INIT_TIMER: i32 = 0x00000001u;
pub const SDL_INIT_AUDIO: i32 = 0x00000010u;
pub const SDL_INIT_VIDEO: i32 = 0x00000020u;
pub const SDL_INIT_JOYSTICK: i32 = 0x00000200u;
pub const SDL_INIT_HAPTIC: i32 = 0x00001000u;
pub const SDL_INIT_GAMECONTROLLER: i32 = 0x00002000u;
pub const SDL_INIT_EVENTS: i32 = 0x00004000u;
pub const SDL_INIT_SENSOR: i32 = 0x00008000u;
pub const SDL_INIT_NOPARACHUTE: i32 = 0x00100000u;
pub const SDL_INIT_EVERYTHING: i32 = (SDL_INIT_TIMER|SDL_INIT_AUDIO|SDL_INIT_VIDEO|SDL_INIT_EVENTS|SDL_INIT_JOYSTICK|SDL_INIT_HAPTIC|SDL_INIT_GAMECONTROLLER|SDL_INIT_SENSOR);

#[link(name = "SDL2", kind = "static")]
extern "C" {

    /// * flags: 
    #[link_name = "SDL_Init"]
    pub fn SDL_Init(
        flags: u32,
    ) -> i32;

    /// * flags: 
    #[link_name = "SDL_InitSubSystem"]
    pub fn SDL_InitSubSystem(
        flags: u32,
    ) -> i32;

    /// * flags: 
    #[link_name = "SDL_QuitSubSystem"]
    pub fn SDL_QuitSubSystem(
        flags: u32,
    ) -> c_void;

    /// * flags: 
    #[link_name = "SDL_WasInit"]
    pub fn SDL_WasInit(
        flags: u32,
    ) -> u32;

    #[link_name = "SDL_Quit"]
    pub fn SDL_Quit() -> c_void;
}
