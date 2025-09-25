/*
Purpose: Represents a single drawable object (triangle, square, circle, mesh)
Responsibilities:
    - Store its vertex/index buffers
    - Own its uniform buffer (transform)
    - Implement update() (sync CPU transform -> GPU)
    - Implement draw() (set buffers and issue draw call)
*/

use crate::uniforms::{Uniforms};
use crate::vertex::Vertex;
use wgpu::util::DeviceExt;

// Defining a material abstraction for renderables
#[repr(C)] // ensures memory layout is C-compatible
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MaterialUniform {
    pub use_texture: u32,
    _padding: [u32; 7],
}

#[allow(dead_code)]
pub struct Renderable {
    pub vertex_buffer: wgpu::Buffer, // vertex data
    pub index_buffer: wgpu::Buffer, // optional
    pub num_indices: u32, // counts for draw cells
    pub texture_bind_group: Option<wgpu::BindGroup>, // None = no texture
    pub uniform_buffer: wgpu::Buffer, // handles transform
    pub material_buffer: wgpu::Buffer,
    pub uniform_material_bind_group: wgpu::BindGroup, // handles transform
    pub position: glam::Vec3,
    pub rotation: glam::Vec3, // rotation in radians (x, y, z)
    pub rotation_speed: glam::Vec3, // how fast to rotate around each axis
    pub scale: glam::Vec3,
    pub start_lit: bool,
    pub start_emission: bool,
    pub emissive_strength: f32,
    pub color: [f32; 3],
}

impl Renderable {
    pub fn new(
        device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _render_pipeline: &wgpu::RenderPipeline,
        uniform_material_bind_group_layout: &wgpu::BindGroupLayout,
        vertices: &[Vertex],
        indices: &[u16],
        texture_bind_group: Option<wgpu::BindGroup>,
        use_texture: bool,
        start_lit: bool,
        start_emission: bool,
        emissive_strength: f32,
        color: [f32; 3],
        position: glam::Vec3,
        rotation_speed: glam::Vec3,
        scale: glam::Vec3,
    ) -> Self {
        // Vertex buffer
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Index buffer (optional)
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            });
        let num_indices = indices.iter().len();

        // Uniform buffer
        let uniforms = Uniforms {
            mvp: glam::Mat4::IDENTITY.to_cols_array_2d(),
            lit: if start_lit { 1 } else { 0 },
            emissive: if start_emission { 1 } else { 0 },
            emissive_strength: emissive_strength,
            color: color,
            _padding: [0; 5],
        };
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Uniform Bind group
        // let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        //     label: Some("Uniform Buind Group"),
        //     layout: uniform_bind_group_layout,
        //     entries: &[wgpu::BindGroupEntry {
        //         binding: 0,
        //         resource: uniform_buffer.as_entire_binding(),
        //     }],
        // });

        // Material Uniform Buffer
        let material_uniform = MaterialUniform {
            use_texture: if use_texture { 1 } else { 0 },
            _padding: [0; 7],
        };
        let material_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Material Buffer"),
            contents: bytemuck::bytes_of(&material_uniform),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_material_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform + Material Bind Group"),
            layout: &uniform_material_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: material_buffer.as_entire_binding(),
                }
            ]
        });

        

        // let material = Self::create_material(device, material_layout, use_texture);

        Self {
            vertex_buffer,
            index_buffer,
            num_indices: num_indices.try_into().unwrap(),
            texture_bind_group,
            uniform_buffer,
            material_buffer,
            uniform_material_bind_group,
            position,
            rotation: glam::Vec3::ZERO, // start with no rotation
            rotation_speed,
            scale,
            start_lit,
            start_emission,
            emissive_strength,
            color,
        }
    }

    pub fn model_matrix(&self, time: f32) -> glam::Mat4 {
        // rotation around Z from now
        let rotation = glam::Mat4::from_rotation_x(time * self.rotation_speed.x) * glam::Mat4::from_rotation_y(time * self.rotation_speed.y) * glam::Mat4::from_rotation_z(time * self.rotation_speed.z);
        let translation = glam::Mat4::from_translation(self.position);
        let scaling = glam::Mat4::from_scale(self.scale);

        translation * rotation * scaling
    }

    // Update uniforms per frame
    pub fn update(&self, queue: &wgpu::Queue, time: f32) {
        let model = self.model_matrix(time);

        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[model.to_cols_array_2d()]));
    }

    // pub fn create_material(
    //     device: &wgpu::Device,
    //     layout: &wgpu::BindGroupLayout,
    //     use_texture: bool,
    // ) -> Material {
    //     let uniform = MaterialUniform {
    //         use_texture: if use_texture { 1 } else { 0 },
    //         _padding: [0; 3],
    //     };

    //     let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //         label: Some("Material Buffer"),
    //         contents: bytemuck::bytes_of(&uniform),
    //         usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    //     });

    //     let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
    //         label: Some("Material Bind Group"),
    //         layout,
    //         entries: &[wgpu::BindGroupEntry {
    //             binding: 0,
    //             resource: buffer.as_entire_binding(),
    //         }],
    //     });

    //     Material {
    //         uniform,
    //         buffer,
    //         bind_group,
    //     }
    // }

}
