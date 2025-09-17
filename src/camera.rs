use glam::Mat4;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        let view = Mat4::look_at_rh(camera.eye, camera.target, camera.up);

        // Use perspective_rh_gl for consistency
        let proj = Mat4::perspective_rh_gl(
            camera.fov_y,
            camera.aspect,
            camera.z_near,
            camera.z_far,
        );

        // Depending on the GPU backend, Y may be flipped
        self.view_proj = (proj * view).to_cols_array_2d();
    }
}

pub enum Projection {
    Orthographic,
    Perspective,
}

pub struct Camera {
    pub eye: glam::Vec3, // Where the camera is located (its position in world space)
    pub target: glam::Vec3, // The point the camera is looking at
    pub up: glam::Vec3, // Which way is "up" for the camera
    pub aspect: f32,
    pub fov_y: f32,
    pub z_near: f32,
    pub z_far: f32,
    pub projection: Projection,
    pub _ortho_scale: f32,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        Self {
            eye: glam::vec3(0.0, 0.0, 10.0),
            target: glam::Vec3::ZERO,
            up: glam::Vec3::Y,
            aspect,
            fov_y: 45f32.to_radians(),
            z_near: 0.1,
            z_far: 100.0,
            projection: Projection::Orthographic, // default
            _ortho_scale: 100.0,
        }
    }
    // pub fn build_view_projection_matrix(&self) -> Mat4 {
    //     // LookAt matrix
    //     let view = Mat4::look_at_rh(self.eye, self.target, self.up);

    //     let proj = match self.projection {
    //         Projection::Perspective => Mat4::perspective_rh_gl(
    //             self.fov_y,
    //             self.aspect,
    //             self.z_near,
    //             self.z_far,
    //         ),
    //         Projection::Orthographic => {
    //             let ortho_scale = self.ortho_scale; //zoom level
    //             Mat4::orthographic_rh_gl(
    //                 -self.aspect * ortho_scale,
    //                 self.aspect * ortho_scale,
    //                 -ortho_scale,
    //                 ortho_scale,
    //                 self.z_near,
    //                 self.z_far,
    //             )
    //         }
    //     };

    //     proj * view
    // }

    pub fn toggle_projection(&mut self) {
        self.projection = match self.projection {
            Projection::Orthographic => Projection::Perspective,
            Projection::Perspective => Projection::Orthographic,
        };
    }
}
