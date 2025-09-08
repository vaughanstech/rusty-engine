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

#[allow(dead_code)]
pub struct Renderable {
    pub vertex_buffer: wgpu::Buffer, // vertex data
    pub index_buffer: Option<wgpu::Buffer>, // optional
    pub num_indices: u32, // counts for draw cells
    pub uniform_buffer: wgpu::Buffer, // handles transform
    pub uniform_bind_group: wgpu::BindGroup, // handles transform
    pub position: glam::Vec3,
    pub rotation: glam::Vec3, // rotation in radians (x, y, z)
    pub rotation_speed: glam::Vec3, // how fast to rotate around each axis
    pub scale: glam::Vec3,
}

impl Renderable {
    pub fn new(
        device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _render_pipeline: &wgpu::RenderPipeline,
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
        vertices: &[Vertex],
        indices: Option<&[u16]>,
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
        let (index_buffer, num_indices) = if let Some(idx) = indices {
            let buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(idx),
                usage: wgpu::BufferUsages::INDEX,
            });
            (Some(buf), idx.len() as u32)
        } else {
            (None, vertices.len() as u32)
        };

        // Uniform buffer
        let uniforms = Uniforms::new();
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Uniform Bind group
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform Buind Group"),
            layout: uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        Self {
            vertex_buffer,
            index_buffer,
            num_indices,
            uniform_buffer,
            uniform_bind_group,
            position,
            rotation: glam::Vec3::ZERO, // start with no rotation
            rotation_speed,
            scale
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
    pub fn update(&self, queue: &wgpu::Queue, time: f32, view_proj: glam::Mat4) {
        let mut uniforms = Uniforms::new();
        let model = self.model_matrix(time);
        let mvp = view_proj * model;
        uniforms.update_model(mvp);

        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
    }

    // Issue draw call
    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        if let Some(index_buf) = &self.index_buffer {
            render_pass.set_index_buffer(index_buf.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        } else {
            render_pass.draw(0..self.num_indices, 0..1);
        }
    } 
}
