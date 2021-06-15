use std::ptr;

extern crate imgui_raw;

fn main() {
    // imgui__raw::IMGUI_CHECKVERSION();
    unsafe {
        imgui_raw::CreateContext(ptr::null_mut());
        let io = imgui_raw::GetIO();
        let a = 0;
    }
}
