use crate::state::State;
use pollster::FutureExt;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowAttributes,
};

pub struct App {
    state: Option<State>,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_title("Rusty Engine")
            .with_inner_size(PhysicalSize::new(800, 600));
        let window = event_loop.create_window(window_attributes).unwrap();
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
                WindowEvent::RedrawRequested => {
                    if let Some(state) = self.state.as_mut() {
                        state.window().request_redraw();
                        state.update();
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
                _ => {}
            }
    }
}
