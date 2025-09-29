/*
Purpose: Entry point of the app
Responsibilities:
    - Create the event loop
    - Create and own your App (which manages State)
    - Forward window + input events into your engine
    - Stay as small as possible (ex: traffic controller)
*/

mod app;
mod camera;
mod instance;
mod model;
mod resources;
mod state;
mod texture;
mod vertex;
mod uniforms;
mod shapes;

use app::App;
use winit::event_loop::EventLoop;

fn main() {
    {
        env_logger::init();
    }
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}