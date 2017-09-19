extern crate isometric;
extern crate glium;

use isometric::Level;
use isometric::Renderer;

use glium::glutin::{Event, WindowEvent};

fn main() {
    let mut level: Level = Level::new(100, 100, 0.0);
    level.set_z(4, 4, 1.0);
    level.set_z(5, 4, 2.0);
    level.set_z(6, 4, 3.0);
    level.set_z(6, 5, 4.0);
    level.set_z(6, 6, 5.0);
//    level.set_z(5, 4, 1.0);
    level.add_border_walls();
    level.add_cliff_walls(0.1);
    let mut renderer = Renderer::new(level);

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title("Level");
    let context = glium::glutin::ContextBuilder::new()
        .with_depth_buffer(24);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut closed = false;
    let mut t = -5.0;
    
    while !closed {
        events_loop.poll_events(|ev| {
            match ev {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Closed => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });
        renderer.render(&display, t);
        t += 0.02;
    }
}
