# Documentation for main.rs

This program:

- Creates an OS window using `winit`.
- Initializes a GPU context using `wgpu` (Instance -> Surface -> Adapter -> Device + Queue).
- Configures the window's swapchain (`SurfaceConfiguration`) to describe how frames are presented.
- Builds a `RenderPipeline` (via a custom `PipelineBuilder`) that uses a WGSL shader.
- Enters an event loop; on a timer, it requests redraws. On redraw, it clears the screen and issues a tiny draw cell.
- Handles resizes and common window events (close, ESC key, etc.).

Key Vocabulary:

- **Instance**: entry-point handle to the graphics API (Vulkan/Metal/DX12/WebGPU).
- **Surface**: The drawable target tied to a window.
- **Adapter**: A physical/virtual GPU choice + capabilities.
- **Device**: A logical connection to the adapter; creates GPU resources.
- **Queue**: Submits encoded GPU commands for execution.
- **Swapchain / SurfaceConfiguration**: How images are produced and presented to the screen.

## Imports

```Rust
use winit::{
    dpi::PhysicalSize,                  // Physical (pixel) size; distinct from logical size on HiDPI displays
    event::*,                           // Core event types (WindowEvent, DeviceEvent, etc.).
    event_loop::EventLoopBuilder,       // Builder to create and configure an event loop
    keyboard::{KeyCode, PhysicalKey},   // Keyboard input enums
    window::{Window, WindowBuilder},    // Window types and builder for OS integration
};
```

Local modules where we define helpers for building pipelines

- `PiplineBuilder` abstracts the verbose setup of a `wgpu::RenderPipeline`

```Rust
mod renderer_backend;
use renderer_backend::pipeline_builder::PipelineBuilder;
```

## State Class

Holds all GPU + windowing state needed to render each frame

### sruct State

Lifetime parameter `'a` ensures `surface: wgpu::Surface<'a>` cannot outlive the window it was created from. `wgpu::Surface` internally references the platform window; tying lifetimes prevents accidental use-after-free.

```Rust
struct State<'a> {
    surface: wgpu::Surface<'a>,             // The drawable target associated with the OS window
    device: wgpu::Device,                   // Logical device: creates resources and encoders
    queue: wgpu::Queue,                     // Submission queue: executes GPU work produces by encoders
    config: wgpu::SurfaceConfiguration,     // Swapchain configuration (format, size, present mode, etc.)
    size: PhysicalSize<u32>,                // Current window size in physical pixels
    window: &'a Window,                     // Borrowed pointer to the winit window; used to request redraws
    render_pipeline: wgpu::RenderPipeline,  // The pipeline state object used for draw calls.
}
```

### impl<'a> State<'a>

Asynchronously constructs a new `State` from a `winit` window

#### Constructor async fn new*()

`wgpu` uses async because selecting an adapter and requesting a device may involve the OS/driver and potentially async resource compilation

