// this is generated.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]        
use std::ffi::c_void;
extern crate va_list;
use super::*;
pub const SDL_BLENDMODE_NONE: i32 = 0;
pub const SDL_BLENDMODE_BLEND: i32 = 0x1;
pub const SDL_BLENDMODE_ADD: i32 = 0x2;
pub const SDL_BLENDMODE_MOD: i32 = 0x4;
pub const SDL_BLENDMODE_MUL: i32 = 0x8;
pub const SDL_BLENDMODE_INVALID: i32 = 0x7fffffff;
pub const SDL_BLENDOPERATION_ADD: i32 = 0x1;
pub const SDL_BLENDOPERATION_SUBTRACT: i32 = 0x2;
pub const SDL_BLENDOPERATION_REV_SUBTRACT: i32 = 0x3;
pub const SDL_BLENDOPERATION_MINIMUM: i32 = 0x4;
pub const SDL_BLENDOPERATION_MAXIMUM: i32 = 0x5;
pub const SDL_BLENDFACTOR_ZERO: i32 = 0x1;
pub const SDL_BLENDFACTOR_ONE: i32 = 0x2;
pub const SDL_BLENDFACTOR_SRC_COLOR: i32 = 0x3;
pub const SDL_BLENDFACTOR_ONE_MINUS_SRC_COLOR: i32 = 0x4;
pub const SDL_BLENDFACTOR_SRC_ALPHA: i32 = 0x5;
pub const SDL_BLENDFACTOR_ONE_MINUS_SRC_ALPHA: i32 = 0x6;
pub const SDL_BLENDFACTOR_DST_COLOR: i32 = 0x7;
pub const SDL_BLENDFACTOR_ONE_MINUS_DST_COLOR: i32 = 0x8;
pub const SDL_BLENDFACTOR_DST_ALPHA: i32 = 0x9;
pub const SDL_BLENDFACTOR_ONE_MINUS_DST_ALPHA: i32 = 0xa;

#[link(name = "SDL2", kind = "static")]
extern "C" {

    /// * srcColorFactor: 
    /// * dstColorFactor: 
    /// * colorOperation: 
    /// * srcAlphaFactor: 
    /// * dstAlphaFactor: 
    /// * alphaOperation: 
    pub fn SDL_ComposeCustomBlendMode(
        srcColorFactor: i32,
        dstColorFactor: i32,
        colorOperation: i32,
        srcAlphaFactor: i32,
        dstAlphaFactor: i32,
        alphaOperation: i32,
    ) -> i32;
}
