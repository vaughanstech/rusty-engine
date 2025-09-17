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
    camera::{Camera, CameraUniform, Projection}, controller::Controller, light::{Light, Lights}, renderable::{self, Material, Renderable}, shapes::{self, create_cube, create_pyramid, create_sphere }, texture::{self, Texture}, uniforms::Uniforms, vertex::Vertex
};
use std::sync::Arc;
use glam::vec3;
use wgpu::util::DeviceExt;
use winit::{event_loop::ActiveEventLoop, keyboard::{KeyCode}};
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
    diffuse_bind_group: wgpu::BindGroup,
    camera: Camera,
    pub controller: Controller,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    lights: Lights,
    light_buffer: wgpu::Buffer,
    light_bind_group: wgpu::BindGroup,
    light_bind_group_layout: wgpu::BindGroupLayout,
    last_frame: std::time::Instant,
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
        let uniform_material_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Uniform Bind Group Layout"),
            entries: &[
                // binding 0: model uniform (mat4)
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // binding 1: material uniform (use_texture)
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
        });
        // let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        //     label: Some("Uniform Bind Group"),
        //     layout: &uniform_material_bind_group_layout,
        //     entries: &[wgpu::BindGroupEntry {
        //         binding: 0,
        //         resource: uniform_buffer.as_entire_binding(),
        //     }],
        // });
        // 9. Setup Camera uniform buffer and bind group
        let camera = Camera::new(
            config.width as f32 / config.height as f32
        );
        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
            label: Some("Camera Bind Group Layout"),
        });
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("Camera Bind Group"),
        });
        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Text Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let diffuse_bytes = include_bytes!("happy_tree.png"); // example image
        let diffuse_texture = Texture::from_bytes(&device, &queue, diffuse_bytes, "happy_tree");
        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("Diffuse Bind Group"),
        });

        // Defining and binding a light source
        let lights = Lights {
            lights: [Light {
                position: [2.0, 2.0, 2.0],
                intensity: 0.0,
                color: [1.0, 1.0, 1.0],
                _padding: 0.0,
            }; 16], // initializes array with same light
            num_lights: 5,
            _padding: [0; 3],
        };
        // Create GPU buffer for the light
        let light_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Light Buffer"),
            contents: bytemuck::bytes_of(&lights),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        // Bind group layout for lights
        let light_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Light Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let light_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Light Bind Group"),
            layout: &light_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: light_buffer.as_entire_binding(),
            }]
        });
        let controller = Controller::new(2.0, 0.5, 1.0);

        // 10. Define pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout, &uniform_material_bind_group_layout, &texture_bind_group_layout, &light_bind_group_layout],
            push_constant_ranges: &[],
        });

        // 11. Create render pipeline
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

        // let (circle_vertices, circle_indices) = create_circle(0.4, 64, [0.0, 0.0, 1.0], [0.5, 0.5]);

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

        let (_white_tex, white_bind_group) = texture::create_white_texture(
            &device,
            &queue,
            &texture_bind_group_layout,
        );
        let (pyramid_vertices, pyramid_indices) = create_pyramid();
        // Create triangle, square, circle
        let pyramid = Renderable::new(
            &device,
            &queue,
            &render_pipeline,
            &uniform_material_bind_group_layout,
            &pyramid_vertices,
            &pyramid_indices,
            None,
            false,
            true,
            false,
            0.0,
            [1.0, 1.0, 1.0],
            glam::vec3(-2.5, 0.0, 0.0), // position in world
            glam::vec3(0.0, 0.0, 0.0), // spin around Z
            glam::vec3(2.0, 2.0, 2.0), // scale
        );

        let (texture, tex_bind_group) = texture::load_texture(
            &device,
            &queue,
            &texture_bind_group_layout,
            r"src\happy_tree.png",
        ).expect("Failed to load texture");
        let (grey_tex, grey_bind_group) = texture::create_grey_texture(
            &device,
            &queue,
            &texture_bind_group_layout,
        );
        let (cube_vertices, cube_indices) = create_cube();
        let cube = Renderable::new(
            &device,
            &queue,
            &render_pipeline,
            &uniform_material_bind_group_layout,
            &cube_vertices,
            &cube_indices,
            Some(grey_bind_group),
            true,
            true,
            false,
            0.0,
            [1.0, 1.0, 1.0],
            glam::vec3(2.5, 0.0, 0.0), // position
            glam::vec3(0.0, 0.0, 0.0), // spin around Y
            glam::vec3(1.0, 1.0, 1.0), // scale
        );

        // let (circ_tex, circ_tex_bind_group) = texture::create_white_texture(
        //     &device,
        //     &queue,
        //     &texture_bind_group_layout,
        // );
        // let circle = Renderable::new(
        //     &device,
        //     &queue,
        //     &render_pipeline,
        //     &uniform_bind_group_layout,
        //     &circle_vertices,
        //     Some(&circle_indices),
        //     Material::Textured { bind_group: circ_tex_bind_group },
        //     glam::vec3(0.0, -1.0, 0.0), // position
        //     glam::vec3(1.0, 0.0, 0.0), // spin around X
        //     glam::vec3(1.0, 1.0, 1.0), // scale
        // );

        let (_white_tex, white_bind_group) = texture::create_white_texture(
            &device,
            &queue,
            &texture_bind_group_layout,
        );
        let (_grey_tex, grey_bind_group) = texture::create_grey_texture(
            &device,
            &queue,
            &texture_bind_group_layout,
        );
        let (sphere_vertices, sphere_indices) = shapes::create_sphere(1.0, 32, 16);
        let sphere = Renderable::new(
            &device,
            &queue,
            &render_pipeline,
            &uniform_material_bind_group_layout,
            &sphere_vertices,
            &sphere_indices,
            Some(grey_bind_group),
            true,
            true,
            true,
            10.0,
            [1.0, 1.0, 1.0],
            glam::vec3(0.0, 0.0, 0.0),
            glam::vec3(0.0, 0.0, 0.0),
            glam::vec3(1.0, 1.0, 1.0),
        );
        let renderables = vec![pyramid, cube, sphere];

        Self {
            surface,
            device,
            queue,
            config,
            size,
            is_surface_configured: false,
            window: window,
            render_pipeline,
            diffuse_bind_group,
            camera,
            camera_bind_group,
            camera_buffer,
            camera_uniform,
            lights,
            light_buffer,
            light_bind_group_layout,
            light_bind_group,
            controller,
            last_frame: std::time::Instant::now(),
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
            self.camera.aspect = new_size.width as f32 / new_size.height as f32
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

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        self.controller.process_events(event)
    }

    pub fn update(&mut self) {
        let now = std::time::Instant::now();
        let dt = now.duration_since(self.last_frame).as_secs_f32();
        self.last_frame = now;
        let view_proj = self.camera.build_view_projection_matrix().into();

        self.controller.update_camera(&mut self.camera, dt);

        let time = self.start_time.elapsed().as_secs_f32();

        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));
        
        for r in &mut self.renderables {
            r.update(&self.queue, time, view_proj);
        }
    }

    pub fn update_lights(&mut self, time: f32) {
        let mut active_lights = Vec::new();

        // Global light
        active_lights.push(Light {
            position: [10.0, 10.0, 10.0],
            color: [1.0, 1.0, 1.0],
            intensity: 0.0,
            _padding: 0.0,
        });

        // Add emissive objects
        for renderable in &self.renderables {
            if renderable.start_emission {
                let model = renderable.model_matrix(time);
                let world_pos = model.transform_point3(glam::Vec3::ZERO);
                active_lights.push(Light {
                    position: [world_pos.x, world_pos.y, world_pos.z],
                    color: renderable.color,
                    intensity: renderable.emissive_strength,
                    _padding: 0.0,
                });
            }
        }

        // CAP to MAX_LIGHTS
        let light_count = active_lights.len().min(16);
        self.lights.num_lights = light_count as u32;
        for (i, l) in active_lights.iter().take(16).enumerate() {
            self.lights.lights[i] = *l;
        }

        // Push to GPU
        self.queue.write_buffer(&self.light_buffer, 0, bytemuck::cast_slice(&[self.lights]));
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

                    // Bind camera once (group 1)
                    render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
                    render_pass.set_bind_group(2, &self.diffuse_bind_group, &[]);
                    render_pass.set_bind_group(3, &self.light_bind_group, &[]);
                    
                    // Draw each renderable
                    for renderable in &self.renderables {
                        // Per-object uniform (MVP matrix)
                        render_pass.set_bind_group(1, &renderable.uniform_material_bind_group, &[]);

                        // Optional texture
                        if let Some(texture_bg) = &renderable.texture_bind_group {
                            render_pass.set_bind_group(2, texture_bg, &[]);
                        }

                        // Material (always)
                        //render_pass.set_bind_group(1, &renderable.uniform_material_bind_group, &[]);

                        // Buffers
                        render_pass.set_vertex_buffer(0, renderable.vertex_buffer.slice(..));
                        
                        render_pass.set_index_buffer(renderable.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

                        // Draw
                        render_pass.draw_indexed(0..renderable.num_indices, 0, 0..1);
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
