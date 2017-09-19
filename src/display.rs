// (C) 2017, Élisabeth Henry
//
// Licensed under either of
// 
// Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
// MIT license: http://opensource.org/licenses/MIT
// at your option.
//
// Unless you explicitly state otherwise, any contribution intentionally submitted
// for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
// dual licensed as above, without any additional terms or conditions.

use level::Level;

use glium;
use glium::Frame;
use glium::Surface;
use glium::Display;

use std::default::Default;

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
    normal: [f32; 3],
}

implement_vertex!(Vertex, position, tex_coords, normal);

/// Contains a level and add methods to render it
pub struct Renderer<FT=(), WT=()> {
    level: Level<FT, WT>,
}

impl<FT:Clone+Default,
     WT:Clone+Default> Renderer<FT, WT> {
    /// Creates a new renderer from an existing level and a glutin display
    pub fn new(level: Level<FT, WT>) -> Renderer<FT, WT> {
        Renderer {
            level: level
        }
    }

    /// Get access to the level
    pub fn level(&self) -> &Level<FT, WT> {
        &self.level
    }

    /// Get mutable access to the level
    pub fn level_mut(&mut self) -> &mut Level<FT, WT> {
        &mut self.level
    }

    fn add_horizontal_wall(&self, vertices: &mut Vec<Vertex>, x: usize, y: usize, z: f32, other_z: f32) {
        let x = x as f32;
        let y = y as f32;
        let a = [x, y, z];
        let b = [x + 1.0, y, z];
        let c = [x, y, other_z];
        let d = [x + 1.0, y, other_z];
        let normal = [0.0, -1.0, 0.0];
        let tex_coords = [0.0, 0.0];
        vertices.push(Vertex {
            position: a,
            tex_coords: tex_coords,
            normal: normal
        });
        vertices.push(Vertex {
            position: b,
            tex_coords: tex_coords,
            normal: normal
        });
        vertices.push(Vertex {
            position: c,
            tex_coords: tex_coords,
            normal: normal
        });
        vertices.push(Vertex {
            position: b,
            tex_coords: tex_coords,
            normal: normal
        });
        vertices.push(Vertex {
            position: d,
            tex_coords: tex_coords,
            normal: normal
        });
        vertices.push(Vertex {
            position: c,
            tex_coords: tex_coords,
            normal: normal
        });
    }
    
    /// Return the vertices corresponding to the walls' data
    fn get_vertices_walls(&self) -> Vec<Vertex> {
        let mut vertices = vec!();
        let level = self.level();
        let width = level.get_width();
        let depth = level.get_depth();

        

        fn add_vertical_wall(vertices: &mut Vec<Vertex>, x: usize, y: usize, z: f32, other_z: f32) {
            let x = x as f32;
            let y = y as f32;
            let a = [x, y, z];
            let b = [x, y + 1.0, z];
            let c = [x, y, other_z];
            let d = [x, y + 1.0, other_z];
            let normal = [-1.0, 0.0, 0.0];
            let tex_coords = [0.0, 0.0];
            vertices.push(Vertex {
                position: a,
                tex_coords: tex_coords,
                normal: normal
            });
            vertices.push(Vertex {
                position: b,
                tex_coords: tex_coords,
                normal: normal
            });
            vertices.push(Vertex {
                position: c,
                tex_coords: tex_coords,
                normal: normal
            });
            vertices.push(Vertex {
                position: b,
                tex_coords: tex_coords,
                normal: normal
            });
            vertices.push(Vertex {
                position: d,
                tex_coords: tex_coords,
                normal: normal
            });
            vertices.push(Vertex {
                position: c,
                tex_coords: tex_coords,
                normal: normal
            });
        }
        
        for x in 0..width {
            for y in 0..depth {
                let wall = level.get_wall_ref(x, y);
                let z = level.get_z(x, y);
                if wall.bottom {
                    if y == 0 {
                        self.add_horizontal_wall(&mut vertices, x, y, z, z + 1.0);
                    } else {
                        self.add_horizontal_wall(&mut vertices, x, y, z, level.get_z(x, y - 1));
                    }
                }
                if wall.left {
                    if x == 0 {
                        add_vertical_wall(&mut vertices, x, y, z, z + 1.0);
                    } else {
                        add_vertical_wall(&mut vertices, x, y, z, level.get_z(x - 1, y));
                    }
                }
                if wall.top  {
                    if y == depth - 1 {
                        self.add_horizontal_wall(&mut vertices, x, y + 1, z, z + 1.0);
                    } else {
                        self.add_horizontal_wall(&mut vertices, x, y + 1, z, level.get_z(x, y + 1));
                    }
                }
                if wall.right {
                    if x == width - 1 {
                        add_vertical_wall(&mut vertices, x + 1, y, z, z + 1.0);
                    } else {
                        add_vertical_wall(&mut vertices, x + 1, y, z, level.get_z(x + 1, y));
                    }
                }
            }
        }
        vertices
    }
    
    /// Returns the vertices corresponding to the level's data
    fn get_vertices(&self) -> Vec<Vertex> {
        let mut vertices = vec!();
        let level = self.level();
        let width = level.get_width();
        let depth = level.get_depth();
        for x in 0..width {
            for y in 0..depth {
                let z = level.get_z(x, y);
                let mut sum_a = z;
                let mut div_a = 1.0;
                let mut sum_b = z;
                let mut div_b = 1.0;
                let mut sum_c = z;
                let mut div_c = 1.0;
                let mut sum_d = z;
                let mut div_d = 1.0;
                
                // Each vertex's height is averaged to all adjacent tiles that
                // a) exist b) have no wall between this tile and them
                if x > 0 && level.is_move_possible((x, y), (x - 1, y)) {
                    let z = level.get_z(x - 1, y);
                    sum_a += z;
                    div_a += 1.0;
                    sum_c += z;
                    div_c += 1.0;
                    if y > 0 && level.is_move_possible((x, y), (x - 1, y - 1)) {
                        let z = level.get_z(x -1, y - 1);
                        sum_a += z;
                        div_a += 1.0;
                    }
                }
                if y > 0 && level.is_move_possible((x, y), (x, y - 1)) {
                    let z = level.get_z(x, y - 1);
                    sum_a += z;
                    div_a += 1.0;
                    sum_b += z;
                    div_b += 1.0;
                    if x < width - 1 && level.is_move_possible((x, y), (x + 1, y - 1)) {
                        let z = level.get_z(x + 1, y - 1);
                        sum_b += z;
                        div_b += 1.0;
                    }
                }
                if x < width - 1 && level.is_move_possible((x, y), (x + 1, y)) {
                    let z = level.get_z(x + 1, y);
                    sum_b += z;
                    div_b += 1.0;
                    sum_d += z;
                    div_d += 1.0;
                    if y < depth - 1 && level.is_move_possible((x, y), (x + 1, y + 1)) {
                        let z = level.get_z(x + 1, y + 1);
                        sum_d += z;
                        div_d += 1.0;
                    }
                }
                if y < depth - 1 && level.is_move_possible((x, y), (x, y + 1)) {
                    let z = level.get_z(x, y + 1);
                    sum_c += z;
                    div_c += 1.0;
                    sum_d += z;
                    div_d += 1.0;
                    if x > 0 && level.is_move_possible((x, y), (x - 1, y + 1)) {
                        let z = level.get_z(x - 1, y + 1);
                        sum_c += z;
                        div_c += 1.0;
                    }
                }

                // Finally build the four vertices
                let a = [x as f32, y as f32, sum_a / div_a];
                let ta = [0.0, 0.0];
                let b = [(x + 1) as f32, y as f32, sum_b / div_b];
                let tb = [1.0, 0.0];
                let c = [x as f32, (y + 1) as f32, sum_c / div_c];
                let tc = [0.0, 1.0];
                let d = [(x + 1) as f32, (y + 1) as f32, sum_d / div_d];
                let td = [1.0, 1.0];
                let td = [d[0] / (width + 1) as f32, d[1] / (depth + 1) as f32];
                let normal = [0.0, 0.0, 1.0];
                vertices.push(Vertex {
                    position: a,
                    tex_coords: ta,
                    normal: normal
                });
                vertices.push(Vertex {
                    position: b,
                    tex_coords: tb,
                    normal: normal
                });
                vertices.push(Vertex {
                    position: c,
                    tex_coords: tc,
                    normal: normal
                });
                vertices.push(Vertex {
                    position: b,
                    tex_coords: tb,
                    normal: normal
                });
                vertices.push(Vertex {
                    position: d,
                    tex_coords: td,
                    normal: normal
                });
                vertices.push(Vertex {
                    position: c,
                    tex_coords: tc,
                    normal: normal
                });
                
            }
        }
        vertices
    }

    /// Render the level content to a Glium display
    pub fn render(&self, display: &Display, t: f32) {
        let right = 20.0;
        let left = -20.0;
        let top = 2.0;
        let bottom = 0.0;
        let far = 10.0;
        let near = 0.0;

        let (w, h) = display.get_framebuffer_dimensions();
        let aspect_ratio = (w as f32)/(h as f32);

        let vertices = self.get_vertices();
        let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let vertices_w = self.get_vertices_walls();
        let vertex_buffer_w =  glium::VertexBuffer::new(display, &vertices_w).unwrap();
        let indices_w = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let v3 = 3.0f32.sqrt();

        let pos = [5.0, t, 0.0];
        let y_ratio = 20.0;
        let x_ratio = y_ratio;
        let z_ratio = y_ratio;
        
        
        let uniforms = uniform! {
            perspective: [
//                [v3 / v6, 1.0/v6, v2/v6, 0.0],
//                [0.0, 2.0/v6, -v2/v6, 0.0],
//                [-v3 / v6, 1.0/v6, v2/v6, 0.0],
                //                [0.0, 0.0, 0.0, 1.0f32]
                [v3 / ( 2.0 * aspect_ratio), 0.5, 1.0/(x_ratio + y_ratio + z_ratio), 0.0],
                [-v3 / (2.0 * aspect_ratio), 0.5, 1.0/(x_ratio + y_ratio + z_ratio), 0.0],
                [0.0, 1.0, 1.0/(x_ratio + y_ratio + z_ratio), 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
                //[0.5, -0.5, 0.0, 0.0],
                //[v3/2.0, v3/2.0, 1.0, 0.0],
                //[1.0, 1.0, 1.0, 0.0],
                //[0.0, 0.0, 0.0, 1.0f32],
                    
            ],
            // transform: [
            //     [2.0 / (right - left), 0.0, 0.0, 0.0],
            //     [0.0, 2.0 / (top - bottom), 0.0, 0.0],
            //     [0.0, 0.0, -2.0 / (far - near), 0.0],
            //     [- (right + left) / (right - left), - (top + bottom) / (top - bottom), - (far + near) / (far - near), 1.0f32]
            // ]
            view: [
                [1.0/x_ratio, 0.0, 0.0, 0.0],
                [0.0, 1.0/y_ratio, 0.0, 0.0],
                [0.0, 0.0, 1.0/z_ratio, 0.0],
                [-pos[0]/x_ratio, -pos[1]/y_ratio, -pos[2]/z_ratio, 1.0f32]
            ],
        };


        let program =
            program!(display,
                     140 => {
                         vertex: "
#version 140
in vec3 position;
out vec2 v_tex_coords;

uniform mat4 perspective;
uniform mat4 view;
uniform vec2 tex_coords;

void main() {
    v_tex_coords = vec2(position[0]/10.0, position[1]/10.0);
    gl_Position = perspective * view * vec4(position, 1.0);
}
",
                         fragment: "
#version 140
out vec4 color;
in vec2 v_tex_coords;

void main() {
    color = vec4(v_tex_coords[0], v_tex_coords[1], 0.0, 1.0);
}
",
                     }).unwrap();

        let program_w =
            program!(display,
                     140 => {
                         vertex: "
#version 140
in vec3 position;
out vec2 v_tex_coords;

uniform mat4 perspective;
uniform mat4 view;
uniform vec2 tex_coords;

void main() {
    v_tex_coords = tex_coords;
    gl_Position = perspective * view * vec4(position, 1.0);
}
",
                         fragment: "
#version 140
out vec4 color;
in vec2 v_tex_coords;

void main() {
    color = vec4(1.0, 0.0, 0.0, 1.0);
}
",
                     }).unwrap();
        
                         
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };
        let mut frame = display.draw();
        frame.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        frame.draw(&vertex_buffer, &indices, &program, &uniforms, &params).unwrap();
        frame.draw(&vertex_buffer_w, &indices_w, &program_w, &uniforms, &params).unwrap();
        frame.finish().unwrap();
    }
}

#[test]
fn new_renderer() {
    let level: Level = Level::new(100, 100, 10.0);
    let renderer = Renderer::new(level);
}
