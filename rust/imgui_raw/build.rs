fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();    
    let imgui_dir = format!("{}/../../../../../../cpp/build/Debug", out_dir);

    println!(
        "cargo:rustc-link-search=native={}/lib",
        imgui_dir
    );
}