```Rust
async fn new(window: &'a Window) -> Self {
    // Query the current window size (in physical pixels).
    let size = window.inner_size();

    // 1) Create a `wgpu::Instance`, the root object used to query adapters & create surfaces.
    //    `backends: wgpu::Backends::all()` asks wgpu to try all supported native backends
    //    (Vulkan, Metal, DX12, GL) depending on the platform.
    let instance_descriptor = wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    };
    let instance = wgpu::Instance::new(&instance_descriptor);

    // Create a `Surface` bound to the OS window. `unwrap` here will panic if the surface
    // cannot be created (e.g., very old drivers or headless environment without a compatible backend).
    let surface = instance.create_surface(window).unwrap();

    // 2) Select an `Adapter` (a specific GPU + capability set). We prefer HighPerformance
    //    and require it to be compatible with our `surface` so we can present frames to it.
    let adapter_descriptor = wgpu::RequestAdapterOptionsBase {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false, // Set true to allow WARP/SwiftShader-style software adapters when no hardware fits.
    };
    let adapter = instance.request_adapter(&adapter_descriptor).await.unwrap();

    // 3) Request a logical `Device` and associated `Queue` from the adapter.
    //    Features/limits can be customized depending on the app’s needs.
    let device_descriptor = wgpu::DeviceDescriptor {
        required_features: wgpu::Features::empty(), // Ask for no optional features; widest compatibility.
        required_limits: wgpu::Limits::default(),   // Default limits are reasonable for most simple demos.
        label: Some("Device"),
        memory_hints: wgpu::MemoryHints::default(),
        trace: wgpu::Trace::Off,                    // Enable to dump a trace for GPU debugging/profiling.
    };
    let (device, queue) = adapter.request_device(&device_descriptor).await.unwrap();

    // 4) Describe how images will be produced and presented to the window (swapchain config).
    let surface_capabilities = surface.get_capabilities(&adapter);

    // Choose an sRGB format when available for correct color-space handling.
    let surface_format = surface_capabilities
        .formats
        .iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(surface_capabilities.formats[0]);

    // Choose a present mode. `Fifo` is the portable V-sync-like mode and always supported on WebGPU.
    let present_mode = if surface_capabilities
        .present_modes
        .contains(&wgpu::PresentMode::Fifo)
    {
        wgpu::PresentMode::Fifo
    } else {
        // Fall back to the first available mode (platform/driver dependent).
        surface_capabilities.present_modes[0]
    };

    // Select an alpha mode. `Auto`/`PreMultiplied` are typical; others include `Opaque`.
    let alpha_mode = surface_capabilities
        .alpha_modes
        .iter()
        .copied()
        .find(|m| *m == wgpu::CompositeAlphaMode::Auto || *m == wgpu::CompositeAlphaMode::PreMultiplied)
        .unwrap_or(surface_capabilities.alpha_modes[0]);

    // Build the final surface configuration using the size and chosen format/modes.
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT, // We’ll render into the surface textures.
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode,
        alpha_mode,
        view_formats: vec![],
        desired_maximum_frame_latency: 2, // Hint: how many frames to queue ahead; backend-dependent.
    };

    // Apply the configuration to the surface (creates/updates the swapchain under the hood).
    surface.configure(&device, &config);

    // 5) Build a render pipeline using your helper `PipelineBuilder`.
    //    This likely compiles the WGSL shader, sets entry points, defines color targets, etc.
    let mut pipeline_builder = PipelineBuilder::new();
    pipeline_builder.set_shader_module("shaders/shader.wgsl", "vs_main", "fs_main");
    pipeline_builder.set_pixel_format(config.format); // Ensure pipeline color target matches the surface format.
    let render_pipeline = pipeline_builder.build_pipeline(&device);

    // Return the fully-initialized state bundle.
    Self {
        window,
        surface,
        device,
        queue,
        config,
        size,
        render_pipeline,
    }
};
```

#### fn resize()

Handle window resizes by updating our `SurfaceConfiguration`. `wgpu` requires reconfiguring the surface whenever size changes

```Rust
fn resize(&mut self, new_size: PhysicalSize<u32>) {
    // Ignore zero-sized resizes (e.g., when a window is minimized); many backends
    // cannot render to a 0×0 surface.
    if new_size.width > 0 && new_size.height > 0 {
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }
};
```

#### fn render()

Render a single frame. Returns `Ok(())` on success or a `SurfaceError` you should react to (Lost -> reconfigure, OutOfMemory -> exit, etc.)

