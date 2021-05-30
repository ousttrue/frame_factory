fn main() {
    let args: Vec<String> = std::env::args().collect();

    match from_cpp_header::run(&args[1..]) {
        Ok(_) => print!("ok"),
        Err(err) => print!("error: {:?}", err),
    }
}
