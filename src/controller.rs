use winit::{event::{ElementState, KeyEvent}};

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
        }
    }

    pub fn process_events(&mut self, event: &winit::event::WindowEvent) -> bool {
        use winit::{
            keyboard::{KeyCode}
        };

        if let winit::event::WindowEvent::KeyboardInput {
            event: KeyEvent {
                physical_key: winit::keyboard::PhysicalKey::Code(keycode),
                state,
                ..
            },
            ..
        } = event
        {
            let is_pressed = *state == ElementState::Pressed;
            match keycode {
                KeyCode::KeyW => {self.w_pressed = is_pressed; true}
                KeyCode::KeyA => {self.a_pressed = is_pressed; true}
                KeyCode::KeyS => {self.s_pressed = is_pressed; true}
                KeyCode::KeyD => {self.d_pressed = is_pressed; true}
                KeyCode::ArrowUp => {self.up_pressed = is_pressed; true}
                KeyCode::ArrowDown => {self.down_pressed = is_pressed; true}
                _ => false,
            }
        } else {
            false
        }
    }

    // pub fn process_events(&mut self, event: &winit::event::WindowEvent) -> bool {
    //     use winit::{
    //             event::{ElementState, KeyEvent, WindowEvent},
    //             keyboard::{KeyCode, PhysicalKey,},
    //         };

    //     match event {
    //         WindowEvent::KeyboardInput {
    //             event: KeyEvent {
    //                 state: ElementState::Pressed,
    //                 physical_key: PhysicalKey::Code(KeyCode::KeyW),
    //                 ..
    //             },
    //             ..
    //         } => {self.w_pressed == true},
    //         WindowEvent::KeyboardInput {
    //             event: KeyEvent {
    //                 state: ElementState::Pressed,
    //                 physical_key: PhysicalKey::Code(KeyCode::KeyS),
    //                 ..
    //             },
    //             ..
    //         } => {self.s_pressed == true},
    //         WindowEvent::KeyboardInput {
    //             event: KeyEvent {
    //                 state: ElementState::Pressed,
    //                 physical_key: PhysicalKey::Code(KeyCode::KeyA),
    //                 ..
    //             },
    //             ..
    //         } => {self.a_pressed == true},
    //         WindowEvent::KeyboardInput {
    //             event: KeyEvent {
    //                 state: ElementState::Pressed,
    //                 physical_key: PhysicalKey::Code(KeyCode::KeyD),
    //                 ..
    //             },
    //             ..
    //         } => {self.d_pressed == true},
    //         WindowEvent::KeyboardInput {
    //             event: KeyEvent {
    //                 state: ElementState::Pressed,
    //                 physical_key: PhysicalKey::Code(KeyCode::ArrowUp),
    //                 ..
    //             },
    //             ..
    //         } => {self.up_pressed == true},
    //         WindowEvent::KeyboardInput {
    //             event: KeyEvent {
    //                 state: ElementState::Pressed,
    //                 physical_key: PhysicalKey::Code(KeyCode::ArrowDown),
    //                 ..
    //             },
    //             ..
    //         } => {self.down_pressed == true},
    //         _ => {false}
    //     }
    // }

    pub fn update_camera(&self, camera: &mut Camera, dt: f32) {
        let forward = (camera.target - camera.eye).normalize();
        let right = forward.cross(camera.up).normalize();

        if self.w_pressed {
            camera.eye += forward * self.speed * dt;
            camera.target += forward * self.speed * dt;
        }
        if self.s_pressed {
            camera.eye -= forward * self.speed * dt;
            camera.target -= forward * self.speed * dt;
        }
        if self.a_pressed {
            camera.eye -= right * self.speed * dt;
            camera.target -= right * self.speed * dt;
        }
        if self.d_pressed {
            camera.eye += right * self.speed * dt;
            camera.target += right * self.speed * dt;
        }
        if self.up_pressed {
            camera.eye.y += self.speed * dt;
            camera.target.y += self.speed * dt;
        }
        if self.down_pressed {
            camera.eye.y -= self.speed * dt;
            camera.target.y -= self.speed * dt;
        }
    }
}