/*
Purpose: Defines your vertex format
Responsibilities:
    - Define the Vertex struct (e.g., positon, color, maybe normals)
    - Implement Vertex::desc() tells WGPU how to read buffer data
    - ex: DNA of an object (what it is made up of)
*/

use glam::Vec3;

// Create a struct to hold the vertices of a triangle
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3], // x, y, z coordinates
    pub color: [f32; 3], // RGB color
    pub tex_coords: [f32; 2], // support for texture coordinates
    pub normal: [f32; 3],
}

impl Vertex {
    pub const ATTRIBS: [wgpu::VertexAttribute; 4] =
        wgpu::vertex_attr_array![
            0 => Float32x3, // position
            1 => Float32x3, // color
            2 => Float32x2, // tex_coords
            3 => Float32x3,
        ];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }

    pub fn compute_normals(vertices: &mut [Vertex], indices: &[u16]) {
        // Reset all normals
        for v in vertices.iter_mut() {
            v.normal = [0.0, 0.0, 0.0];
        }

        // Accumulate face normals
        for tri in indices.chunks_exact(3) {
            let (i0, i1, i2) = (tri[0] as usize, tri[1] as usize, tri[2] as usize);

            let p0 = Vec3::from(vertices[i0].position);
            let p1 = Vec3::from(vertices[i1].position);

            let face_normal = Vec3::from(p1 - p0).normalize();

            for &i in &[i0, i1, i2] {
                let mut n = Vec3::from(vertices[i].normal);
                n += face_normal;
                vertices[i].normal = n.into();
            }
        }
        
        // Normalize all accumulated normals
        for v in vertices.iter_mut() {
            v.normal = Vec3::from(v.normal).normalize().into();
        }
    }
}
