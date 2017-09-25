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

use glium::Display;

const V3: f32 = 1.732050807568877293; // sqrt of 3

/// Orthogonal camera.
#[derive(Debug, Clone)]
pub struct Camera {
    pos: [f32; 3],
    aspect_ratio: f32,
    y_ratio: f32,
    z_ratio: f32,
}

impl Camera {
    /// Creates a new camera with default settings
    pub fn new(display: &Display) -> Camera {
        let (w, h) = display.get_framebuffer_dimensions();
        let aspect_ratio = (w as f32)/(h as f32);
        Camera {
            aspect_ratio: aspect_ratio,
            pos: [0.0; 3],
            y_ratio: 5.0,
            z_ratio: 5.0,
            
        }
    }

    /// Retuns an array containing the positions of the camera
    pub fn pos(&self) -> [f32; 3] {
        self.pos
    }
    
    /// Set x and y position of the camera
    pub fn set_pos(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.pos[0] = x;
        self.pos[1] = y;
        self.pos[2] = z;
        self
    }

    /// Sets the (approximate) number of visible tiles (equivalent to zoom in/out)
    pub fn set_ratio(&mut self, ratio: f32) -> &mut Self {
        self.y_ratio = ratio;
        self.z_ratio = 0.5 * ratio;
        self
    }

    /// Get the perspective matrix
    pub fn perspective(&self) -> [[f32; 4]; 4] {
        [
            [V3 / (2.0 * self.aspect_ratio), 0.5, 0.5/(self.y_ratio + self.z_ratio), 0.0],
            [-V3 / (2.0 * self.aspect_ratio), 0.5, 0.5/(self.y_ratio + self.z_ratio), 0.0],
            [0.0, 1.0, -1.0/(self.y_ratio + self.z_ratio), 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]
    }

    /// Get the view matrix
    pub fn view(&self) -> [[f32; 4]; 4] {
        [
            [1.0/self.y_ratio, 0.0, 0.0, 0.0],
            [0.0, 1.0/self.y_ratio, 0.0, 0.0],
            [0.0, 0.0, 1.0/self.z_ratio, 0.0],
            [-self.pos[0]/self.y_ratio, -self.pos[1]/self.y_ratio, -self.pos[2]/self.z_ratio, 1.0f32]
        ]
    }
}
