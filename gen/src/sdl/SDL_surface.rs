// this is generated.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]        
use std::ffi::c_void;
extern crate va_list;
use super::*;
//SDL_SWSURFACE 0
//SDL_PREALLOC 0x00000001
//SDL_RLEACCEL 0x00000002
//SDL_DONTFREE 0x00000004
//SDL_SIMD_ALIGNED 0x00000008
//SDL_BlitSurface SDL_UpperBlit
//SDL_BlitScaled SDL_UpperBlitScaled

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_Surface {
    pub flags: u32,
    pub format: *mut SDL_PixelFormat,
    pub w: i32,
    pub h: i32,
    pub pitch: i32,
    pub pixels: *mut c_void,
    pub userdata: *mut c_void,
    pub locked: i32,
    pub list_blitmap: *mut c_void,
    pub clip_rect: SDL_Rect,
    pub map: *mut SDL_BlitMap,
    pub refcount: i32,
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum SDL_YUV_CONVERSION_MODE {
    SDL_YUV_CONVERSION_JPEG = 0,
    SDL_YUV_CONVERSION_BT601 = 0x1,
    SDL_YUV_CONVERSION_BT709 = 0x2,
    SDL_YUV_CONVERSION_AUTOMATIC = 0x3,
}

#[link(name = "SDL2", kind = "static")]
extern "C" {

    /// * flags: 
    /// * width: 
    /// * height: 
    /// * depth: 
    /// * Rmask: 
    /// * Gmask: 
    /// * Bmask: 
    /// * Amask: 
    pub fn SDL_CreateRGBSurface(
        flags: u32,
        width: i32,
        height: i32,
        depth: i32,
        Rmask: u32,
        Gmask: u32,
        Bmask: u32,
        Amask: u32,
    ) -> *mut SDL_Surface;

    /// * flags: 
    /// * width: 
    /// * height: 
    /// * depth: 
    /// * format: 
    pub fn SDL_CreateRGBSurfaceWithFormat(
        flags: u32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
    ) -> *mut SDL_Surface;

    /// * pixels: 
    /// * width: 
    /// * height: 
    /// * depth: 
    /// * pitch: 
    /// * Rmask: 
    /// * Gmask: 
    /// * Bmask: 
    /// * Amask: 
    pub fn SDL_CreateRGBSurfaceFrom(
        pixels: *mut c_void,
        width: i32,
        height: i32,
        depth: i32,
        pitch: i32,
        Rmask: u32,
        Gmask: u32,
        Bmask: u32,
        Amask: u32,
    ) -> *mut SDL_Surface;

    /// * pixels: 
    /// * width: 
    /// * height: 
    /// * depth: 
    /// * pitch: 
    /// * format: 
    pub fn SDL_CreateRGBSurfaceWithFormatFrom(
        pixels: *mut c_void,
        width: i32,
        height: i32,
        depth: i32,
        pitch: i32,
        format: u32,
    ) -> *mut SDL_Surface;

    /// * surface: 
    pub fn SDL_FreeSurface(
        surface: *mut SDL_Surface,
    ) -> c_void;

    /// * surface: 
    /// * palette: 
    pub fn SDL_SetSurfacePalette(
        surface: *mut SDL_Surface,
        palette: *mut SDL_Palette,
    ) -> i32;

    /// * surface: 
    pub fn SDL_LockSurface(
        surface: *mut SDL_Surface,
    ) -> i32;

    /// * surface: 
    pub fn SDL_UnlockSurface(
        surface: *mut SDL_Surface,
    ) -> c_void;

    /// * src: 
    /// * freesrc: 
    pub fn SDL_LoadBMP_RW(
        src: *mut SDL_RWops,
        freesrc: i32,
    ) -> *mut SDL_Surface;

    /// * surface: 
    /// * dst: 
    /// * freedst: 
    pub fn SDL_SaveBMP_RW(
        surface: *mut SDL_Surface,
        dst: *mut SDL_RWops,
        freedst: i32,
    ) -> i32;

    /// * surface: 
    /// * flag: 
    pub fn SDL_SetSurfaceRLE(
        surface: *mut SDL_Surface,
        flag: i32,
    ) -> i32;

    /// * surface: 
    pub fn SDL_HasSurfaceRLE(
        surface: *mut SDL_Surface,
    ) -> SDL_bool;

    /// * surface: 
    /// * flag: 
    /// * key: 
    pub fn SDL_SetColorKey(
        surface: *mut SDL_Surface,
        flag: i32,
        key: u32,
    ) -> i32;

    /// * surface: 
    pub fn SDL_HasColorKey(
        surface: *mut SDL_Surface,
    ) -> SDL_bool;

    /// * surface: 
    /// * key: 
    pub fn SDL_GetColorKey(
        surface: *mut SDL_Surface,
        key: *mut u32,
    ) -> i32;

    /// * surface: 
    /// * r: 
    /// * g: 
    /// * b: 
    pub fn SDL_SetSurfaceColorMod(
        surface: *mut SDL_Surface,
        r: Uint8,
        g: Uint8,
        b: Uint8,
    ) -> i32;

    /// * surface: 
    /// * r: 
    /// * g: 
    /// * b: 
    pub fn SDL_GetSurfaceColorMod(
        surface: *mut SDL_Surface,
        r: *mut Uint8,
        g: *mut Uint8,
        b: *mut Uint8,
    ) -> i32;

    /// * surface: 
    /// * alpha: 
    pub fn SDL_SetSurfaceAlphaMod(
        surface: *mut SDL_Surface,
        alpha: Uint8,
    ) -> i32;

    /// * surface: 
    /// * alpha: 
    pub fn SDL_GetSurfaceAlphaMod(
        surface: *mut SDL_Surface,
        alpha: *mut Uint8,
    ) -> i32;

    /// * surface: 
    /// * blendMode: 
    pub fn SDL_SetSurfaceBlendMode(
        surface: *mut SDL_Surface,
        blendMode: SDL_BlendMode,
    ) -> i32;

    /// * surface: 
    /// * blendMode: 
    pub fn SDL_GetSurfaceBlendMode(
        surface: *mut SDL_Surface,
        blendMode: *mut SDL_BlendMode,
    ) -> i32;

    /// * surface: 
    /// * rect: 
    pub fn SDL_SetClipRect(
        surface: *mut SDL_Surface,
        rect: *const SDL_Rect,
    ) -> SDL_bool;

    /// * surface: 
    /// * rect: 
    pub fn SDL_GetClipRect(
        surface: *mut SDL_Surface,
        rect: *mut SDL_Rect,
    ) -> c_void;

    /// * surface: 
    pub fn SDL_DuplicateSurface(
        surface: *mut SDL_Surface,
    ) -> *mut SDL_Surface;

    /// * src: 
    /// * fmt: 
    /// * flags: 
    pub fn SDL_ConvertSurface(
        src: *mut SDL_Surface,
        fmt: *const SDL_PixelFormat,
        flags: u32,
    ) -> *mut SDL_Surface;

    /// * src: 
    /// * pixel_format: 
    /// * flags: 
    pub fn SDL_ConvertSurfaceFormat(
        src: *mut SDL_Surface,
        pixel_format: u32,
        flags: u32,
    ) -> *mut SDL_Surface;

    /// * width: 
    /// * height: 
    /// * src_format: 
    /// * src: 
    /// * src_pitch: 
    /// * dst_format: 
    /// * dst: 
    /// * dst_pitch: 
    pub fn SDL_ConvertPixels(
        width: i32,
        height: i32,
        src_format: u32,
        src: *const c_void,
        src_pitch: i32,
        dst_format: u32,
        dst: *mut c_void,
        dst_pitch: i32,
    ) -> i32;

    /// * dst: 
    /// * rect: 
    /// * color: 
    pub fn SDL_FillRect(
        dst: *mut SDL_Surface,
        rect: *const SDL_Rect,
        color: u32,
    ) -> i32;

    /// * dst: 
    /// * rects: 
    /// * count: 
    /// * color: 
    pub fn SDL_FillRects(
        dst: *mut SDL_Surface,
        rects: *const SDL_Rect,
        count: i32,
        color: u32,
    ) -> i32;

    /// * src: 
    /// * srcrect: 
    /// * dst: 
    /// * dstrect: 
    pub fn SDL_UpperBlit(
        src: *mut SDL_Surface,
        srcrect: *const SDL_Rect,
        dst: *mut SDL_Surface,
        dstrect: *mut SDL_Rect,
    ) -> i32;

    /// * src: 
    /// * srcrect: 
    /// * dst: 
    /// * dstrect: 
    pub fn SDL_LowerBlit(
        src: *mut SDL_Surface,
        srcrect: *mut SDL_Rect,
        dst: *mut SDL_Surface,
        dstrect: *mut SDL_Rect,
    ) -> i32;

    /// * src: 
    /// * srcrect: 
    /// * dst: 
    /// * dstrect: 
    pub fn SDL_SoftStretch(
        src: *mut SDL_Surface,
        srcrect: *const SDL_Rect,
        dst: *mut SDL_Surface,
        dstrect: *const SDL_Rect,
    ) -> i32;

    /// * src: 
    /// * srcrect: 
    /// * dst: 
    /// * dstrect: 
    pub fn SDL_SoftStretchLinear(
        src: *mut SDL_Surface,
        srcrect: *const SDL_Rect,
        dst: *mut SDL_Surface,
        dstrect: *const SDL_Rect,
    ) -> i32;

    /// * src: 
    /// * srcrect: 
    /// * dst: 
    /// * dstrect: 
    pub fn SDL_UpperBlitScaled(
        src: *mut SDL_Surface,
        srcrect: *const SDL_Rect,
        dst: *mut SDL_Surface,
        dstrect: *mut SDL_Rect,
    ) -> i32;

    /// * src: 
    /// * srcrect: 
    /// * dst: 
    /// * dstrect: 
    pub fn SDL_LowerBlitScaled(
        src: *mut SDL_Surface,
        srcrect: *mut SDL_Rect,
        dst: *mut SDL_Surface,
        dstrect: *mut SDL_Rect,
    ) -> i32;

    /// * mode: 
    pub fn SDL_SetYUVConversionMode(
        mode: SDL_YUV_CONVERSION_MODE,
    ) -> c_void;

    pub fn SDL_GetYUVConversionMode() -> SDL_YUV_CONVERSION_MODE;

    /// * width: 
    /// * height: 
    pub fn SDL_GetYUVConversionModeForResolution(
        width: i32,
        height: i32,
    ) -> SDL_YUV_CONVERSION_MODE;
}
