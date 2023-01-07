use crate::math::{Mat4, Vec3};

/// Takes a rotation (the rotation around the X, Y, and Z axis), and
/// creates a normalized vector ray in the facing direction.<p>
/// the rotation values should be in radians (0..TAU)
pub fn axis_rot_to_ray(rot: Vec3<f32>) -> Vec3<f32> {
    // the Z rotation doesn't effect the ray
    // the Y rotation effects the ray's X and Z
    // the X rotation effects the ray's X, Y, and Z

    // the ray's X, Z is along the edge of a circle, cutting through the y-axis, radius R
    // R goes follows 0..1..0, and is derived from the X rotation (aka the vertical tilt)
    // the ray's Y is also derived from the X rotation

    // radius of Y axis cross-sections
    let r = rot.x.cos();
    let x = r * -rot.y.sin();
    let z = r * -rot.y.cos();
    let y = -rot.x.sin();
    Vec3 { x, y, z }
}

#[derive(Clone)]
pub struct Cam {
    pub pos: Vec3<f32>,
    // rotation about the X, Y, and Z axes, in degrees
    pub rot: Vec3<f32>,
}
impl Cam {
    pub fn new(pos: Vec3<f32>, rot: Vec3<f32>) -> Self {
        Self { pos, rot }
    }

    #[inline(always)]
    pub fn matrix(&self) -> Mat4 {
        Mat4::view(self.pos, self.rot.map(f32::to_radians))
    }

    #[inline(always)]
    pub fn dir(&self) -> Vec3<f32> {
        axis_rot_to_ray(self.rot.map(|e| e.to_radians()))
    }
}
