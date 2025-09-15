#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Light {
    pub position: [f32; 3],
    pub intensity: f32,
    pub color: [f32; 3],
    pub _padding: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Lights {
    pub lights: [Light; 16],
    pub num_lights: u32,
    pub _padding: [u32; 3],
}