```Rust
fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
    // 1) Acquire the next surface texture to render into.
    //    Failure here is common on resize/device-lost; we bubble the error up to the caller.
    let drawable = self.surface.get_current_texture()?;

    // Create a `TextureView` so we can attach the texture to a render pass.
    let image_view_descriptor = wgpu::TextureViewDescriptor::default();
    let image_view = drawable.texture.create_view(&image_view_descriptor);

    // 2) Create a command encoder — a builder for GPU command buffers.
    let command_encoder_descriptor = wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    };
    let mut command_encoder = self
        .device
        .create_command_encoder(&command_encoder_descriptor);

    // 3) Define the color attachment with a clear operation. This clears the frame
    //    to a pleasant orange-ish color before we issue draw calls.
    let color_attachment = wgpu::RenderPassColorAttachment {
        view: &image_view,
        depth_slice: None,          // For array textures; not used here.
        resolve_target: None,       // For MSAA resolve; not used here.
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color {
                r: 0.75,
                g: 0.5,
                b: 0.25,
                a: 1.0,
            }),
            store: wgpu::StoreOp::Store, // Keep the rendered result for presentation.
        },
    };

    // Describe the render pass. We have a single color target and no depth/stencil.
    let render_pass_descriptor = wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(color_attachment)],
        depth_stencil_attachment: None,
        occlusion_query_set: None,
        timestamp_writes: None,
    };

    // Begin encoding the render pass scope. Dropping `render_pass` ends the pass.
    {
        let mut render_pass = command_encoder.begin_render_pass(&render_pass_descriptor);
        render_pass.set_pipeline(&self.render_pipeline); // Bind our previously created pipeline.
        render_pass.draw(0..3, 0..1);                   // Issue a tiny draw: 3 vertices, 1 instance.
    }

    // 4) Finish encoding, submit to the GPU queue, and present the frame to the screen.
    self.queue
        .submit(std::iter::once(command_encoder.finish()));
    drawable.present();

    Ok(())
};
```

## Custom Event

This is a custom type so we can wake the event loop from another thread

```rust
#[derive(Debug, Clone, Copy)]
enum CustomEvent {
    Timer,
}
```

## Run function

Top-level run function that wires everything together, then enters the event loop

```rust
async fn run() {
    // Initialize the env_logger so `log`/`tracing`-style logs show in stdout/stderr.
    env_logger::init();

    // Build an event loop capable of receiving our `CustomEvent` messages.
    let event_loop = EventLoopBuilder::<CustomEvent>::with_user_event()
        .build()
        .unwrap();

    // Create a window attached to the above event loop. On some platforms, window creation
    // may fail (e.g., display server issues), hence the `unwrap`.
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // Create a proxy that allows other threads to send `CustomEvent`s into this `event_loop`.
    let event_loop_proxy = event_loop.create_proxy();

    // Spawn a background thread that periodically sends a `Timer` event.
    // NOTE: The code sleeps for 100 ms (10 Hz), not 17 ms (~60 Hz).
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
        let _ = event_loop_proxy.send_event(CustomEvent::Timer);
    });

    // Initialize all GPU state.
    let mut state = State::new(&window).await;

    // Enter the event loop. The closure receives events and the `elwt` control handle.
    event_loop
        .run(move |event, elwt| match event {
            // Our background timer merely triggers redraws.
            Event::UserEvent(..) => {
                state.window.request_redraw();
            }

            // Filter events for our window, then handle them.
            Event::WindowEvent { window_id, ref event } if window_id == state.window.id() => match event {
                // When the OS resizes the window, update surface configuration.
                WindowEvent::Resized(physical_size) => state.resize(*physical_size),

                // Close the app when the user clicks the close box or presses ESC without repeat.
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            state: ElementState::Pressed,
                            repeat: false,
                            ..
                        },
                    ..
                } => {
                    println!("Goodbye!");
                    elwt.exit(); // Ask the event loop to exit cleanly.
                }

                // Redraw when requested by OS or via `request_redraw`.
                WindowEvent::RedrawRequested => match state.render() {
                    Ok(_) => {}
                    // The surface was lost (e.g., after a mode change); reconfigure with current size.
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // Out of memory: best we can do is exit.
                    Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                    // Other errors (Timeout, Outdated) are transient or non-fatal; log them.
                    Err(e) => eprintln!("{:?}", e),
                },

                // Ignore all other window events.
                _ => {}
            },

            // Ignore all other event kinds (e.g., device events) in this simple example.
            _ => {}
        })
        .expect("Error!");
}
```

## Main thread

We will block the main thread until the async `run()` future completes

- `pollster` is a tiny crate useful for bridging sync `main` with async code

```rust
fn main() {
    pollster::block_on(run());
}
```
