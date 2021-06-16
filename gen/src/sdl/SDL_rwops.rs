// this is generated.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]        
use std::ffi::c_void;
extern crate va_list;
use super::*;
//SDL_RWOPS_UNKNOWN 0U
//SDL_RWOPS_WINFILE 1U
//SDL_RWOPS_STDFILE 2U
//SDL_RWOPS_JNIFILE 3U
//SDL_RWOPS_MEMORY 4U
//SDL_RWOPS_MEMORY_RO 5U
//RW_SEEK_SET 0
//RW_SEEK_CUR 1
//RW_SEEK_END 2

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_RWops {
    pub size: *mut extern fn(*mut SDL_RWops,) -> Sint64,
    pub seek: *mut extern fn(*mut SDL_RWops,Sint64,i32,) -> Sint64,
    pub read: *mut extern fn(*mut SDL_RWops,*mut c_void,usize,usize,) -> usize,
    pub write: *mut extern fn(*mut SDL_RWops,*mut c_void,usize,usize,) -> usize,
    pub close: *mut extern fn(*mut SDL_RWops,) -> i32,
    pub r#type: u32,
    pub hidden: anonymous_0,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union anonymous_0 {
    pub windowsio: anonymous_1,
    pub mem: anonymous_3,
    pub unknown: anonymous_4,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct anonymous_1 {
    pub append: SDL_bool,
    pub h: *mut c_void,
    pub buffer: anonymous_2,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct anonymous_2 {
    pub data: *mut c_void,
    pub size: usize,
    pub left: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct anonymous_3 {
    pub base: *mut Uint8,
    pub here: *mut Uint8,
    pub stop: *mut Uint8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct anonymous_4 {
    pub data1: *mut c_void,
    pub data2: *mut c_void,
}

#[link(name = "SDL2", kind = "static")]
extern "C" {

    /// * file: 
    /// * mode: 
    pub fn SDL_RWFromFile(
        file: *const i8,
        mode: *const i8,
    ) -> *mut SDL_RWops;

    /// * fp: 
    /// * autoclose: 
    pub fn SDL_RWFromFP(
        fp: *mut c_void,
        autoclose: SDL_bool,
    ) -> *mut SDL_RWops;

    /// * mem: 
    /// * size: 
    pub fn SDL_RWFromMem(
        mem: *mut c_void,
        size: i32,
    ) -> *mut SDL_RWops;

    /// * mem: 
    /// * size: 
    pub fn SDL_RWFromConstMem(
        mem: *const c_void,
        size: i32,
    ) -> *mut SDL_RWops;

    pub fn SDL_AllocRW() -> *mut SDL_RWops;

    /// * area: 
    pub fn SDL_FreeRW(
        area: *mut SDL_RWops,
    ) -> c_void;

    /// * context: 
    pub fn SDL_RWsize(
        context: *mut SDL_RWops,
    ) -> Sint64;

    /// * context: 
    /// * offset: 
    /// * whence: 
    pub fn SDL_RWseek(
        context: *mut SDL_RWops,
        offset: Sint64,
        whence: i32,
    ) -> Sint64;

    /// * context: 
    pub fn SDL_RWtell(
        context: *mut SDL_RWops,
    ) -> Sint64;

    /// * context: 
    /// * ptr: 
    /// * size: 
    /// * maxnum: 
    pub fn SDL_RWread(
        context: *mut SDL_RWops,
        ptr: *mut c_void,
        size: usize,
        maxnum: usize,
    ) -> usize;

    /// * context: 
    /// * ptr: 
    /// * size: 
    /// * num: 
    pub fn SDL_RWwrite(
        context: *mut SDL_RWops,
        ptr: *const c_void,
        size: usize,
        num: usize,
    ) -> usize;

    /// * context: 
    pub fn SDL_RWclose(
        context: *mut SDL_RWops,
    ) -> i32;

    /// * src: 
    /// * datasize: 
    /// * freesrc: 
    pub fn SDL_LoadFile_RW(
        src: *mut SDL_RWops,
        datasize: *mut usize,
        freesrc: i32,
    ) -> *mut c_void;

    /// * file: 
    /// * datasize: 
    pub fn SDL_LoadFile(
        file: *const i8,
        datasize: *mut usize,
    ) -> *mut c_void;

    /// * src: 
    pub fn SDL_ReadU8(
        src: *mut SDL_RWops,
    ) -> Uint8;

    /// * src: 
    pub fn SDL_ReadLE16(
        src: *mut SDL_RWops,
    ) -> u16;

    /// * src: 
    pub fn SDL_ReadBE16(
        src: *mut SDL_RWops,
    ) -> u16;

    /// * src: 
    pub fn SDL_ReadLE32(
        src: *mut SDL_RWops,
    ) -> u32;

    /// * src: 
    pub fn SDL_ReadBE32(
        src: *mut SDL_RWops,
    ) -> u32;

    /// * src: 
    pub fn SDL_ReadLE64(
        src: *mut SDL_RWops,
    ) -> Uint64;

    /// * src: 
    pub fn SDL_ReadBE64(
        src: *mut SDL_RWops,
    ) -> Uint64;

    /// * dst: 
    /// * value: 
    pub fn SDL_WriteU8(
        dst: *mut SDL_RWops,
        value: Uint8,
    ) -> usize;

    /// * dst: 
    /// * value: 
    pub fn SDL_WriteLE16(
        dst: *mut SDL_RWops,
        value: u16,
    ) -> usize;

    /// * dst: 
    /// * value: 
    pub fn SDL_WriteBE16(
        dst: *mut SDL_RWops,
        value: u16,
    ) -> usize;

    /// * dst: 
    /// * value: 
    pub fn SDL_WriteLE32(
        dst: *mut SDL_RWops,
        value: u32,
    ) -> usize;

    /// * dst: 
    /// * value: 
    pub fn SDL_WriteBE32(
        dst: *mut SDL_RWops,
        value: u32,
    ) -> usize;

    /// * dst: 
    /// * value: 
    pub fn SDL_WriteLE64(
        dst: *mut SDL_RWops,
        value: Uint64,
    ) -> usize;

    /// * dst: 
    /// * value: 
    pub fn SDL_WriteBE64(
        dst: *mut SDL_RWops,
        value: Uint64,
    ) -> usize;
}
