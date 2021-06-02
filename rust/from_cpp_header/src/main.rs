use std::io::stderr;
use std::io::stdout;
use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match from_cpp_header::run(&args[1..]) {
        Ok(_) => print!("ok"),
        Err(err) => print!("error: {:?}", err),
    }

    stderr().flush().unwrap();    
    stdout().flush().unwrap();    
}
