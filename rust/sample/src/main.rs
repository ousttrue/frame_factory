// mod window;
// use winapi::shared::windef::HWND;

// #[link(name = "renderer.dll", kind = "static")]
// extern "C" {
//     fn FRAME_FACTORY_initialize(hwnd: HWND) -> bool;

//     fn FRAME_FACTORY_sample_scene(
//         source: *const u8,
//         source_size: usize,
//         vs_main: *const u8,
//         ps_main: *const u8,
//     ) -> bool;

//     fn FRAME_FACTORY_sample_render();
// }

// extern crate cgmath;
// use cgmath::{Matrix, One};

// fn run() -> Result<(), Box<dyn std::error::Error>> {
//     let hwnd = window::create_window("WindowClass", "D3D11 Demo")?;

//     if !unsafe { FRAME_FACTORY_initialize(hwnd) } {
//         // error
//         panic!("FRAME_FACTORY_initialize");
//     }

//     let source = std::fs::read_to_string("shaders/mvp.hlsl")?;

//     if !unsafe {
//         FRAME_FACTORY_sample_scene(
//             source.as_ptr(),
//             source.bytes().len(),
//             "vsMain\0".as_ptr(),
//             "psMain\0".as_ptr(),
//         )
//     } {
//         // error
//         panic!("FRAME_FACTORY_sample_scene");
//     }

//     for _ in window::main_loop(hwnd) {
//         unsafe { FRAME_FACTORY_sample_render() };
//     }
//     Ok(())
// }

fn main() {
    // run().unwrap();
}
