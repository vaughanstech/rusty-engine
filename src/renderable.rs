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

pub struct Renderable {
    pub vertex_buffer: wgpu::Buffer, // vertex data
    pub index_buffer: Option<wgpu::Buffer>, // optional
    pub num_vertices: u32, // counts for draw cells
    pub num_indices: u32, // counts for draw cells
    pub uniform_buffer: wgpu::Buffer, // handles transform
    pub bind_group: wgpu::BindGroup, // handles transform
    pub transform: glam::Mat4, // CPU-side state, updated in update()
}

impl Renderable {
    pub fn new(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        vertices: &[Vertex],
        indices: Option<&[u16]>,
        initial_transform: glam::Mat4,
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
            (None, 0)
        };

        // Uniform buffer
        let uniforms = Uniforms::from_mat4(initial_transform);
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform Buind Group"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        Self {
            vertex_buffer,
            index_buffer,
            num_vertices: vertices.len() as u32,
            num_indices,
            uniform_buffer,
            bind_group,
            transform: initial_transform,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue) {
        let uniforms = Uniforms::from_mat4(self.transform);
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        if let Some(index_buf) = &self.index_buffer {
            render_pass.set_index_buffer(index_buf.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        } else {
            render_pass.draw(0..self.num_vertices, 0..1);
        }
    } 
}
