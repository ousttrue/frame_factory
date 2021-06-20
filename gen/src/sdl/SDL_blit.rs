// this is generated.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_parens)]
use std::ffi::c_void;
extern crate va_list;
use super::*;

pub const SDL_COPY_MODULATE_COLOR: i32 = 0x00000001;
pub const SDL_COPY_MODULATE_ALPHA: i32 = 0x00000002;
pub const SDL_COPY_BLEND: i32 = 0x00000010;
pub const SDL_COPY_ADD: i32 = 0x00000020;
pub const SDL_COPY_MOD: i32 = 0x00000040;
pub const SDL_COPY_MUL: i32 = 0x00000080;
pub const SDL_COPY_COLORKEY: i32 = 0x00000100;
pub const SDL_COPY_NEAREST: i32 = 0x00000200;
pub const SDL_COPY_RLE_DESIRED: i32 = 0x00001000;
pub const SDL_COPY_RLE_COLORKEY: i32 = 0x00002000;
pub const SDL_COPY_RLE_ALPHAKEY: i32 = 0x00004000;
//SDL_COPY_RLE_MASK (SDL_COPY_RLE_DESIRED|SDL_COPY_RLE_COLORKEY|SDL_COPY_RLE_ALPHAKEY)
pub const SDL_CPU_ANY: i32 = 0x00000000;
pub const SDL_CPU_MMX: i32 = 0x00000001;
pub const SDL_CPU_3DNOW: i32 = 0x00000002;
pub const SDL_CPU_SSE: i32 = 0x00000004;
pub const SDL_CPU_SSE2: i32 = 0x00000008;
pub const SDL_CPU_ALTIVEC_PREFETCH: i32 = 0x00000010;
pub const SDL_CPU_ALTIVEC_NOPREFETCH: i32 = 0x00000020;
/* DECLARE_ALIGNED(t,v,a)__declspec(align(a))tv */
/* RGB_FROM_PIXEL(Pixel,fmt,r,g,b)\
{r=SDL_expand_byte[fmt->Rloss][((Pixel&fmt->Rmask)>>fmt->Rshift)];g=SDL_expand_byte[fmt->Gloss][((Pixel&fmt->Gmask)>>fmt->Gshift)];b=SDL_expand_byte[fmt->Bloss][((Pixel&fmt->Bmask)>>fmt->Bshift)];\
} */
/* RGB_FROM_RGB565(Pixel,r,g,b)\
{r=SDL_expand_byte[3][((Pixel&0xF800)>>11)];g=SDL_expand_byte[2][((Pixel&0x07E0)>>5)];b=SDL_expand_byte[3][(Pixel&0x001F)];\
} */
/* RGB_FROM_RGB555(Pixel,r,g,b)\
{r=SDL_expand_byte[3][((Pixel&0x7C00)>>10)];g=SDL_expand_byte[3][((Pixel&0x03E0)>>5)];b=SDL_expand_byte[3][(Pixel&0x001F)];\
} */
/* RGB_FROM_RGB888(Pixel,r,g,b)\
{r=((Pixel&0xFF0000)>>16);g=((Pixel&0xFF00)>>8);b=(Pixel&0xFF);\
} */
/* RETRIEVE_RGB_PIXEL(buf,bpp,Pixel)do{switch(bpp){case1:Pixel=*((Uint8*)(buf));break;case2:Pixel=*((Uint16*)(buf));break;case3:{Uint8*B=(Uint8*)(buf);if(SDL_BYTEORDER==SDL_LIL_ENDIAN){Pixel=B[0]+(B[1]<<8)+(B[2]<<16);}else{Pixel=(B[0]<<16)+(B[1]<<8)+B[2];}}break;case4:Pixel=*((Uint32*)(buf));break;default:Pixel=0;/* stop gcc complaints */break;}\
}while(0) */
/* DISEMBLE_RGB(buf,bpp,fmt,Pixel,r,g,b)do{switch(bpp){case1:Pixel=*((Uint8*)(buf));RGB_FROM_PIXEL(Pixel,fmt,r,g,b);break;case2:Pixel=*((Uint16*)(buf));RGB_FROM_PIXEL(Pixel,fmt,r,g,b);break;case3:{Pixel=0;if(SDL_BYTEORDER==SDL_LIL_ENDIAN){r=*((buf)+fmt->Rshift/8);g=*((buf)+fmt->Gshift/8);b=*((buf)+fmt->Bshift/8);}else{r=*((buf)+2-fmt->Rshift/8);g=*((buf)+2-fmt->Gshift/8);b=*((buf)+2-fmt->Bshift/8);}}break;case4:Pixel=*((Uint32*)(buf));RGB_FROM_PIXEL(Pixel,fmt,r,g,b);break;default:/* stop gcc complaints */Pixel=0;r=g=b=0;break;}\
}while(0) */
/* PIXEL_FROM_RGB(Pixel,fmt,r,g,b)\
{Pixel=((r>>fmt->Rloss)<<fmt->Rshift)|((g>>fmt->Gloss)<<fmt->Gshift)|((b>>fmt->Bloss)<<fmt->Bshift)|fmt->Amask;\
} */
/* RGB565_FROM_RGB(Pixel,r,g,b)\
{Pixel=((r>>3)<<11)|((g>>2)<<5)|(b>>3);\
} */
/* RGB555_FROM_RGB(Pixel,r,g,b)\
{Pixel=((r>>3)<<10)|((g>>3)<<5)|(b>>3);\
} */
/* RGB888_FROM_RGB(Pixel,r,g,b)\
{Pixel=(r<<16)|(g<<8)|b;\
} */
/* ARGB8888_FROM_RGBA(Pixel,r,g,b,a)\
{Pixel=(a<<24)|(r<<16)|(g<<8)|b;\
} */
/* RGBA8888_FROM_RGBA(Pixel,r,g,b,a)\
{Pixel=(r<<24)|(g<<16)|(b<<8)|a;\
} */
/* ABGR8888_FROM_RGBA(Pixel,r,g,b,a)\
{Pixel=(a<<24)|(b<<16)|(g<<8)|r;\
} */
/* BGRA8888_FROM_RGBA(Pixel,r,g,b,a)\
{Pixel=(b<<24)|(g<<16)|(r<<8)|a;\
} */
/* ARGB2101010_FROM_RGBA(Pixel,r,g,b,a)\
{r=r?((r<<2)|0x3):0;g=g?((g<<2)|0x3):0;b=b?((b<<2)|0x3):0;a=(a*3)/255;Pixel=(a<<30)|(r<<20)|(g<<10)|b;\
} */
/* ASSEMBLE_RGB(buf,bpp,fmt,r,g,b)\
{switch(bpp){case1:{Uint8_Pixel;PIXEL_FROM_RGB(_Pixel,fmt,r,g,b);*((Uint8*)(buf))=_Pixel;}break;case2:{Uint16_Pixel;PIXEL_FROM_RGB(_Pixel,fmt,r,g,b);*((Uint16*)(buf))=_Pixel;}break;case3:{if(SDL_BYTEORDER==SDL_LIL_ENDIAN){*((buf)+fmt->Rshift/8)=r;*((buf)+fmt->Gshift/8)=g;*((buf)+fmt->Bshift/8)=b;}else{*((buf)+2-fmt->Rshift/8)=r;*((buf)+2-fmt->Gshift/8)=g;*((buf)+2-fmt->Bshift/8)=b;}}break;case4:{Uint32_Pixel;PIXEL_FROM_RGB(_Pixel,fmt,r,g,b);*((Uint32*)(buf))=_Pixel;}break;}\
} */
/* RGBA_FROM_PIXEL(Pixel,fmt,r,g,b,a)\
{r=SDL_expand_byte[fmt->Rloss][((Pixel&fmt->Rmask)>>fmt->Rshift)];g=SDL_expand_byte[fmt->Gloss][((Pixel&fmt->Gmask)>>fmt->Gshift)];b=SDL_expand_byte[fmt->Bloss][((Pixel&fmt->Bmask)>>fmt->Bshift)];a=SDL_expand_byte[fmt->Aloss][((Pixel&fmt->Amask)>>fmt->Ashift)];\
} */
/* RGBA_FROM_8888(Pixel,fmt,r,g,b,a)\
{r=(Pixel&fmt->Rmask)>>fmt->Rshift;g=(Pixel&fmt->Gmask)>>fmt->Gshift;b=(Pixel&fmt->Bmask)>>fmt->Bshift;a=(Pixel&fmt->Amask)>>fmt->Ashift;\
} */
/* RGBA_FROM_RGBA8888(Pixel,r,g,b,a)\
{r=(Pixel>>24);g=((Pixel>>16)&0xFF);b=((Pixel>>8)&0xFF);a=(Pixel&0xFF);\
} */
/* RGBA_FROM_ARGB8888(Pixel,r,g,b,a)\
{r=((Pixel>>16)&0xFF);g=((Pixel>>8)&0xFF);b=(Pixel&0xFF);a=(Pixel>>24);\
} */
/* RGBA_FROM_ABGR8888(Pixel,r,g,b,a)\
{r=(Pixel&0xFF);g=((Pixel>>8)&0xFF);b=((Pixel>>16)&0xFF);a=(Pixel>>24);\
} */
/* RGBA_FROM_BGRA8888(Pixel,r,g,b,a)\
{r=((Pixel>>8)&0xFF);g=((Pixel>>16)&0xFF);b=(Pixel>>24);a=(Pixel&0xFF);\
} */
/* RGBA_FROM_ARGB2101010(Pixel,r,g,b,a)\
{r=((Pixel>>22)&0xFF);g=((Pixel>>12)&0xFF);b=((Pixel>>2)&0xFF);a=SDL_expand_byte[6][(Pixel>>30)];\
} */
/* DISEMBLE_RGBA(buf,bpp,fmt,Pixel,r,g,b,a)do{switch(bpp){case1:Pixel=*((Uint8*)(buf));RGBA_FROM_PIXEL(Pixel,fmt,r,g,b,a);break;case2:Pixel=*((Uint16*)(buf));RGBA_FROM_PIXEL(Pixel,fmt,r,g,b,a);break;case3:{Pixel=0;if(SDL_BYTEORDER==SDL_LIL_ENDIAN){r=*((buf)+fmt->Rshift/8);g=*((buf)+fmt->Gshift/8);b=*((buf)+fmt->Bshift/8);}else{r=*((buf)+2-fmt->Rshift/8);g=*((buf)+2-fmt->Gshift/8);b=*((buf)+2-fmt->Bshift/8);}a=0xFF;}break;case4:Pixel=*((Uint32*)(buf));RGBA_FROM_PIXEL(Pixel,fmt,r,g,b,a);break;default:/* stop gcc complaints */Pixel=0;r=g=b=a=0;break;}\
}while(0) */
/* PIXEL_FROM_RGBA(Pixel,fmt,r,g,b,a)\
{Pixel=((r>>fmt->Rloss)<<fmt->Rshift)|((g>>fmt->Gloss)<<fmt->Gshift)|((b>>fmt->Bloss)<<fmt->Bshift)|((a>>fmt->Aloss)<<fmt->Ashift);\
} */
/* ASSEMBLE_RGBA(buf,bpp,fmt,r,g,b,a)\
{switch(bpp){case1:{Uint8_pixel;PIXEL_FROM_RGBA(_pixel,fmt,r,g,b,a);*((Uint8*)(buf))=_pixel;}break;case2:{Uint16_pixel;PIXEL_FROM_RGBA(_pixel,fmt,r,g,b,a);*((Uint16*)(buf))=_pixel;}break;case3:{if(SDL_BYTEORDER==SDL_LIL_ENDIAN){*((buf)+fmt->Rshift/8)=r;*((buf)+fmt->Gshift/8)=g;*((buf)+fmt->Bshift/8)=b;}else{*((buf)+2-fmt->Rshift/8)=r;*((buf)+2-fmt->Gshift/8)=g;*((buf)+2-fmt->Bshift/8)=b;}}break;case4:{Uint32_pixel;PIXEL_FROM_RGBA(_pixel,fmt,r,g,b,a);*((Uint32*)(buf))=_pixel;}break;}\
} */
/* ALPHA_BLEND_RGB(sR,sG,sB,A,dR,dG,dB)do{dR=(Uint8)((((int)(sR-dR)*(int)A)/255)+dR);dG=(Uint8)((((int)(sG-dG)*(int)A)/255)+dG);dB=(Uint8)((((int)(sB-dB)*(int)A)/255)+dB);\
}while(0) */
/* ALPHA_BLEND_RGBA(sR,sG,sB,sA,dR,dG,dB,dA)do{dR=(Uint8)((((int)(sR-dR)*(int)sA)/255)+dR);dG=(Uint8)((((int)(sG-dG)*(int)sA)/255)+dG);dB=(Uint8)((((int)(sB-dB)*(int)sA)/255)+dB);dA=(Uint8)((int)sA+dA-((int)sA*dA)/255);\
}while(0) */
/* DUFFS_LOOP8(pixel_copy_increment,width)\
{intn=(width+7)/8;switch(width&7){case0:do{pixel_copy_increment;/* fallthrough */case7:pixel_copy_increment;/* fallthrough */case6:pixel_copy_increment;/* fallthrough */case5:pixel_copy_increment;/* fallthrough */case4:pixel_copy_increment;/* fallthrough */case3:pixel_copy_increment;/* fallthrough */case2:pixel_copy_increment;/* fallthrough */case1:pixel_copy_increment;/* fallthrough */}while(--n>0);}\
} */
/* DUFFS_LOOP4(pixel_copy_increment,width)\
{intn=(width+3)/4;switch(width&3){case0:do{pixel_copy_increment;/* fallthrough */case3:pixel_copy_increment;/* fallthrough */case2:pixel_copy_increment;/* fallthrough */case1:pixel_copy_increment;/* fallthrough */}while(--n>0);}\
} */
/* DUFFS_LOOP(pixel_copy_increment,width)DUFFS_LOOP8(pixel_copy_increment,width) */
/* DUFFS_LOOP_124(pixel_copy_increment1,pixel_copy_increment2,pixel_copy_increment4,width)\
{intn=width;if(n&1){pixel_copy_increment1;n-=1;}if(n&2){pixel_copy_increment2;n-=2;}if(n&4){pixel_copy_increment4;n-=4;}if(n){n/=8;do{pixel_copy_increment4;pixel_copy_increment4;}while(--n>0);}\
} */

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_BlitInfo {
    pub src: *mut u8,
    pub src_w: i32,
    pub src_h: i32,
    pub src_pitch: i32,
    pub src_skip: i32,
    pub dst: *mut u8,
    pub dst_w: i32,
    pub dst_h: i32,
    pub dst_pitch: i32,
    pub dst_skip: i32,
    pub src_fmt: *mut SDL_PixelFormat,
    pub dst_fmt: *mut SDL_PixelFormat,
    pub table: *mut u8,
    pub flags: i32,
    pub colorkey: u32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
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
