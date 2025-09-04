use wgpu::util::DeviceExt;
// Import crates
use winit::{
    application::ApplicationHandler, dpi::PhysicalSize, event::*, event_loop::{ActiveEventLoop, EventLoop}, keyboard::{KeyCode, PhysicalKey}, window::{Window, WindowAttributes}
};
use std::{sync::Arc};
// pollster lets us block on async setup code (wgpu uses async APIs)
use pollster::{FutureExt};

// Create a struct to hold the vertices of a triangle
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 2], // x, y coordinates
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position attribute
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0, // matches shader @location(0)
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex {position: [0.0, 0.5]}, // top
    Vertex {position: [-0.5, -0.5]}, // bottom left
    Vertex {position: [0.5, -0.5]}, // bottom right
];

// We'll create a struct to manage our GPU state
pub struct State {
    surface: wgpu::Surface<'static>, // The surface (connection between window & GPU)
    device: wgpu::Device, // Logical device (our handle to the GPU)
    queue: wgpu::Queue, // Command queue to submit work to the GPU
    config: wgpu::SurfaceConfiguration, // How the surface is configured (size, format, etc.)
    size: winit::dpi::PhysicalSize<u32>,
    is_surface_configured: bool,
    window: Arc<Window>,
    vertex_buffer: wgpu::Buffer,
    render_pipeline: wgpu::RenderPipeline,
}

impl State {
    // Async setup because GPU initialization may take time
    async fn new(window: Window) -> Self {
        // Get window size
        let size = window.inner_size();
        let window = Arc::new(window);

        // 1. Create GPU instance (entry point to wgpu)
        let instance = wgpu::Instance::default();

        // 2. Choose an surface (binds GPU rendering to our window)
        let surface = instance.create_surface(window.clone()).unwrap();

        // 3. Choose an adapter (represents a physical GPU)
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find an appropriate adapter");

        // 4. Request device and queue (logical GPU + command queue)
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: wgpu::MemoryHints::default(),
                },
                None, // Trace path (for debugging)
            )
            .await
            .unwrap();

        // 5. Get the surface's preferred format (like RGBA8Unorm)
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats[0];

        // 6. Configure the surface with width, height, format, and presentation mode
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        // 7. Load shaders from file
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // 8. Define pipeline layout (no uniforms yet, so empty)
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        // 9. Create render pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",  // vertex shader function
                buffers: &[Vertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main", // fragment shader function
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // 10. Create vertex buffer from vertices
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            is_surface_configured: false,
            window: window,
            vertex_buffer,
            render_pipeline
        }
    }

    // Called when window resizes
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    // This is where we'll handle keyboard events
    fn _handle_key(&self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            _ => {}
        }
    }

    fn window(&self) -> &Window {
        self.window.as_ref()
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {
        // remove `todo!()`
    }

    // Render a single frame (clear screen to a color)
    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // 1. Acquire next frame from surface
        let output = self.surface.get_current_texture()?;

        // 2. Create a view into the frame (like a convas we draw on)
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // 3. Create command encoder (records GPU commands)
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {label: Some("Render Encoder")});

        {
            // 4. Begin render pass (define clear color + attachments)
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        // This clears the screen every frame
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..VERTICES.len() as u32, 0..1);
            // Render pass dropped here, finishing recording
        }

        // 5. Submit recording command to GPU queue
        self.queue.submit(std::iter::once(encoder.finish()));

        // 6. Present frame to screen
        output.present();

        Ok(())
    }
}

struct App {
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

fn main() {
    {
        env_logger::init();
    }
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}