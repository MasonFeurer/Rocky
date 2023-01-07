use crate::Vec3;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pos: [f32; 3],
    tex_coords: [f32; 2],
}
impl Vertex {
    #[inline(always)]
    pub const fn new(pos: [f32; 3], tex_coords: [f32; 2]) -> Self {
        Self {
            pos: pos,
            tex_coords: tex_coords,
        }
    }

    pub const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    pub fn buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub const NUM_INDICES: u32 = 6 * 6;
pub fn default_model() -> (Vec<Vertex>, Vec<u16>) {
    let (from, to) = (Vec3::all(0.0), Vec3::all(1.0));
    let mut vertices = vec![];
    let mut indices = vec![];
    let mut next_index = 0;
    let mut quad = |verts: [Vec3<f32>; 4]| {
        vertices.push(Vertex::new(verts[0].into(), [1.0, 0.0]));
        vertices.push(Vertex::new(verts[1].into(), [0.0, 0.0]));
        vertices.push(Vertex::new(verts[2].into(), [0.0, 1.0]));
        vertices.push(Vertex::new(verts[3].into(), [1.0, 1.0]));
        indices.extend([
            next_index + 0,
            next_index + 1,
            next_index + 2,
            next_index + 3,
            next_index + 0,
            next_index + 2,
        ]);
        next_index += 4;
    };

    quad(px_face_verts(from, to));
    quad(nx_face_verts(from, to));
    quad(pz_face_verts(from, to));
    quad(nz_face_verts(from, to));
    quad(py_face_verts(from, to));
    quad(ny_face_verts(from, to));

    (vertices, indices)
}

pub fn px_face_verts<T: Copy>(from: Vec3<T>, to: Vec3<T>) -> [Vec3<T>; 4] {
    [
        Vec3::new(to.x, to.y, from.z),   // X1 Y1 Z0
        Vec3::new(to.x, to.y, to.z),     // X1 Y1 Z1
        Vec3::new(to.x, from.y, to.z),   // X1 Y0 Z1
        Vec3::new(to.x, from.y, from.z), // X1 Y0 Z0
    ]
}
pub fn nx_face_verts<T: Copy>(from: Vec3<T>, to: Vec3<T>) -> [Vec3<T>; 4] {
    [
        Vec3::new(from.x, to.y, to.z),     // X0 Y1 Z1
        Vec3::new(from.x, to.y, from.z),   // X0 Y1 Z0
        Vec3::new(from.x, from.y, from.z), // X0 Y0 Z0
        Vec3::new(from.x, from.y, to.z),   // X0 Y0 Z1
    ]
}
pub fn pz_face_verts<T: Copy>(from: Vec3<T>, to: Vec3<T>) -> [Vec3<T>; 4] {
    [
        Vec3::new(to.x, to.y, to.z),     // X1 Y1 Z1
        Vec3::new(from.x, to.y, to.z),   // X0 Y1 Z1
        Vec3::new(from.x, from.y, to.z), // X0 Y0 Z1
        Vec3::new(to.x, from.y, to.z),   // X1 Y0 Z1
    ]
}
pub fn nz_face_verts<T: Copy>(from: Vec3<T>, to: Vec3<T>) -> [Vec3<T>; 4] {
    [
        Vec3::new(from.x, to.y, from.z),   // X0 Y1 Z0
        Vec3::new(to.x, to.y, from.z),     // X1 Y1 Z0
        Vec3::new(to.x, from.y, from.z),   // X1 Y0 Z0
        Vec3::new(from.x, from.y, from.z), // X0 Y0 Z0
    ]
}
pub fn py_face_verts<T: Copy>(from: Vec3<T>, to: Vec3<T>) -> [Vec3<T>; 4] {
    [
        Vec3::new(from.x, to.y, to.z),   // X0 Y1 Z1
        Vec3::new(to.x, to.y, to.z),     // X1 Y1 Z1
        Vec3::new(to.x, to.y, from.z),   // X1 Y1 Z0
        Vec3::new(from.x, to.y, from.z), // X0 Y1 Z0
    ]
}
pub fn ny_face_verts<T: Copy>(from: Vec3<T>, to: Vec3<T>) -> [Vec3<T>; 4] {
    [
        Vec3::new(from.x, from.y, from.z), // X0 Y0 Z0
        Vec3::new(to.x, from.y, from.z),   // X1 Y0 Z0
        Vec3::new(to.x, from.y, to.z),     // X1 Y0 Z1
        Vec3::new(from.x, from.y, to.z),   // X0 Y0 Z1
    ]
}
