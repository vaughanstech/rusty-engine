use glam::Mat4;

pub struct Camera {
    pub eye: glam::Vec3,
    pub target: glam::Vec3,
    pub up: glam::Vec3,
    pub aspect: f32,
    pub fov_y: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> Mat4 {
        // LookAt matrix
        let view = Mat4::look_at_rh(self.eye, self.target, self.up);

        // Perspective projection
        let proj = Mat4::perspective_rh_gl(self.fov_y, self.aspect, self.z_near, self.z_far);
        proj * view
    }
}
