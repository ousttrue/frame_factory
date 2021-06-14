use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let root_dir = Path::new(&out_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    assert!(root_dir.exists());
    let imgui_src_dir = format!("{}/cpp/_external/imgui", root_dir.to_string_lossy());
    let sdl_src_dir = format!("{}/cpp/_external/sdl", root_dir.to_string_lossy());

    // println!(
    //     "cargo:rustc-link-search=native={}/lib",
    //     imgui_dir
    // );

    cc::Build::new()
        .cpp(true)
        // .warnings(true)
        // .flag("-std=c++17")
        // .flag("-Wall")
        // .flag("-Wextra")
        // .flag("-v")
        // .flag("-g")
        // .file("src/cpp/src/hello.cpp")
        .file(format!("{}/imgui.cpp", imgui_src_dir))
        .file(format!("{}/imgui_widgets.cpp", imgui_src_dir))
        .file(format!("{}/imgui_tables.cpp", imgui_src_dir))
        .file(format!("{}/imgui_draw.cpp", imgui_src_dir))
        .file(format!("{}/imgui_demo.cpp", imgui_src_dir))
        .file(format!("{}/backends/imgui_impl_win32.cpp", imgui_src_dir))
        .file(format!("{}/backends/imgui_impl_dx11.cpp", imgui_src_dir))
        .include(format!("{}", imgui_src_dir))
        .file(format!("{}/backends/imgui_impl_sdl.cpp", imgui_src_dir))
        .include(format!("{}/include", sdl_src_dir))
        .compile("imgui_static");
}
