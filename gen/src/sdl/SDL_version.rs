// this is generated.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]        
use std::ffi::c_void;
extern crate va_list;
use super::*;
pub const SDL_MAJOR_VERSION: i32 = 2;
pub const SDL_MINOR_VERSION: i32 = 0;
pub const SDL_PATCHLEVEL: i32 = 15;
/* SDL_VERSION(x)\
{(x)->major=SDL_MAJOR_VERSION;(x)->minor=SDL_MINOR_VERSION;(x)->patch=SDL_PATCHLEVEL;\
} */
// SDL_VERSIONNUM(X,Y,Z)((X)*1000+(Y)*100+(Z))
//SDL_COMPILEDVERSION SDL_VERSIONNUM(SDL_MAJOR_VERSION,SDL_MINOR_VERSION,SDL_PATCHLEVEL)
/* SDL_VERSION_ATLEAST(X,Y,Z)(SDL_COMPILEDVERSION>=SDL_VERSIONNUM(X,Y,Z)) */

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

#[link(name = "SDL2", kind = "static")]
extern "C" {

    /// * ver: 
    pub fn SDL_GetVersion(
        ver: *mut SDL_version,
    ) -> c_void;

    pub fn SDL_GetRevision() -> *mut i8;

    pub fn SDL_GetRevisionNumber() -> i32;
}
