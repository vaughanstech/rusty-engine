

// Describing each instance
pub struct Instance {
    pub position: cgmath::Vector3<f32>,
    pub rotation: Option<cgmath::Quaternion<f32>>,
}

// To avoid writing the math in the shader, we will store Instance data into a matrix
// This is the data that will go in wgpu::Buffer
// We keep these separate so that we can update the Instance as much as we want without messing with matrices
// Only need to update the raw data
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    model: [[f32; 4]; 4],
}

// Create method to convert Instance to InstanceRaw
impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        let model_matrix = match self.rotation {
            Some(r ) => {
                cgmath::Matrix4::from_translation(self.position) * cgmath::Matrix4::from(r)
            }
            None => {
                cgmath::Matrix4::from_translation(self.position)
            }
        };

        InstanceRaw {
            model: model_matrix.into(),
        }
            
    }
}

// impl Instance {
//     pub fn to_raw(&self) -> InstanceRaw {
//         InstanceRaw {
//             model: (cgmath::Matrix4::from_translation(self.position)
//                 * cgmath::Matrix4::from(self.rotation))
//             .into(),
//         }
//     }
// }

impl InstanceRaw {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            // Need to switch from using step mode of Vertex to Instance
            // This means that the shaders will only change to use the next instance
            // when the shader starts processing a new instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // A mat4takes up 4 vertex slots as it is technically 4 vec4s
                // Need to define a slot for each vec4
                // Will have to reassemble the mat4 in the shader
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ]
        }
    }
}