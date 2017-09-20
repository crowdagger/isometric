extern crate isometric;
extern crate glium;

use isometric::Level;
use isometric::Renderer;
use isometric::Camera;

use glium::glutin::{Event, WindowEvent, DeviceEvent};

fn main() {
    let mut level: Level = Level::new(20, 20, 0.0);

    level.set_z(0, 0, 0.1);
    level.set_z(0, 1, 0.2);
    level.set_z(1, 0, 0.15);
    level.set_z(1, 1, 0.2);
    level.set_z(2, 1, 0.0);
    level.set_z(3, 1, 0.2);
    level.set_z(4, 1, 0.1);
    level.set_z(3, 2, 0.1);
    level.set_z(2, 2, 0.0);
    level.set_z(2, 3, 0.1);
    level.set_z(4, 4, 1.0);
    level.set_z(5, 4, 2.0);
    level.set_z(6, 4, 3.0);
    level.set_z(6, 5, 4.0);
    level.set_z(6, 6, 5.0);
   level.set_z(5, 4, 1.0);
   level.add_border_walls();
   level.add_cliff_walls(0.5);


    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title("Level");
    let context = glium::glutin::ContextBuilder::new()
        .with_depth_buffer(24);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut camera = Camera::new(&display);
    let mut renderer = Renderer::new(level, &display);

    let mut closed = false;
    let mut t = -5.0;
    
    while !closed {
        events_loop.poll_events(|ev| {
            match ev {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Closed => closed = true,
                    _ => (),
                },
                Event::DeviceEvent { event, .. } => match event {
                    DeviceEvent::Key(glium::glutin::KeyboardInput { scancode, .. }) => {
                        match scancode {
                            111 => { // top
                                let mut pos = camera.pos();
                                pos[1] += 1.0;
                                camera.set_pos(pos[0], pos[1], pos[2]);
                            },
                            113 => { //left
                                let mut pos = camera.pos();
                                pos[0] -= 1.0;
                                camera.set_pos(pos[0], pos[1], pos[2]);
                            },
                            114 => { //right
                                let mut pos = camera.pos();
                                pos[0] += 1.0;
                                camera.set_pos(pos[0], pos[1], pos[2]);
                            },
                            116 => { // down
                                let mut pos = camera.pos();
                                pos[1] -= 1.0;
                                camera.set_pos(pos[0], pos[1], pos[2]);
                            },
                            _ => println!("{}", scancode),

                        }
                    },
                    _ => (),
                },
                _ => (),
            }
        });
        renderer.render(&display, &camera);
    }
}
