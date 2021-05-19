use gltf::glTF;

extern crate gltf;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let json =std::fs::read_to_string(&args[1]).unwrap();
    let deserialized: glTF = serde_json::from_str(&json).unwrap();
    println!("deserialized = {:?}", deserialized);
}
