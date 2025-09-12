/*
Purpose: Defines your vertex format
Responsibilities:
    - Define the Vertex struct (e.g., positon, color, maybe normals)
    - Implement Vertex::desc() tells WGPU how to read buffer data
    - ex: DNA of an object (what it is made up of)
*/

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
}
