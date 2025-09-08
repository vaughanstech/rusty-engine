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
    pub mvp: [[f32; 4]; 4],
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            mvp: glam::Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update_model(&mut self, mvp: glam::Mat4) {
        self.mvp = mvp.to_cols_array_2d();
    }
}