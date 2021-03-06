use std::path::{Path, PathBuf};

pub struct Export {
    pub header: PathBuf,
    pub link: String,
}

pub struct Args {
    pub target: String,
    pub exports: Vec<Export>,
    pub compile_args: Vec<String>,
    pub out_dir: PathBuf,
}

impl Args {
    pub fn parse(args: &[String]) -> Args {
        let mut exports: Vec<Export> = Vec::new();
        let mut compile_args: Vec<String> = Vec::new();
        let mut dst = std::path::PathBuf::new();
        let mut target = "rust";

        for arg in args {
            if arg == "lua" {
                target = "lua";
            } else {
                match &arg[0..2] {
                    "-E" => {
                        let split: Vec<&str> = arg[2..].rsplitn(2, ",").collect();
                        exports.push(Export {
                            header: Path::new(split[1]).to_owned(),
                            link: split[0].to_owned(),
                        });
                    }
                    "-D" | "-I" => {
                        compile_args.push(arg.clone());
                    }
                    "-O" => {
                        dst = Path::new(&arg[2..]).to_owned();
                    }
                    _ => panic!(),
                }
            }
        }

        Args {
            target: target.to_owned(),
            exports,
            compile_args,
            out_dir: dst,
        }
    }

    pub fn find_export(&self, header: &Path) -> Option<&Export> {
        self.exports.iter().find(|x| header == x.header)
    }

    pub fn merge_export_header(&self) -> String {
        let mut buffer = String::new();
        for export in &self.exports {
            buffer.push_str(&format!(
                "#include \"{}\"\n",
                export.header.to_string_lossy()
            ));
        }
        buffer
    }
}
