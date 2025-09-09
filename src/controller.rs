use winit::event::{DeviceEvent, ElementState, MouseButton};

use crate::camera::Camera;

pub struct Controller {
    pub speed: f32,
    pub sensitivity: f32,

    w_pressed: bool,
    s_pressed: bool,
    a_pressed: bool,
    d_pressed: bool,
    up_pressed: bool,
    down_pressed: bool,

    yaw: f32,
    pitch: f32,
    last_mouse_position: Option<(f64, f64)>,
    mouse_pressed: bool,
}

impl Controller {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            speed,
            sensitivity,
            w_pressed: false,
            s_pressed: false,
            a_pressed: false,
            d_pressed: false,
            up_pressed: false,
            down_pressed: false,
            yaw: -90.0, // facing -Z by default
            pitch: 0.0,
            last_mouse_position: None,
            mouse_pressed: false,
        }
    }

    pub fn process_events(&mut self, event: &winit::event::WindowEvent) -> bool {

        match event {
            winit::event::WindowEvent::KeyboardInput {event, ..} => {
                use winit::keyboard::{KeyCode, PhysicalKey};
                let is_pressed = event.state == ElementState::Pressed;
                if let PhysicalKey::Code(code) = event.physical_key {
                    match code {
                        KeyCode::KeyW => {self.w_pressed = is_pressed; true}
                        KeyCode::KeyA => {self.a_pressed = is_pressed; true}
                        KeyCode::KeyS => {self.s_pressed = is_pressed; true}
                        KeyCode::KeyD => {self.d_pressed = is_pressed; true}
                        KeyCode::ArrowUp => {self.up_pressed = is_pressed; true}
                        KeyCode::ArrowDown => {self.down_pressed = is_pressed; true}
                        _ => false
                    }
                } else {false}
            }
            winit::event::WindowEvent::MouseInput {button: MouseButton::Left, state, ..} => {
                self.mouse_pressed = *state == ElementState::Pressed;
                true
            }
            _ => false,
        }
    }

    // Handle raw mouse movement events
    pub fn process_device_event(&mut self, event: &DeviceEvent) -> bool {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                if self.mouse_pressed {
                    let (dy, dx) = *delta;
                    self.yaw += (dx as f32) * self.sensitivity;
                    self.pitch -= (dy as f32) * self.sensitivity;

                    // clamp pitch to avoid gimbal lock
                    self.pitch = self.pitch.clamp(-89.0, 89.0);
                }
                true
            }
            _ => false,
        }
    }

    pub fn update_camera(&self, camera: &mut Camera, dt: f32) {
        let forward = (camera.target - camera.eye).normalize();
        let right = forward.cross(camera.up).normalize();

        let mut new_eye = camera.eye;
        if self.w_pressed {
            new_eye += forward * self.speed * dt;
        }
        if self.s_pressed {
            new_eye -= forward * self.speed * dt;
        }
        if self.a_pressed {
            new_eye -= right * self.speed * dt;
        }
        if self.d_pressed {
            new_eye += right * self.speed * dt;
        }
        if self.up_pressed {
            new_eye.y += self.speed * dt;
        }
        if self.down_pressed {
            new_eye.y -= self.speed * dt;
        }

        // update camera eye
        camera.eye = new_eye;

        // update target based on yaw/pitch
        let yaw_rad = self.yaw.to_radians();
        let pitch_rad = self.pitch.to_radians();
        let dir = glam::Vec3::new(
            -pitch_rad.sin(),
            yaw_rad.cos() * pitch_rad.cos(),
            yaw_rad.sin() * pitch_rad.cos(),
        ).normalize();
        camera.target = camera.eye + dir;
    }
}