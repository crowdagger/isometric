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
use camera::Camera;
use wall::WallPosition;
use wall::Wall;

use glium;
use glium::Surface;
use glium::Display;
use glium::program::Program;

use image;

use std::default::Default;
use std::io::Cursor;

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
    normal: [f32; 3],
    lighted: f32,
}

implement_vertex!(Vertex, position, tex_coords, normal, lighted);

/// Contains a level and add methods to render it
pub struct Renderer<'a, FT=(), WT=()> {
    level: Level<FT, WT>,
    display: &'a Display,
    program: Program,
}

impl<'a,
     FT:Clone+Default,
     WT:Wall> Renderer<'a, FT, WT> {
    /// Creates a new renderer from an existing level and a glutin display
    pub fn new(level: Level<FT, WT>, display: &'a Display) -> Renderer<'a, FT, WT> {
        Renderer {
            level: level,
            display: display,
            program: program!(display,
                              140 => {
                                  vertex: include_str!("../shaders/renderer.glslv"),
                                  fragment: include_str!("../shaders/renderer.glslf"),
                     }).unwrap(),
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

    // Add vertical wall to the vertices
    fn add_horizontal_wall(&self, vertices: &mut Vec<Vertex>, data: &WT,
                           f: &Fn(usize, usize) -> f32,
                           x: usize, y: usize, z: f32, other_z: f32) {
        let other_z = if !data.is_cliff() {
            z + 1.0
        } else {
            other_z
        };
        let lighted = {
            let y = if y > 0 { y - 1 } else { y };
            f(x, y)
        };
        println!("light for ({}, {}) is {}", x, y, lighted);
        let x = x as f32;
        let y = y as f32;
        let a = [x, y, z];
        let b = [x + 1.0, y, z];
        let c = [x, y, other_z];
        let d = [x + 1.0, y, other_z];
        let normal = [0.0, -1.0, 0.0];
        vertices.push(Vertex {
            position: a,
            tex_coords: [0.0, 1.0],
            normal: normal,
            lighted: lighted
        });
        vertices.push(Vertex {
            position: b,
            tex_coords: [1.0, 0.0],
            normal: normal,
            lighted: lighted
        });
        vertices.push(Vertex {
            position: c,
            tex_coords: [0.0, 1.0],
            normal: normal,
            lighted: lighted
        });
        vertices.push(Vertex {
            position: b,
            tex_coords: [1.0, 0.0],
            normal: normal,
            lighted: lighted
        });
        vertices.push(Vertex {
            position: d,
            tex_coords: [1.0, 1.0],
            normal: normal,
            lighted: lighted
        });
        vertices.push(Vertex {
            position: c,
            tex_coords: [0.0, 1.0],
            normal: normal,
            lighted: lighted
        });
    }

    // Add horizontal wall to the vertices
    fn add_vertical_wall(&self, vertices: &mut Vec<Vertex>, data: &WT,
                         f: &Fn(usize, usize) -> f32,
                         x: usize, y: usize, z: f32, other_z: f32) {
        let other_z = if !data.is_cliff() {
            z + 1.0
        } else {
            other_z
        };
        let lighted = {
            let x = if x > 0 { x - 1 } else { x };
            f(x, y)
        };
        let x = x as f32;
        let y = y as f32;
        let a = [x, y, z];
        let b = [x, y + 1.0, z];
        let c = [x, y, other_z];
        let d = [x, y + 1.0, other_z];
        let normal = [-1.0, 0.0, 0.0];
        vertices.push(Vertex {
            position: a,
            tex_coords: [0.0, 0.0],
            normal: normal,
            lighted: lighted,
        });
        vertices.push(Vertex {
            position: b,
            tex_coords: [1.0, 0.0],
            normal: normal,
            lighted: lighted,
        });
        vertices.push(Vertex {
            position: c,
            tex_coords: [0.0, 1.0],
            normal: normal,
            lighted: lighted,
        });
        vertices.push(Vertex {
            position: b,
            tex_coords: [1.0, 0.0],
            normal: normal,
            lighted: lighted,
        });
        vertices.push(Vertex {
            position: d,
            tex_coords: [1.0, 1.0],
            normal: normal,
            lighted: lighted,
        });
        vertices.push(Vertex {
            position: c,
            tex_coords: [0.0, 1.0],
            normal: normal,
            lighted: lighted,
        });
    }
    
    /// Return the vertices corresponding to the walls' data
    fn get_vertices_walls(&self, f: &Fn(usize, usize) -> f32) -> Vec<Vertex> {
        let mut vertices = vec!();
        let level = self.level();
        let width = level.width();
        let depth = level.depth();

        for x in 0..width {
            for y in 0..depth {
                let z = level.z(x, y);
                if let &Some(ref data) = level.wall(x, y, WallPosition::Bottom) {
                    if y == 0 {
                        self.add_horizontal_wall(&mut vertices, data, f, x, y, z, z + 1.0);
                    } else {
                        self.add_horizontal_wall(&mut vertices, data, f, x, y, z, level.z(x, y - 1));
                    }
                }
                if let &Some(ref data) = level.wall(x, y, WallPosition::Left) {
                    if x == 0 {
                        self.add_vertical_wall(&mut vertices, data, f, x, y, z, z + 1.0);
                    } else {
                        self.add_vertical_wall(&mut vertices, data, f, x, y, z, level.z(x - 1, y));
                    }
                }
                if let &Some(ref data) = level.wall(x, y, WallPosition::Top) {
                    if y == depth - 1 {
                        self.add_horizontal_wall(&mut vertices, data, f, x, y + 1, z, z + 1.0);
                    } else {
                        self.add_horizontal_wall(&mut vertices, data, f, x, y + 1, z, level.z(x, y + 1));
                    }
                }
                if let &Some(ref data) = level.wall(x, y, WallPosition::Right)  {
                    if x == width - 1 {
                        self.add_vertical_wall(&mut vertices, data, f, x + 1, y, z, z + 1.0);
                    } else {
                        self.add_vertical_wall(&mut vertices, data, f, x + 1, y, z, level.z(x + 1, y));
                    }
                }
            }
        }
        vertices
    }
    
    /// Returns the vertices corresponding to the level's data
    fn get_vertices(&self, f: &Fn(usize, usize) -> f32) -> Vec<Vertex> {
        let mut vertices = vec!();
        let level = self.level();
        let width = level.width();
        let depth = level.depth();
        for x in 0..width {
            for y in 0..depth {
                let z = level.z(x, y);
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
                    let z = level.z(x - 1, y);
                    sum_a += z;
                    div_a += 1.0;
                    sum_c += z;
                    div_c += 1.0;
                    if y > 0 && level.is_move_possible((x, y), (x - 1, y - 1)) {
                        let z = level.z(x -1, y - 1);
                        sum_a += z;
                        div_a += 1.0;
                    }
                }
                if y > 0 && level.is_move_possible((x, y), (x, y - 1)) {
                    let z = level.z(x, y - 1);
                    sum_a += z;
                    div_a += 1.0;
                    sum_b += z;
                    div_b += 1.0;
                    if x < width - 1 && level.is_move_possible((x, y), (x + 1, y - 1)) {
                        let z = level.z(x + 1, y - 1);
                        sum_b += z;
                        div_b += 1.0;
                    }
                }
                if x < width - 1 && level.is_move_possible((x, y), (x + 1, y)) {
                    let z = level.z(x + 1, y);
                    sum_b += z;
                    div_b += 1.0;
                    sum_d += z;
                    div_d += 1.0;
                    if y < depth - 1 && level.is_move_possible((x, y), (x + 1, y + 1)) {
                        let z = level.z(x + 1, y + 1);
                        sum_d += z;
                        div_d += 1.0;
                    }
                }
                if y < depth - 1 && level.is_move_possible((x, y), (x, y + 1)) {
                    let z = level.z(x, y + 1);
                    sum_c += z;
                    div_c += 1.0;
                    sum_d += z;
                    div_d += 1.0;
                    if x > 0 && level.is_move_possible((x, y), (x - 1, y + 1)) {
                        let z = level.z(x - 1, y + 1);
                        sum_c += z;
                        div_c += 1.0;
                    }
                }

                // Finally build the four vertices
                let a = [x as f32, y as f32, sum_a / div_a];
                let ta = [0.0 + a[0] / (width as f32 + 1.0), 0.0 + a[1] / (width as f32 + 1.0)];
                let b = [(x + 1) as f32, y as f32, sum_b / div_b];
                let tb = [0.0 + b[0] / (width as f32 + 1.0), 0.0 + b[1] / (width as f32 + 1.0)];
                let c = [x as f32, (y + 1) as f32, sum_c / div_c];
                let tc = [0.0 + c[0] / (width as f32 + 1.0), 0.0 + c[1] / (width as f32 + 1.0)];
                let d = [(x + 1) as f32, (y + 1) as f32, sum_d / div_d];
                let td = [0.0 + d[0] / (width as f32 + 1.0), 0.0 + d[1] / (width as f32 + 1.0)];
                let lighted = f(x, y);
                let normal = [0.0, 0.0, 1.0];
                vertices.push(Vertex {
                    position: a,
                    tex_coords: ta,
                    normal: normal,
                    lighted: lighted
                });
                vertices.push(Vertex {
                    position: b,
                    tex_coords: tb,
                    normal: normal,
                    lighted: lighted
                });
                vertices.push(Vertex {
                    position: c,
                    tex_coords: tc,
                    normal: normal,
                    lighted: lighted
                });
                vertices.push(Vertex {
                    position: b,
                    tex_coords: tb,
                    normal: normal,
                    lighted: lighted
                });
                vertices.push(Vertex {
                    position: d,
                    tex_coords: td,
                    normal: normal,
                    lighted: lighted
                });
                vertices.push(Vertex {
                    position: c,
                    tex_coords: tc,
                    normal: normal,
                    lighted: lighted
                });
                
            }
        }
        vertices
    }

    /// Render the level content to a Glium display
    pub fn render(&self, display: &Display, camera: &Camera) {
        let pos = camera.pos();
        let visible = self.level.visibility((pos[0] as usize, pos[1] as usize), 5);
        let f = |x, y| {
            if visible(x, y) {
                let x = x as f32;
                let y = y as f32;
                1.0/(1.0 + (((x-pos[0])*(x-pos[0]) + (y - pos[1]) * (y - pos[1])) ) / 25.0)
            } else {
                0.0
            }
        };
        
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        
        let vertices = self.get_vertices(&f);
        let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();

        let vertices_w = self.get_vertices_walls(&f);
        let vertex_buffer_w =  glium::VertexBuffer::new(display, &vertices_w).unwrap();


        let image = image::load(Cursor::new(&include_bytes!("../assets/floor_1.png")[..]),
                                image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw()[..], image_dimensions);
        let floor_texture = glium::texture::Texture2d::new(display, image).unwrap();
        let image = image::load(Cursor::new(&include_bytes!("../assets/wall_1.png")[..]),
                                image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw()[..], image_dimensions);
        let wall_texture = glium::texture::Texture2d::new(display, image).unwrap();

        
        let uniforms = uniform! {
            perspective: camera.perspective(),
            view: camera.view(),
            tex: &floor_texture,
            v_light: [1.0, 0.0, 0.0f32],
            light_color: [1.0, 1.0, 1.0f32],
            dark_color: [0.25, 0.25, 0.5f32],
            wood_tex: &wall_texture,
        };


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
        frame.draw(&vertex_buffer, &indices, &self.program,
                   &uniform! {
                       perspective: camera.perspective(),
                       view: camera.view(),
                       tex: &floor_texture,
                       v_light: [1.0, 0.0, 0.0f32],
                       light_color: [1.0, 1.0, 1.0f32],
                       dark_color: [0.75, 0.75, 1.0f32],
                   },
                   &params).unwrap();
        frame.draw(&vertex_buffer_w, &indices, &self.program,
                   &uniform! {
                       perspective: camera.perspective(),
                       view: camera.view(),
                       tex: &wall_texture,
                       v_light: [1.0, 0.0, 0.0f32],
                       light_color: [1.0, 1.0, 1.0f32],
                       dark_color: [0.75, 0.75, 1.0f32],
                   },
                   &params).unwrap();

        // let vertices = vec![
        //     Vertex{
        //         position: [2.0 -0.5, 2.5, 0.0],
        //         normal: [-1.0, -1.0, 0.0],
        //         tex_coords: [0.0, 0.0]
        //     },
        //     Vertex{
        //         position: [2.5, 2.0 - 0.5, 0.0],
        //         normal: [-1.0, -1.0, 0.0],
        //         tex_coords: [0.0, 1.0]
        //     },
        //     Vertex{
        //         position: [1.5, 2.5, 1.0],
        //         normal: [-1.0, -1.0, 0.0],
        //         tex_coords: [1.0, 0.0]
        //     },
        //     Vertex{
        //         position: [2.5, 1.5, 0.0],
        //         normal: [-1.0, -1.0, 0.0],
        //         tex_coords: [0.0, 1.0]
        //     },
        //     Vertex{
        //         position: [2.5, 1.5, 1.0],
        //         normal: [-1.0, -1.0, 0.0],
        //         tex_coords: [1.0, 1.0]
        //     },
        //     Vertex{
        //         position: [1.5, 2.5, 1.0],
        //         normal: [-1.0, -1.0, 0.0],
        //         tex_coords: [1.0, 0.0]
        //     },
        //     ];
        // let new_vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
        // let new_indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList        frame.draw(&new_vertex_buffer, &new_indices, &program_w, &uniforms, &params).unwrap();
        frame.finish().unwrap();
    }
}

