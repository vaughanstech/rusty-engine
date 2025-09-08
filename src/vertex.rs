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
    pub(crate) position: [f32; 3], pub(crate) // x, y, z coordinates
    color: [f32; 3], // RGB color
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position attribute @location(0)
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0, // matches shader @location(0)
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Color @location(1)
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
