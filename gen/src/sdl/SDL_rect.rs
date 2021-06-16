// this is generated.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]        
use std::ffi::c_void;
extern crate va_list;
use super::*;

#[repr(C)]
pub struct SDL_Point {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
pub struct SDL_FPoint {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
pub struct SDL_Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[repr(C)]
pub struct SDL_FRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[link(name = "SDL2", kind = "static")]
extern "C" {

    /// * p: 
    /// * r: 
    #[link_name = "SDL_PointInRect"]
    pub fn SDL_PointInRect(
        p: *const SDL_Point,
        r: *const SDL_Rect,
    ) -> SDL_bool;

    /// * r: 
    #[link_name = "SDL_RectEmpty"]
    pub fn SDL_RectEmpty(
        r: *const SDL_Rect,
    ) -> SDL_bool;

    /// * a: 
    /// * b: 
    #[link_name = "SDL_RectEquals"]
    pub fn SDL_RectEquals(
        a: *const SDL_Rect,
        b: *const SDL_Rect,
    ) -> SDL_bool;

    /// * A: 
    /// * B: 
    #[link_name = "SDL_HasIntersection"]
    pub fn SDL_HasIntersection(
        A: *const SDL_Rect,
        B: *const SDL_Rect,
    ) -> SDL_bool;

    /// * A: 
    /// * B: 
    /// * result: 
    #[link_name = "SDL_IntersectRect"]
    pub fn SDL_IntersectRect(
        A: *const SDL_Rect,
        B: *const SDL_Rect,
        result: *mut SDL_Rect,
    ) -> SDL_bool;

    /// * A: 
    /// * B: 
    /// * result: 
    #[link_name = "SDL_UnionRect"]
    pub fn SDL_UnionRect(
        A: *const SDL_Rect,
        B: *const SDL_Rect,
        result: *mut SDL_Rect,
    ) -> c_void;

    /// * points: 
    /// * count: 
    /// * clip: 
    /// * result: 
    #[link_name = "SDL_EnclosePoints"]
    pub fn SDL_EnclosePoints(
        points: *const SDL_Point,
        count: i32,
        clip: *const SDL_Rect,
        result: *mut SDL_Rect,
    ) -> SDL_bool;

    /// * rect: 
    /// * X1: 
    /// * Y1: 
    /// * X2: 
    /// * Y2: 
    #[link_name = "SDL_IntersectRectAndLine"]
    pub fn SDL_IntersectRectAndLine(
        rect: *const SDL_Rect,
        X1: *mut i32,
        Y1: *mut i32,
        X2: *mut i32,
        Y2: *mut i32,
    ) -> SDL_bool;
}
