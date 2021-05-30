extern crate clang_sys;

#[derive(Debug)]
enum Error {
    StaticMessage(&'static str),
}

fn run() -> Result<(), Error> {
    // get translation unit
    let index = unsafe { clang_sys::clang_createIndex(0, 1) };
    if index.is_null() {
        return Err(Error::StaticMessage("fail to clang_sys::clang_createIndex"));
    }

    Ok(())
}

fn main() {
    match run() {
        Ok(_) => print!("ok"),
        Err(err) => print!("error: {:?}", err),
    }
}
