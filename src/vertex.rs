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
    pub(crate) position: [f32; 2], pub(crate) // x, y coordinates
    color: [f32; 3], // RGB color
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position attribute
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0, // matches shader @location(0)
                    format: wgpu::VertexFormat::Float32x2,
                },
                // Color
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}
