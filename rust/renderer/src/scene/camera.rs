pub struct Camera {
    pub view: cgmath::Matrix4<f32>,
    pub projection: cgmath::Matrix4<f32>,
}

impl Camera {
    pub fn persepective(fovy: cgmath::Deg<f32>, aspect: f32, near: f32, far: f32) -> Camera {
        let fovy = cgmath::Rad::from(fovy);
        let projection: cgmath::Matrix4<f32> = cgmath::perspective(fovy, aspect, near, far);
        let view: cgmath::Matrix4<f32> =
            cgmath::Matrix4::from_translation(cgmath::Vector3::new(0f32, 0f32, -1.0f32));
        Camera { view, projection }
    }
}
