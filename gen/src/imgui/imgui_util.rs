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


#[link(name = "imgui_static", kind = "static")]
extern "C" {

    /// * pOut: 
    #[link_name = "?pGetContentRegionAvail@ImGui@@YAXPEAUImVec2@@@Z"]
    pub fn pGetContentRegionAvail(
        pOut: *mut ImVec2,
    ) -> c_void;

    /// * pOut: 
    #[link_name = "?pGetWindowPos@ImGui@@YAXPEAUImVec2@@@Z"]
    pub fn pGetWindowPos(
        pOut: *mut ImVec2,
    ) -> c_void;
}
