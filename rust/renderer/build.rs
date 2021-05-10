use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_lib_path = Path::new(&out_dir).join(
        "renderer.dll.lib",
    );
    // let dest_dll_path = Path::new(&out_dir).join("../../..").join("hoge.dll");
    // let _ = fs::copy("./vendor/hoge.dll", dest_dll_path);
    let _ = fs::copy("./target/debug/renderer.lib", dest_lib_path);
}
