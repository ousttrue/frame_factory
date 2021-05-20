extern crate gltf2;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let json = std::fs::read_to_string(&args[1]).unwrap();
    let deserialized: gltf2::glTF = serde_json::from_str(&json).unwrap();
    println!("deserialized = {:?}", deserialized);
}
