// this is generated.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]        
use std::ffi::c_void;
extern crate va_list;


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
