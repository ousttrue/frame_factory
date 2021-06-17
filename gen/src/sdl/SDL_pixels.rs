// this is generated.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]        
use std::ffi::c_void;
extern crate va_list;
use super::*;
//SDL_ALPHA_OPAQUE 255
//SDL_ALPHA_TRANSPARENT 0
// SDL_DEFINE_PIXELFOURCC(A,B,C,D)SDL_FOURCC(A,B,C,D)
// SDL_DEFINE_PIXELFORMAT(type,order,layout,bits,bytes)((1<<28)|((type)<<24)|((order)<<20)|((layout)<<16)|((bits)<<8)|((bytes)<<0))
// SDL_PIXELFLAG(X)(((X)>>28)&0x0F)
// SDL_PIXELTYPE(X)(((X)>>24)&0x0F)
// SDL_PIXELORDER(X)(((X)>>20)&0x0F)
// SDL_PIXELLAYOUT(X)(((X)>>16)&0x0F)
// SDL_BITSPERPIXEL(X)(((X)>>8)&0xFF)
// SDL_BYTESPERPIXEL(X)(SDL_ISPIXELFORMAT_FOURCC(X)?((((X)==SDL_PIXELFORMAT_YUY2)||((X)==SDL_PIXELFORMAT_UYVY)||((X)==SDL_PIXELFORMAT_YVYU))?2:1):(((X)>>0)&0xFF))
// SDL_ISPIXELFORMAT_INDEXED(format)(!SDL_ISPIXELFORMAT_FOURCC(format)&&((SDL_PIXELTYPE(format)==SDL_PIXELTYPE_INDEX1)||(SDL_PIXELTYPE(format)==SDL_PIXELTYPE_INDEX4)||(SDL_PIXELTYPE(format)==SDL_PIXELTYPE_INDEX8)))
// SDL_ISPIXELFORMAT_PACKED(format)(!SDL_ISPIXELFORMAT_FOURCC(format)&&((SDL_PIXELTYPE(format)==SDL_PIXELTYPE_PACKED8)||(SDL_PIXELTYPE(format)==SDL_PIXELTYPE_PACKED16)||(SDL_PIXELTYPE(format)==SDL_PIXELTYPE_PACKED32)))
// SDL_ISPIXELFORMAT_ARRAY(format)(!SDL_ISPIXELFORMAT_FOURCC(format)&&((SDL_PIXELTYPE(format)==SDL_PIXELTYPE_ARRAYU8)||(SDL_PIXELTYPE(format)==SDL_PIXELTYPE_ARRAYU16)||(SDL_PIXELTYPE(format)==SDL_PIXELTYPE_ARRAYU32)||(SDL_PIXELTYPE(format)==SDL_PIXELTYPE_ARRAYF16)||(SDL_PIXELTYPE(format)==SDL_PIXELTYPE_ARRAYF32)))
// SDL_ISPIXELFORMAT_ALPHA(format)((SDL_ISPIXELFORMAT_PACKED(format)&&((SDL_PIXELORDER(format)==SDL_PACKEDORDER_ARGB)||(SDL_PIXELORDER(format)==SDL_PACKEDORDER_RGBA)||(SDL_PIXELORDER(format)==SDL_PACKEDORDER_ABGR)||(SDL_PIXELORDER(format)==SDL_PACKEDORDER_BGRA)))||(SDL_ISPIXELFORMAT_ARRAY(format)&&((SDL_PIXELORDER(format)==SDL_ARRAYORDER_ARGB)||(SDL_PIXELORDER(format)==SDL_ARRAYORDER_RGBA)||(SDL_PIXELORDER(format)==SDL_ARRAYORDER_ABGR)||(SDL_PIXELORDER(format)==SDL_ARRAYORDER_BGRA))))
// SDL_ISPIXELFORMAT_FOURCC(format)((format)&&(SDL_PIXELFLAG(format)!=1))
//SDL_Colour SDL_Color

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum SDL_PixelType {
    SDL_PIXELTYPE_UNKNOWN = 0,
    SDL_PIXELTYPE_INDEX1 = 0x1,
    SDL_PIXELTYPE_INDEX4 = 0x2,
    SDL_PIXELTYPE_INDEX8 = 0x3,
    SDL_PIXELTYPE_PACKED8 = 0x4,
    SDL_PIXELTYPE_PACKED16 = 0x5,
    SDL_PIXELTYPE_PACKED32 = 0x6,
    SDL_PIXELTYPE_ARRAYU8 = 0x7,
    SDL_PIXELTYPE_ARRAYU16 = 0x8,
    SDL_PIXELTYPE_ARRAYU32 = 0x9,
    SDL_PIXELTYPE_ARRAYF16 = 0xa,
    SDL_PIXELTYPE_ARRAYF32 = 0xb,
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum SDL_BitmapOrder {
    SDL_BITMAPORDER_NONE = 0,
    SDL_BITMAPORDER_4321 = 0x1,
    SDL_BITMAPORDER_1234 = 0x2,
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum SDL_PackedOrder {
    SDL_PACKEDORDER_NONE = 0,
    SDL_PACKEDORDER_XRGB = 0x1,
    SDL_PACKEDORDER_RGBX = 0x2,
    SDL_PACKEDORDER_ARGB = 0x3,
    SDL_PACKEDORDER_RGBA = 0x4,
    SDL_PACKEDORDER_XBGR = 0x5,
    SDL_PACKEDORDER_BGRX = 0x6,
    SDL_PACKEDORDER_ABGR = 0x7,
    SDL_PACKEDORDER_BGRA = 0x8,
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum SDL_ArrayOrder {
    SDL_ARRAYORDER_NONE = 0,
    SDL_ARRAYORDER_RGB = 0x1,
    SDL_ARRAYORDER_RGBA = 0x2,
    SDL_ARRAYORDER_ARGB = 0x3,
    SDL_ARRAYORDER_BGR = 0x4,
    SDL_ARRAYORDER_BGRA = 0x5,
    SDL_ARRAYORDER_ABGR = 0x6,
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum SDL_PackedLayout {
    SDL_PACKEDLAYOUT_NONE = 0,
    SDL_PACKEDLAYOUT_332 = 0x1,
    SDL_PACKEDLAYOUT_4444 = 0x2,
    SDL_PACKEDLAYOUT_1555 = 0x3,
    SDL_PACKEDLAYOUT_5551 = 0x4,
    SDL_PACKEDLAYOUT_565 = 0x5,
    SDL_PACKEDLAYOUT_8888 = 0x6,
    SDL_PACKEDLAYOUT_2101010 = 0x7,
    SDL_PACKEDLAYOUT_1010102 = 0x8,
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum SDL_PixelFormatEnum {
    SDL_PIXELFORMAT_UNKNOWN = 0,
    SDL_PIXELFORMAT_INDEX1LSB = 0x11100100,
    SDL_PIXELFORMAT_INDEX1MSB = 0x11200100,
    SDL_PIXELFORMAT_INDEX4LSB = 0x12100400,
    SDL_PIXELFORMAT_INDEX4MSB = 0x12200400,
    SDL_PIXELFORMAT_INDEX8 = 0x13000801,
    SDL_PIXELFORMAT_RGB332 = 0x14110801,
    SDL_PIXELFORMAT_XRGB4444 = 0x15120c02,
    // SDL_PIXELFORMAT_RGB444 = 0x15120c02,
    SDL_PIXELFORMAT_XBGR4444 = 0x15520c02,
    // SDL_PIXELFORMAT_BGR444 = 0x15520c02,
    SDL_PIXELFORMAT_XRGB1555 = 0x15130f02,
    // SDL_PIXELFORMAT_RGB555 = 0x15130f02,
    SDL_PIXELFORMAT_XBGR1555 = 0x15530f02,
    // SDL_PIXELFORMAT_BGR555 = 0x15530f02,
    SDL_PIXELFORMAT_ARGB4444 = 0x15321002,
    SDL_PIXELFORMAT_RGBA4444 = 0x15421002,
    SDL_PIXELFORMAT_ABGR4444 = 0x15721002,
    SDL_PIXELFORMAT_BGRA4444 = 0x15821002,
    SDL_PIXELFORMAT_ARGB1555 = 0x15331002,
    SDL_PIXELFORMAT_RGBA5551 = 0x15441002,
    SDL_PIXELFORMAT_ABGR1555 = 0x15731002,
    SDL_PIXELFORMAT_BGRA5551 = 0x15841002,
    SDL_PIXELFORMAT_RGB565 = 0x15151002,
    SDL_PIXELFORMAT_BGR565 = 0x15551002,
    SDL_PIXELFORMAT_RGB24 = 0x17101803,
    SDL_PIXELFORMAT_BGR24 = 0x17401803,
    SDL_PIXELFORMAT_XRGB8888 = 0x16161804,
    // SDL_PIXELFORMAT_RGB888 = 0x16161804,
    SDL_PIXELFORMAT_RGBX8888 = 0x16261804,
    SDL_PIXELFORMAT_XBGR8888 = 0x16561804,
    // SDL_PIXELFORMAT_BGR888 = 0x16561804,
    SDL_PIXELFORMAT_BGRX8888 = 0x16661804,
    SDL_PIXELFORMAT_ARGB8888 = 0x16362004,
    SDL_PIXELFORMAT_RGBA8888 = 0x16462004,
    SDL_PIXELFORMAT_ABGR8888 = 0x16762004,
    SDL_PIXELFORMAT_BGRA8888 = 0x16862004,
    SDL_PIXELFORMAT_ARGB2101010 = 0x16372004,
    // SDL_PIXELFORMAT_RGBA32 = 0x16762004,
    // SDL_PIXELFORMAT_ARGB32 = 0x16862004,
    // SDL_PIXELFORMAT_BGRA32 = 0x16362004,
    // SDL_PIXELFORMAT_ABGR32 = 0x16462004,
    SDL_PIXELFORMAT_YV12 = 0x32315659,
    SDL_PIXELFORMAT_IYUV = 0x56555949,
    SDL_PIXELFORMAT_YUY2 = 0x32595559,
    SDL_PIXELFORMAT_UYVY = 0x59565955,
    SDL_PIXELFORMAT_YVYU = 0x55595659,
    SDL_PIXELFORMAT_NV12 = 0x3231564e,
    SDL_PIXELFORMAT_NV21 = 0x3132564e,
    SDL_PIXELFORMAT_EXTERNAL_OES = 0x2053454f,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_Palette {
    pub ncolors: i32,
    pub colors: *mut SDL_Color,
    pub version: u32,
    pub refcount: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_PixelFormat {
    pub format: u32,
    pub palette: *mut SDL_Palette,
    pub BitsPerPixel: u8,
    pub BytesPerPixel: u8,
    pub padding: [u8; 2],
    pub Rmask: u32,
    pub Gmask: u32,
    pub Bmask: u32,
    pub Amask: u32,
    pub Rloss: u8,
    pub Gloss: u8,
    pub Bloss: u8,
    pub Aloss: u8,
    pub Rshift: u8,
    pub Gshift: u8,
    pub Bshift: u8,
    pub Ashift: u8,
    pub refcount: i32,
    pub next: *mut SDL_PixelFormat,
}

#[link(name = "SDL2", kind = "static")]
extern "C" {

    /// * format: 
    pub fn SDL_GetPixelFormatName(
        format: u32,
    ) -> *mut i8;

    /// * format: 
    /// * bpp: 
    /// * Rmask: 
    /// * Gmask: 
    /// * Bmask: 
    /// * Amask: 
    pub fn SDL_PixelFormatEnumToMasks(
        format: u32,
        bpp: *mut i32,
        Rmask: *mut u32,
        Gmask: *mut u32,
        Bmask: *mut u32,
        Amask: *mut u32,
    ) -> SDL_bool;

    /// * bpp: 
    /// * Rmask: 
    /// * Gmask: 
    /// * Bmask: 
    /// * Amask: 
    pub fn SDL_MasksToPixelFormatEnum(
        bpp: i32,
        Rmask: u32,
        Gmask: u32,
        Bmask: u32,
        Amask: u32,
    ) -> u32;

    /// * pixel_format: 
    pub fn SDL_AllocFormat(
        pixel_format: u32,
    ) -> *mut SDL_PixelFormat;

    /// * format: 
    pub fn SDL_FreeFormat(
        format: *mut SDL_PixelFormat,
    ) -> c_void;

    /// * ncolors: 
    pub fn SDL_AllocPalette(
        ncolors: i32,
    ) -> *mut SDL_Palette;

    /// * format: 
    /// * palette: 
    pub fn SDL_SetPixelFormatPalette(
        format: *mut SDL_PixelFormat,
        palette: *mut SDL_Palette,
    ) -> i32;

    /// * palette: 
    /// * colors: 
    /// * firstcolor: 
    /// * ncolors: 
    pub fn SDL_SetPaletteColors(
        palette: *mut SDL_Palette,
        colors: *const SDL_Color,
        firstcolor: i32,
        ncolors: i32,
    ) -> i32;

    /// * palette: 
    pub fn SDL_FreePalette(
        palette: *mut SDL_Palette,
    ) -> c_void;

    /// * format: 
    /// * r: 
    /// * g: 
    /// * b: 
    pub fn SDL_MapRGB(
        format: *const SDL_PixelFormat,
        r: u8,
        g: u8,
        b: u8,
    ) -> u32;

    /// * format: 
    /// * r: 
    /// * g: 
    /// * b: 
    /// * a: 
    pub fn SDL_MapRGBA(
        format: *const SDL_PixelFormat,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    ) -> u32;

    /// * pixel: 
    /// * format: 
    /// * r: 
    /// * g: 
    /// * b: 
    pub fn SDL_GetRGB(
        pixel: u32,
        format: *const SDL_PixelFormat,
        r: *mut u8,
        g: *mut u8,
        b: *mut u8,
    ) -> c_void;

    /// * pixel: 
    /// * format: 
    /// * r: 
    /// * g: 
    /// * b: 
    /// * a: 
    pub fn SDL_GetRGBA(
        pixel: u32,
        format: *const SDL_PixelFormat,
        r: *mut u8,
        g: *mut u8,
        b: *mut u8,
        a: *mut u8,
    ) -> c_void;

    /// * gamma: 
    /// * ramp: 
    pub fn SDL_CalculateGammaRamp(
        gamma: f32,
        ramp: *mut u16,
    ) -> c_void;
}
