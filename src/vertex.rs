// /*
// Purpose: Defines your vertex format
// Responsibilities:
//     - Define the Vertex struct (e.g., positon, color, maybe normals)
//     - Implement Vertex::desc() tells WGPU how to read buffer data
//     - ex: DNA of an object (what it is made up of)
// */

// use glam::Vec3;

// // Describe what the vertex should look like
// #[repr(C)]
// #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
// pub struct Vertex {
//     position: [f32; 3],
//     tex_coords: [f32; 2],
// }

// // data that will make up a shape
// pub const VERTICES: &[Vertex] = &[
//     // Changed
//     Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.00759614], }, // A
//     Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.43041354], }, // B
//     Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.949397], }, // C
//     Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.84732914], }, // D
//     Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.2652641], }, // E
// ];


// pub const INDICES: &[u16] = &[
//     0, 1, 4,
//     1, 2, 4,
//     2, 3, 4,
// ];
// pub const NUM_VERTICES: u32 = VERTICES.len() as u32;
// pub const NUM_INDICES: u32 = INDICES.len() as u32;

// impl Vertex {
//     pub fn desc() -> wgpu::VertexBufferLayout<'static> {
//         wgpu::VertexBufferLayout {
//             array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress, // defines how wide a vertex is
//             step_mode: wgpu::VertexStepMode::Vertex, // tells the pipeline whether each element in this buffer represents per-vertex data or per-instance data
//             attributes: &[ // describes individual parts of the vertex. usually 1:1 mapping with a struct's fields
//                 wgpu::VertexAttribute {
//                     offset: 0, // how many bytes until the next attribute starts
//                     shader_location: 0, // tells the shader what location to store this attribute at
//                     format: wgpu::VertexFormat::Float32x3, // the shape of the attribute
//                 },
//                 wgpu::VertexAttribute {
//                     offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress, // sum of the previous attributes size
//                     shader_location: 1,
//                     format: wgpu::VertexFormat::Float32x2,
//                 }
//             ]
//         }
//     }
// }


