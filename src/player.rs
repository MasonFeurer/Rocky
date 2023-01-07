use crate::cam::Cam;
use crate::input::{FrameInput, Key};
use crate::math::{Vec2, Vec3};
use std::ops::Neg;

#[derive(Clone)]
pub struct Player {
    pub pos: Vec3<f32>,
    pub vel: Vec3<f32>,
    pub acc: Vec3<f32>,
    pub cam: Cam,
}
impl Player {
    pub fn new(pos: Vec3<f32>, rot: Vec3<f32>) -> Self {
        Self {
            pos,
            vel: Vec3::all(0.0),
            acc: Vec3::all(0.0),
            cam: Cam {
                pos: Vec3::all(0.0),
                rot,
            },
        }
    }

    pub fn move_cursor(&mut self, delta: Vec2<f64>) {
        const SENSITIVITY: f32 = 0.4;

        // in model space, the camera is looking negative along the Z axis, so
        // moving the cursor up/down corresponds to rotation about the X axis
        self.cam.rot.x -= SENSITIVITY * delta.y as f32;
        self.cam.rot.x = self.cam.rot.x.clamp(-90.0, 90.0);

        // moving the cursor left/right corresponds to rotation about the Y axis
        self.cam.rot.y += SENSITIVITY * delta.x as f32;

        // the camera does not rotate about the Z axis. That would be like tilting your head
    }

    pub fn update(&mut self, input: &FrameInput) {
        self.move_cursor(input.cursor_change().map(f64::neg));

        const SPEED: f32 = 0.1;
        let dx = SPEED * self.cam.rot.y.to_radians().sin();
        let dz = SPEED * self.cam.rot.y.to_radians().cos();
        let dy = SPEED;

        if input.key_pressed(Key::W) {
            self.pos.x -= dx;
            self.pos.z -= dz;
        }
        if input.key_pressed(Key::S) {
            self.pos.x += dx;
            self.pos.z += dz;
        }
        if input.key_pressed(Key::D) {
            self.pos.x += dz;
            self.pos.z -= dx;
        }
        if input.key_pressed(Key::A) {
            self.pos.x -= dz;
            self.pos.z += dx;
        }
        if input.key_pressed(Key::Space) {
            self.pos.y += dy;
        }
        if input.key_pressed(Key::LShift) {
            self.pos.y -= dy;
        }

        self.cam.pos = self.pos;
    }
}
