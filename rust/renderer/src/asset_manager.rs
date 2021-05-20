use std::{collections::HashMap, io};

pub struct ShaderSource {
    pub source: String,
    pub vs_main: String,
    pub ps_main: String,
}

impl ShaderSource {
    pub fn new(source: String, vs_main: String, ps_main: String) -> ShaderSource {
        ShaderSource {
            source,
            vs_main,
            ps_main,
        }
    }
}

pub struct AssetManager {
    pub asset_path: String,
    shader_sources: HashMap<String, ShaderSource>,
}

impl AssetManager {
    pub fn get_shader_source(&mut self, path: &str) -> io::Result<&ShaderSource> {
        if !self.shader_sources.contains_key(path) {
            let full_path = format!("{}/{}", &self.asset_path, path);
            let source = std::fs::read_to_string(full_path)?;
            self.shader_sources.insert(
                path.to_owned(),
                ShaderSource::new(source, "vsMain\0".to_owned(), "psMain\0".to_owned()),
            );
        }

        Ok(self.shader_sources.get(path).unwrap())
    }
}

static mut G_MANAGER: Option<Box<AssetManager>> = None;

pub fn get() -> Option<&'static mut Box<AssetManager>> {
    unsafe {
        if G_MANAGER.is_none() {
            G_MANAGER = Some(Box::new(AssetManager {
                asset_path: "".to_owned(),
                shader_sources: HashMap::new(),
            }))
        }
    }

    if let Some(asset_manager) = unsafe { &mut G_MANAGER } {
        Some(asset_manager)
    } else {
        None
    }
}

pub fn shutdown() {
    unsafe { G_MANAGER = None };
}
