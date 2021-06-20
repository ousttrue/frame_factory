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

pub type SDL_Window = c_void;
pub type SDL_Event = c_void;

#[link(name = "imgui_static", kind = "static")]
extern "C" {

    /// * window: 
    /// * sdl_gl_context: 
    #[link_name = "?ImGui_ImplSDL2_InitForOpenGL@@YA_NPEAUSDL_Window@@PEAX@Z"]
    pub fn ImGui_ImplSDL2_InitForOpenGL(
        window: *mut SDL_Window,
        sdl_gl_context: *mut c_void,
    ) -> bool;

    /// * window: 
    #[link_name = "?ImGui_ImplSDL2_InitForVulkan@@YA_NPEAUSDL_Window@@@Z"]
    pub fn ImGui_ImplSDL2_InitForVulkan(
        window: *mut SDL_Window,
    ) -> bool;

    /// * window: 
    #[link_name = "?ImGui_ImplSDL2_InitForD3D@@YA_NPEAUSDL_Window@@@Z"]
    pub fn ImGui_ImplSDL2_InitForD3D(
        window: *mut SDL_Window,
    ) -> bool;

    /// * window: 
    #[link_name = "?ImGui_ImplSDL2_InitForMetal@@YA_NPEAUSDL_Window@@@Z"]
    pub fn ImGui_ImplSDL2_InitForMetal(
        window: *mut SDL_Window,
    ) -> bool;

    #[link_name = "?ImGui_ImplSDL2_Shutdown@@YAXXZ"]
    pub fn ImGui_ImplSDL2_Shutdown() -> c_void;

    /// * window: 
    #[link_name = "?ImGui_ImplSDL2_NewFrame@@YAXPEAUSDL_Window@@@Z"]
    pub fn ImGui_ImplSDL2_NewFrame(
        window: *mut SDL_Window,
    ) -> c_void;

    /// * event: 
    #[link_name = "?ImGui_ImplSDL2_ProcessEvent@@YA_NPEBTSDL_Event@@@Z"]
    pub fn ImGui_ImplSDL2_ProcessEvent(
        event: *const SDL_Event,
    ) -> bool;
}
