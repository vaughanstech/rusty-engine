/*
Purpose: Manages global GPU + winfow state
Responsibilities:
    - Initilize WGPU (device, queue, surface, pipeline)
    - Store the RenderPipeline, SurfaceConfiguration, and window size
    - Own a collection of Renderable objects
    - Handle resizing, updating transforms, and rendering per frame
    - ex: engine room
*/

use crate::{
    camera::{Camera}, renderable::Renderable, shapes::{create_circle, SQUARE_INDICES, SQUARE_VERTICES, TRIANGLE_VERTICES}, uniforms::Uniforms, vertex::Vertex
};
use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode};
use winit::event::WindowEvent;
use winit::window::Window;

// We'll create a struct to manage our GPU state
pub struct State {
    surface: wgpu::Surface<'static>, // The surface (connection between window & GPU)
    device: wgpu::Device, // Logical device (our handle to the GPU)
    queue: wgpu::Queue, // Command queue to submit work to the GPU
    config: wgpu::SurfaceConfiguration, pub(crate) // How the surface is configured (size, format, etc.)
    size: winit::dpi::PhysicalSize<u32>,
    is_surface_configured: bool,
    window: Arc<Window>,
    render_pipeline: wgpu::RenderPipeline,
    uniform_bind_group: wgpu::BindGroup,
    camera: Camera,
    renderables: Vec<Renderable>,
    start_time: std::time::Instant,
}

impl State {
    // Async setup because GPU initialization may take time
    pub async fn new(window: Window) -> Self {
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
            source: wgpu::ShaderSource::Wgsl(include_str!(r"shaders\shader.wgsl").into()),
        });

        // 8. Create uniform buffer and bind group
        let uniforms = Uniforms::new();
        // Creates block of GPU memory that holds your transformation matrix
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Uniform Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform Bind Group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // 9. Define pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        // 10. Create render pipeline
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

        let (circle_vertices, circle_indices) = create_circle(0.4, 64, [0.0, 0.0, 1.0]);

        let start_time = std::time::Instant::now();

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Uniform Bind Group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None,
            }],
        });

        let camera = Camera {
            eye: (0.0, 0.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: glam::Vec3::Y,
            aspect: config.width as f32 / config.height as f32,
            fov_y: 45.0f32.to_radians(),
            z_near: 0.1,
            z_far: 100.0,
        };

        // Create triangle, square, circle
        let triangle = Renderable::new(
            &device,
            &uniform_bind_group_layout,
            &TRIANGLE_VERTICES,
            None,
            glam::Mat4::from_translation(glam::vec3(0.0, 0.5, 0.0)),
        );
        let square = Renderable::new(
            &device,
            &uniform_bind_group_layout,
            &SQUARE_VERTICES,
            Some(&SQUARE_INDICES),
            glam::Mat4::from_translation(glam::vec3(-0.5, -0.5, 0.0)),
        );
        let circle = Renderable::new(
            &device,
            &uniform_bind_group_layout,
            &circle_vertices,
            Some(&circle_indices),
            glam::Mat4::from_translation(glam::vec3(0.5, -0.5, 0.0)),
        );
        let renderables = vec![triangle, square, circle];

        Self {
            surface,
            device,
            queue,
            config,
            size,
            is_surface_configured: false,
            window: window,
            render_pipeline,
            uniform_bind_group,
            camera,
            renderables,
            start_time
        }
    }

    // Called when window resizes
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
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

    pub fn window(&self) -> &Window {
        self.window.as_ref()
    }

    #[allow(unused_variables)]
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let view_proj = self.camera.build_view_projection_matrix();
        
        for r in &mut self.renderables {
            r.update(&self.queue, elapsed, view_proj);
        }
    }

    // Render a single frame (clear screen to a color)
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // 1. Acquire next frame from surface
        // Refine error handling
        match self.surface.get_current_texture() {
            Ok(output) => {
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
                    render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
                    
                    for r in &self.renderables {
                        r.draw(&mut render_pass);
                    }
                    // Render pass dropped here, finishing recording
                }

                // 5. Submit recording command to GPU queue
                self.queue.submit(std::iter::once(encoder.finish()));

                // 6. Present frame to screen
                output.present();

                Ok(())
            }
            Err(wgpu::SurfaceError::Lost) => {
                // Reconfigure with the current state
                self.resize(self.size);
                Ok(())
            }
            Err(wgpu::SurfaceError::OutOfMemory) => {
                // Fatal: exit program
                Err(wgpu::SurfaceError::OutOfMemory)
            }
            Err(e) => {
                eprintln!("Render error: {:?}", e);
                Ok(())
            }
        }
    }
}
