// this is generated.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]        
use std::ffi::c_void;

use crate::ImDrawData;
extern crate va_list;

pub type ID3D11Device = c_void;
pub type ID3D11DeviceContext = c_void;

#[link(name = "imgui_static", kind = "static")]
extern "C" {

    /// * device: 
    /// * device_context: 
    #[link_name = "?ImGui_ImplDX11_Init@@YA_NPEAUID3D11Device@@PEAUID3D11DeviceContext@@@Z"]
    pub fn ImGui_ImplDX11_Init(
        device: *mut ID3D11Device,
        device_context: *mut ID3D11DeviceContext,
    ) -> bool;

    #[link_name = "?ImGui_ImplDX11_Shutdown@@YAXXZ"]
    pub fn ImGui_ImplDX11_Shutdown() -> c_void;

    #[link_name = "?ImGui_ImplDX11_NewFrame@@YAXXZ"]
    pub fn ImGui_ImplDX11_NewFrame() -> c_void;

    /// * draw_data: 
    #[link_name = "?ImGui_ImplDX11_RenderDrawData@@YAXPEAUImDrawData@@@Z"]
    pub fn ImGui_ImplDX11_RenderDrawData(
        draw_data: *mut ImDrawData,
    ) -> c_void;

    #[link_name = "?ImGui_ImplDX11_InvalidateDeviceObjects@@YAXXZ"]
    pub fn ImGui_ImplDX11_InvalidateDeviceObjects() -> c_void;

    #[link_name = "?ImGui_ImplDX11_CreateDeviceObjects@@YA_NXZ"]
    pub fn ImGui_ImplDX11_CreateDeviceObjects() -> bool;
}
