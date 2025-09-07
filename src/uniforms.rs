/*
Purpose: Defines your uniform data (things you send once per-object or per-frame)
Responsibilities:
    - Define Uniforms struct (transform matrix, camera, etc)
    - Implement helpers (Uniforms::new(), Uniforms::from_mat4())
    - Implement bytemuck::Pod + Zeroable for safe GPU transfer
    - ex: Backpack of data for each actor
*/

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    transform: [[f32; 4]; 4],
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            transform: glam::Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn from_mat4(mat: glam::Mat4) -> Self {
        Self {transform: mat.to_cols_array_2d()}
    }
}