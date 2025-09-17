use crate::state::{State};
use pollster::FutureExt;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{DeviceEvent, ElementState, KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{WindowAttributes,CursorGrabMode},
};

pub struct App {
    state: Option<State>,
    cursor_locked: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: None,
            cursor_locked: false,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_title("Rusty Engine")
            .with_inner_size(PhysicalSize::new(800, 600));
        let window = event_loop.create_window(window_attributes).unwrap();
        // Try to confine or lock the cursor to the window
        if window.set_cursor_grab(CursorGrabMode::Confined).is_err() {
            // Fallback if platform doesn't support confinement
            let _ = window.set_cursor_grab(CursorGrabMode::Locked);
        }
        self.state = Some(State::new(window).block_on());
    }

    fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            _window_id: winit::window::WindowId,
            event: WindowEvent,
        ) {
            if let Some(state) = self.state.as_mut() {
                if state.input(&event) {
                    return;
                }
            };

            match event {
                WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            ..
                        },
                    ..
                } => event_loop.exit(),
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            physical_key: PhysicalKey::Code(KeyCode::KeyL),
                            ..
                        },
                    ..
                } => {
                    if let Some(state) = self.state.as_mut() {
                        let window = state.window();
                        if self.cursor_locked {
                            // Unlock
                            let _ = window.set_cursor_grab(CursorGrabMode::None);
                            self.cursor_locked = false;
                        } else {
                            // Lock
                            if window.set_cursor_grab(CursorGrabMode::Confined).is_err() {
                                let _ = window.set_cursor_grab(CursorGrabMode::Locked);
                            }
                            self.cursor_locked = true;
                        }
                    }
                }
                WindowEvent::RedrawRequested => {
                    if let Some(state) = self.state.as_mut() {
                        let time = std::time::Instant::now();
                        state.window().request_redraw();
                        state.update();
                        state.update_lights(time.elapsed().as_secs_f32());
                        match state.render() {
                            Ok(_) => {}
                            // Reconfigure the surface if it's lost or outdated
                            Err(
                                wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
                            ) => state.resize(state.size),
                            // The system is out of memory, we should probably quit
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                log::error!("OutOfMemory");
                                event_loop.exit();
                            }
                            // This happens when the a frame takes too long to present
                            Err(wgpu::SurfaceError::Timeout) => {
                                log::warn!("Surface timeout")
                            }
                        }
                    }
                }
                WindowEvent::Resized(physical_size) => {
                    if let Some(state) = self.state.as_mut() {
                        state.resize(physical_size);
                    }
                }
                _ => {
                    if let Some(state) = self.state.as_mut() {
                        state.controller.process_events(&event);
                    }
                }
            }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        if let Some(state) = self.state.as_mut() {
            state.controller.process_device_event(&event);
        }
    }
}
