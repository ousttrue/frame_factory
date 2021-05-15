use cgmath::{Matrix4, Vector3, One};

use super::screenstate::{MouseButtonFlags, ScreenState};

pub struct Camera {
    pub projection: cgmath::Matrix4<f32>,
    pub view: cgmath::Matrix4<f32>,

    // projection param
    pub fovy: cgmath::Deg<f32>,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,

    // view param
    pub lastMouseX: i16,
    pub lastMouseY: i16,
    pub shift: cgmath::Vector3<f32>,
    pub yaw: f32,
    pub pitch: f32,
}

impl Camera {
    pub fn new() -> Camera {
        let mut camera = Camera {
            projection: Matrix4::<f32>::one(),
            view: Matrix4::<f32>::one(),
            //
            fovy: cgmath::Deg::<f32>(60f32),
            aspect: 1.0f32,
            near: 0.1f32,
            far: 100f32,
            //
            lastMouseX: 0,
            lastMouseY: 0,
            shift: Vector3::new(0f32, 0f32, 1f32),
            yaw: 0f32,
            pitch: 0f32,
        };
        camera.calc_projection();
        camera.calc_view();
        camera
    }

    fn calc_projection(&mut self)
    {
        self.projection = cgmath::perspective(self.fovy, self.aspect, self.near, self.far);
    }

    fn calc_view(&mut self) {
        self.view = cgmath::Matrix4::from_translation(-self.shift);
    }

    fn shift(&mut self, dx: i16, dy: i16) {}

    fn yawpitch(&mut self, dx: i16, dy: i16) {}

    fn dolly(&mut self, d: i16) {
        if d > 0 {
            self.shift.z *= 0.9f32;
        } else if d < 0 {
            self.shift.z *= 1.1f32;
        }
    }

    pub fn update(&mut self, state: &ScreenState) {
        let dx = state.MouseX - self.lastMouseX;
        let dy = state.MouseY - self.lastMouseY;
        self.lastMouseX = state.MouseX;
        self.lastMouseY = state.MouseY;

        let aspect = state.Width as f32/state.Height as f32;
        if aspect!=self.aspect
        {
            self.aspect = aspect;
            self.calc_projection();
        }

        if state.MouseFlag.has(MouseButtonFlags::MiddleDown) {
            // shift
            self.shift(dx, dy);
        }

        if state.MouseFlag.has(MouseButtonFlags::RightDown) {
            // yaw, pitch
            self.shift(dx, dy);
        }

        if state.MouseFlag.has(MouseButtonFlags::WheelMinus) {
            // dolly
            self.dolly(-1);
        }

        if state.MouseFlag.has(MouseButtonFlags::WheelPlus) {
            // dolly
            self.dolly(1);
        }

        self.calc_view();
    }
}
