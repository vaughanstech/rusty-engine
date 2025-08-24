use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::EventLoopBuilder,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};
mod renderer_backend;
use renderer_backend::pipeline_builder::PipelineBuilder;

// <'a> in this case demonstrates that the struct will exist for some amount of time
struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device, // physical or Logical implementation of the Graphics card
    queue: wgpu::Queue, // work that needs tp be executed by the GPU
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>, // size of the window
    window: &'a Window,
    render_pipeline: wgpu::RenderPipeline,
}

// Start a constructor
impl<'a> State<'a> {
    
    async fn new(window: &'a Window) -> Self {
        // Query the size of the window
        let size = window.inner_size();

        // Create an instance -> will be used for various constructors in this function
        // 1. Descriptor -> Holds the specification of the thing we want to create -> Later used to create the thing
        let instance_descriptor = wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(), ..Default::default()
        };
        // 2. Use the Descriptor to create an instance
        let instance = wgpu::Instance::new(&instance_descriptor);
        let surface = instance.create_surface(window).unwrap();
        // 3. Adapter -> Used to query the capabilities of the system
        let adapter_descriptor = wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface), // The Some() here is used to create an optional value
            force_fallback_adapter: false,
        };
        let adapter = instance.request_adapter(&adapter_descriptor).await.unwrap();
        // 4. Create the device -> the logical device that will send instructions to the GPU
        let device_descriptor = wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            label: Some("Device"),
            memory_hints: wgpu::MemoryHints::default(),
            trace: wgpu::Trace::Off,
        };
        let (device, queue) = adapter.request_device(&device_descriptor).await.unwrap();
        // 5. Create the surface configuration -> a combination of properties of how the surface should look
        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .filter(| f | f.is_srgb())
            .next()
            .unwrap_or(surface_capabilities.formats[0]);
        let present_mode = if surface_capabilities.present_modes.contains(&wgpu::PresentMode::Fifo) {
            wgpu::PresentMode::Fifo
        } else {
            surface_capabilities.present_modes[0]
        };
        let alpha_mode = surface_capabilities.alpha_modes.iter().copied().find(|m| *m == wgpu::CompositeAlphaMode::Auto || *m == wgpu::CompositeAlphaMode::PreMultiplied).unwrap_or(surface_capabilities.alpha_modes[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode,
            alpha_mode,
            view_formats: vec![],
            desired_maximum_frame_latency: 2
        };
        surface.configure(&device, &config);

        let mut pipeline_builder = PipelineBuilder::new();
        pipeline_builder.set_shader_module("shaders/shader.wgsl", "vs_main", "fs_main");
        pipeline_builder.set_pixel_format(config.format);
        let render_pipeline = pipeline_builder.build_pipeline(&device);

        // 6. Create a return window
        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
        }
    }

    // Resize function -> To enable resizing of window
    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    // Render function -> To render the window and any contents inside
    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // 1. Grab an image for the window to draw to
        // --> Image: The raw pixels displayed to the screen
        // --> Image View: Used to interface with the Image
        let drawable = self.surface.get_current_texture()?;
        let image_view_descriptor = wgpu::TextureViewDescriptor::default();
        let image_view = drawable.texture.create_view(&image_view_descriptor);

        // 2. Create command encoder
        let command_encoder_descriptor = wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder")
        };
        let mut command_encoder = self.device.create_command_encoder(&command_encoder_descriptor);

        // 3. Create color that should flash on the blank window screen
        let color_attachment =  wgpu::RenderPassColorAttachment {
            view: &image_view,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.75,
                    g: 0.5,
                    b: 0.25,
                    a: 1.0
                }),
                store: wgpu::StoreOp::Store,
            }
        };

        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        };

        {
            let mut render_pass = command_encoder.begin_render_pass(&render_pass_descriptor);
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3, 0..1);
        }

        // 4. Begin render
        self.queue.submit(std::iter::once(command_encoder.finish()));

        drawable.present();

        Ok(())
    }
}

// Creating event enum
#[derive(Debug, Clone, Copy)]
enum CustomEvent {
    Timer,
}

async fn run() {
    // Setup the environment logger
    env_logger::init();

    // Construct event loop -> runs the main window
    let event_loop = EventLoopBuilder::<CustomEvent>::with_user_event()
        .build()
        .unwrap();

    // Make a window -> make sure it has a reference to the event loop
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // Spawn a separate thread which runs a sensor of signals that need to be passed to the event loop
    let event_loop_proxy = event_loop.create_proxy();

    // Infinite loop that runs in the background -> Waits 17ms and sends a signal to the EventLoop
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
        event_loop_proxy.send_event(CustomEvent::Timer).ok();
    });

    // Make the wgpu state
    let mut state= State::new(&window).await;

    // Kick off the event loop for the window -> Where we match inputs to their corresponding command
    event_loop.run(move | event, elwt | match event {
        Event::UserEvent(..) => {
            state.window.request_redraw();
        },

        // Deal with any other window event
        Event::WindowEvent { window_id, ref event } if window_id == state.window.id() => match event {
            // Handle Window Resizing
            WindowEvent::Resized(physical_size) => state.resize(*physical_size),

            // Requesting the window be closed by the user
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                 event:
                  KeyEvent {
                     physical_key: PhysicalKey::Code(KeyCode::Escape),
                     state: ElementState::Pressed, repeat: false, .. }, .. } => {
                println!("Goodbye!");
                elwt.exit();
            }

            // Handle window redraw event
            WindowEvent::RedrawRequested => match state.render() {
                Ok(_) => {},
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                Err(e) => eprintln!("{:?}", e),
            }
            _ => {},
        },

        _ => {},
    }).expect("Error!");
}

fn main() {
    // Block the main thread until the asynchronous main function finishes executing
    pollster::block_on(run());
}