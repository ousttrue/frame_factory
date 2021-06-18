// this is generated.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]        
use std::ffi::c_void;
extern crate va_list;
use super::*;

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum SDL_BlendMode {
    SDL_BLENDMODE_NONE = 0,
    SDL_BLENDMODE_BLEND = 0x1,
    SDL_BLENDMODE_ADD = 0x2,
    SDL_BLENDMODE_MOD = 0x4,
    SDL_BLENDMODE_MUL = 0x8,
    SDL_BLENDMODE_INVALID = 0x7fffffff,
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum SDL_BlendOperation {
    SDL_BLENDOPERATION_ADD = 0x1,
    SDL_BLENDOPERATION_SUBTRACT = 0x2,
    SDL_BLENDOPERATION_REV_SUBTRACT = 0x3,
    SDL_BLENDOPERATION_MINIMUM = 0x4,
    SDL_BLENDOPERATION_MAXIMUM = 0x5,
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum SDL_BlendFactor {
    SDL_BLENDFACTOR_ZERO = 0x1,
    SDL_BLENDFACTOR_ONE = 0x2,
    SDL_BLENDFACTOR_SRC_COLOR = 0x3,
    SDL_BLENDFACTOR_ONE_MINUS_SRC_COLOR = 0x4,
    SDL_BLENDFACTOR_SRC_ALPHA = 0x5,
    SDL_BLENDFACTOR_ONE_MINUS_SRC_ALPHA = 0x6,
    SDL_BLENDFACTOR_DST_COLOR = 0x7,
    SDL_BLENDFACTOR_ONE_MINUS_DST_COLOR = 0x8,
    SDL_BLENDFACTOR_DST_ALPHA = 0x9,
    SDL_BLENDFACTOR_ONE_MINUS_DST_ALPHA = 0xa,
}

#[link(name = "SDL2", kind = "static")]
extern "C" {

    /// * srcColorFactor: 
    /// * dstColorFactor: 
    /// * colorOperation: 
    /// * srcAlphaFactor: 
    /// * dstAlphaFactor: 
    /// * alphaOperation: 
    pub fn SDL_ComposeCustomBlendMode(
        srcColorFactor: SDL_BlendFactor,
        dstColorFactor: SDL_BlendFactor,
        colorOperation: SDL_BlendOperation,
        srcAlphaFactor: SDL_BlendFactor,
        dstAlphaFactor: SDL_BlendFactor,
        alphaOperation: SDL_BlendOperation,
    ) -> SDL_BlendMode;
}