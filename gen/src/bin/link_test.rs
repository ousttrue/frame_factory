use std::ptr;
use gen::imgui;


fn main() {
    // imgui__raw::IMGUI_CHECKVERSION();
    unsafe {
        imgui::CreateContext(ptr::null_mut());
        let _io = imgui::GetIO();
        let _a = 0;
    }
}
