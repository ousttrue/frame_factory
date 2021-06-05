use std::path::{Path, PathBuf};

pub struct Export {
    pub header: PathBuf,
    pub dll: String,
}

pub struct Args {
    pub exports: Vec<Export>,
    pub compile_args: Vec<String>,
    pub out: PathBuf,
}

impl Args {
    pub fn parse(args: &[String]) -> Args {
        let mut exports: Vec<Export> = Vec::new();
        let mut compile_args: Vec<String> = Vec::new();
        let mut dst = std::path::PathBuf::new();

        for arg in args {
            match &arg[0..2] {
                "-E" => {
                    let split: Vec<&str> = arg[2..].rsplitn(2, ",").collect();
                    exports.push(Export {
                        header: Path::new(split[1]).to_owned(),
                        dll: split[0].to_owned(),
                    });
                }
                "-D" => {
                    compile_args.push(arg.clone());
                }
                "-O" => {
                    dst = Path::new(&arg[2..]).to_owned();
                }
                _ => panic!(),
            }
        }

        Args {
            exports,
            compile_args,
            out: dst,
        }
    }

    pub fn find_export(&self, header: &Path) -> Option<&Export> {
        self.exports.iter().find(|x| header == x.header)
    }
}
