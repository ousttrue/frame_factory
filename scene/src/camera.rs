use cgmath::{Angle, Matrix4, One, Vector3};

use super::screen_state::{MouseButtonFlags, ScreenState};

pub struct Camera {
    pub projection: cgmath::Matrix4<f32>,
    pub view: cgmath::Matrix4<f32>,

    // projection param
    pub fovy: cgmath::Deg<f32>,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,

    // view param
    pub last_mouse_x: i16,
    pub last_mouse_y: i16,
    pub shift: cgmath::Vector3<f32>,
    pub yaw: cgmath::Deg<f32>,
    pub pitch: cgmath::Deg<f32>,
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
            last_mouse_x: 0,
            last_mouse_y: 0,
            shift: Vector3::new(0f32, 0f32, 2f32),
            yaw: cgmath::Deg(0f32),
            pitch: cgmath::Deg(0f32),
        };
        camera.calc_projection();
        camera.calc_view();
        camera
    }

    fn calc_projection(&mut self) {
        self.projection = cgmath::perspective(self.fovy, self.aspect, self.near, self.far);
    }

    fn calc_view(&mut self) {
        self.view = cgmath::Matrix4::from_translation(-self.shift)
            * cgmath::Matrix4::from_angle_x(self.pitch)
            * cgmath::Matrix4::from_angle_y(self.yaw);
    }

    fn shift(&mut self, dx: f32, dy: f32) {
        let half_fovy = self.fovy / 2 as f32;
        self.shift.x += dx * self.shift.z * half_fovy.tan() * 2f32 * self.aspect;
        self.shift.y += dy * self.shift.z * half_fovy.tan() * 2f32;
    }

    fn yaw_pitch(&mut self, dx: cgmath::Deg<f32>, dy: cgmath::Deg<f32>) {
        self.yaw += dx;
        self.pitch += dy;
    }

    fn dolly(&mut self, d: i16) {
        if d > 0 {
            self.shift.z *= 0.9f32;
        } else if d < 0 {
            self.shift.z *= 1.1f32;
        }
    }

    pub fn update(&mut self, state: &ScreenState) {
        let dx = state.mouse_x - self.last_mouse_x;
        let dy = state.mouse_y - self.last_mouse_y;
        self.last_mouse_x = state.mouse_x;
        self.last_mouse_y = state.mouse_y;

        let aspect = state.width as f32 / state.height as f32;
        if aspect != self.aspect {
            self.aspect = aspect;
            self.calc_projection();
        }

        if state.mouse_flag.contains(MouseButtonFlags::MiddleDown) {
            // shift
            self.shift(
                -dx as f32 / state.width as f32,
                dy as f32 / state.height as f32,
            );
        }

        if state.mouse_flag.contains(MouseButtonFlags::RightDown) {
            // yaw, pitch
            self.yaw_pitch(cgmath::Deg(dx as f32), cgmath::Deg(dy as f32));
        }

        if state.mouse_flag.contains(MouseButtonFlags::WheelMinus) {
            // dolly
            self.dolly(-1);
        }

        if state.mouse_flag.contains(MouseButtonFlags::WheelPlus) {
            // dolly
            self.dolly(1);
        }

        self.calc_view();
    }
}
