// this is generated.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]        
use std::ffi::c_void;
extern crate va_list;
use super::*;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_Point {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_FPoint {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
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
    pub fn SDL_PointInRect(
        p: *const SDL_Point,
        r: *const SDL_Rect,
    ) -> i32;

    /// * r: 
    pub fn SDL_RectEmpty(
        r: *const SDL_Rect,
    ) -> i32;

    /// * a: 
    /// * b: 
    pub fn SDL_RectEquals(
        a: *const SDL_Rect,
        b: *const SDL_Rect,
    ) -> i32;

    /// * A: 
    /// * B: 
    pub fn SDL_HasIntersection(
        A: *const SDL_Rect,
        B: *const SDL_Rect,
    ) -> i32;

    /// * A: 
    /// * B: 
    /// * result: 
    pub fn SDL_IntersectRect(
        A: *const SDL_Rect,
        B: *const SDL_Rect,
        result: *mut SDL_Rect,
    ) -> i32;

    /// * A: 
    /// * B: 
    /// * result: 
    pub fn SDL_UnionRect(
        A: *const SDL_Rect,
        B: *const SDL_Rect,
        result: *mut SDL_Rect,
    ) -> c_void;

    /// * points: 
    /// * count: 
    /// * clip: 
    /// * result: 
    pub fn SDL_EnclosePoints(
        points: *const SDL_Point,
        count: i32,
        clip: *const SDL_Rect,
        result: *mut SDL_Rect,
    ) -> i32;

    /// * rect: 
    /// * X1: 
    /// * Y1: 
    /// * X2: 
    /// * Y2: 
    pub fn SDL_IntersectRectAndLine(
        rect: *const SDL_Rect,
        X1: *mut i32,
        Y1: *mut i32,
        X2: *mut i32,
        Y2: *mut i32,
    ) -> i32;
}
