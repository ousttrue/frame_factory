use cgmath::One;
pub enum Shader {
    UnLight,
}

pub struct AccessorBytes {
    pub bytes: Vec<u8>,
    pub stride: u32,
    pub count: u32,
}

impl AccessorBytes {
    pub fn new(bytes: Vec<u8>, stride: u32, count: u32) -> AccessorBytes {
        AccessorBytes {
            bytes,
            stride,
            count,
        }
    }

    pub fn from<T: Sized>(src: &[T]) -> AccessorBytes {
        let stride = std::mem::size_of::<T>();
        let mut bytes: Vec<u8> = Vec::with_capacity(stride * src.len());
        let len = bytes.len();
        unsafe {
            std::ptr::copy_nonoverlapping(
                src.as_ptr() as *const std::ffi::c_void,
                bytes.as_mut_ptr() as *mut std::ffi::c_void,
                bytes.len(),
            );
        }
        Self::new(bytes, stride as u32, len as u32)
    }
}

pub struct Model {
    id: u32,
    pub transform: cgmath::Matrix4<f32>,
    pub positions: AccessorBytes,
    pub indices: AccessorBytes,
    pub material: Shader,
}

static mut g_next_id: u32 = 1;

impl Model {
    pub fn new(positions: AccessorBytes, indices: AccessorBytes, material: Shader) -> Model {
        let id = unsafe { g_next_id };
        unsafe { g_next_id += 1 };
        Model {
            id,
            transform: cgmath::Matrix4::<f32>::one(),
            positions,
            indices,
            material,
        }
    }

    pub fn create_triangle() -> Model {
        let size = 0.5f32;
        Self::new(
            AccessorBytes::from(&[
                (size, -size, 0.0f32),
                (-size, -size, 0.0f32),
                (0.0f32, size, 0.0f32),
            ]),
            AccessorBytes::from(&[0, 1, 2]),
            Shader::UnLight,
        )
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}
