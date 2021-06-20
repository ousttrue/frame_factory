use std::io::stderr;
use std::io::stdout;
use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match clanggen::run(&args[1..]) {
        Ok(_) => println!("ok"),
        Err(err) => println!("error: {:?}", err),
    }

    stderr().flush().unwrap();    
    stdout().flush().unwrap();    
}
