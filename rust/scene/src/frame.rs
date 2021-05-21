#[repr(C)]
pub struct c0 {
    pub view: [f32; 16],
    pub projection: [f32; 16],
}

#[allow(dead_code)]
#[repr(C)]
pub struct c1 {
    pub model: [f32; 16],
}
