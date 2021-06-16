// this is generated.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]        
use std::ffi::c_void;
extern crate va_list;
use super::*;
//SDL_COPY_MODULATE_COLOR 0x00000001
//SDL_COPY_MODULATE_ALPHA 0x00000002
//SDL_COPY_BLEND 0x00000010
//SDL_COPY_ADD 0x00000020
//SDL_COPY_MOD 0x00000040
//SDL_COPY_MUL 0x00000080
//SDL_COPY_COLORKEY 0x00000100
//SDL_COPY_NEAREST 0x00000200
//SDL_COPY_RLE_DESIRED 0x00001000
//SDL_COPY_RLE_COLORKEY 0x00002000
//SDL_COPY_RLE_ALPHAKEY 0x00004000
//SDL_COPY_RLE_MASK (SDL_COPY_RLE_DESIRED|SDL_COPY_RLE_COLORKEY|SDL_COPY_RLE_ALPHAKEY)
//SDL_CPU_ANY 0x00000000
//SDL_CPU_MMX 0x00000001
//SDL_CPU_3DNOW 0x00000002
//SDL_CPU_SSE 0x00000004
//SDL_CPU_SSE2 0x00000008
//SDL_CPU_ALTIVEC_PREFETCH 0x00000010
//SDL_CPU_ALTIVEC_NOPREFETCH 0x00000020

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_BlitInfo {
    pub src: *mut Uint8,
    pub src_w: i32,
    pub src_h: i32,
    pub src_pitch: i32,
    pub src_skip: i32,
    pub dst: *mut Uint8,
    pub dst_w: i32,
    pub dst_h: i32,
    pub dst_pitch: i32,
    pub dst_skip: i32,
    pub src_fmt: *mut SDL_PixelFormat,
    pub dst_fmt: *mut SDL_PixelFormat,
    pub table: *mut Uint8,
    pub flags: i32,
    pub colorkey: u32,
    pub r: Uint8,
    pub g: Uint8,
    pub b: Uint8,
    pub a: Uint8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_BlitFuncEntry {
    pub src_format: u32,
    pub dst_format: u32,
    pub flags: i32,
    pub cpu: i32,
    pub func: extern fn(*mut SDL_BlitInfo,) -> c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_BlitMap {
    pub dst: *mut SDL_Surface,
    pub identity: i32,
    pub blit: extern fn(*mut SDL_Surface,*mut SDL_Rect,*mut SDL_Surface,*mut SDL_Rect,) -> i32,
    pub data: *mut c_void,
    pub info: SDL_BlitInfo,
    pub dst_palette_version: u32,
    pub src_palette_version: u32,
}

#[link(name = "SDL2", kind = "static")]
extern "C" {

    /// * surface: 
    #[link_name = "?SDL_CalculateBlit@@YAHPEAUSDL_Surface@@@Z"]
    pub fn SDL_CalculateBlit(
        surface: *mut SDL_Surface,
    ) -> i32;

    /// * surface: 
    #[link_name = "?SDL_CalculateBlit0@@YAP6AXPEAUSDL_BlitInfo@@@ZPEAUSDL_Surface@@@Z"]
    pub fn SDL_CalculateBlit0(
        surface: *mut SDL_Surface,
    ) -> extern fn(*mut SDL_BlitInfo,) -> c_void;

    /// * surface: 
    #[link_name = "?SDL_CalculateBlit1@@YAP6AXPEAUSDL_BlitInfo@@@ZPEAUSDL_Surface@@@Z"]
    pub fn SDL_CalculateBlit1(
        surface: *mut SDL_Surface,
    ) -> extern fn(*mut SDL_BlitInfo,) -> c_void;

    /// * surface: 
    #[link_name = "?SDL_CalculateBlitN@@YAP6AXPEAUSDL_BlitInfo@@@ZPEAUSDL_Surface@@@Z"]
    pub fn SDL_CalculateBlitN(
        surface: *mut SDL_Surface,
    ) -> extern fn(*mut SDL_BlitInfo,) -> c_void;

    /// * surface: 
    #[link_name = "?SDL_CalculateBlitA@@YAP6AXPEAUSDL_BlitInfo@@@ZPEAUSDL_Surface@@@Z"]
    pub fn SDL_CalculateBlitA(
        surface: *mut SDL_Surface,
    ) -> extern fn(*mut SDL_BlitInfo,) -> c_void;
}
