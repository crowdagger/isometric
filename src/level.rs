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

use wall::Wall;

use std::default::Default;
use std::f32;

/// Represents a level.
///
/// Contains the floor, walls, and objects.
#[derive(Debug)]
pub struct Level<FT=(),
                 WT=()> {
    width: usize,
    height: usize,
    floor: Vec<f32>,
    floor_data: Vec<FT>,
    wall_data: Vec<WT>,
    walls: Vec<Wall>,
}

impl<FT:Default+Clone,
     WT:Default+Clone> Level<FT, WT> {
    /// Helper function that returns the index from x and y coordinates
    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
    
    /// Creates a new `Level` with default z set to 0.0.
    ///
    /// x axis will go from 0 to `width`.
    /// y axis will go from 0 to `height`.
    /// `default_z` is the default height in the world
    pub fn new(width: usize, height: usize, default_z: f32) -> Level<FT,WT> {
        Level {
            width: width,
            height: height,
            floor: vec![default_z ; width * height],
            walls: vec![Wall::none() ; width * height],
            floor_data: vec![FT::default() ; width * height],
            wall_data: vec![WT::default() ; width * height],
        }
    }

    /// Get the z value (height level in the world) of a tile
    ///
    /// x must be strictly less than level's width and
    /// y must be strictly less than level's height
    pub fn get_z(&self, x: usize, y: usize) -> f32 {
        debug_assert!(x < self.width && y < self.height, "x and y must be in level's bounds");
        let i = self.get_index(x, y);
        self.floor[i]
    }

    /// Set the z value (height in the world) of a tile
    ///
    /// x must be strictly less than level's width and
    /// y must be strictly less than level's height
    pub fn set_z(&mut self, x: usize, y: usize, z: f32) -> &mut Self {
        debug_assert!(x < self.width && y < self.height, "x and y must be in level's bounds");
        let i = self.get_index(x, y);
        self.floor[i] = z;
        self
    }

    /// Get a reference to a tile's walls (so you can check it)
    pub fn get_wall_ref(&self, x: usize, y: usize) -> &Wall {
        debug_assert!(x < self.width && y < self.height, "x and y must be in level's bounds");
        let i = self.get_index(x, y);
        &self.walls[i]
    }

    /// Get a mutable access to a tile's walls (so you can add/remove walls to it)
    pub fn get_wall_mut(&mut self, x: usize, y: usize) -> &mut Wall {
        debug_assert!(x < self.width && y < self.height, "x and y must be in level's bounds");
        let i = self.get_index(x, y);
        &mut self.walls[i]
    }

    /// Add walls to the border of the levels
    ///
    /// (bottom wall at y = 0, left wall at x = 0, and so on)
    pub fn add_border_walls(&mut self) {
        for x in 0..self.width {
            let i = self.get_index(x, 0);
            let j = self.get_index(x, self.height - 1);
            self.walls[i].bottom = true;
            self.walls[j].top = true;
        }
        for y in 0..self.height {
            let i = self.get_index(0, y);
            let j = self.get_index(self.width - 1, y);
            self.walls[i].left = true;
            self.walls[j].right = true;
        }
    }

    /// Add walls between two tiles if the height (z) difference between
    /// the two is superior or equal to the given threshold.
    pub fn add_cliff_walls(&mut self, threshold: f32) {
        for x in 0..(self.width - 1) {
            for y in 0..(self.height - 1) {
                let z = self.get_z(x, y);
                let z_right = self.get_z(x + 1, y);
                let z_top = self.get_z(x, y + 1);

                if (z - z_top).abs() >= threshold {
                    self.get_wall_mut(x, y).top = true;
                }
                if (z - z_right).abs() >= threshold {
                    self.get_wall_mut(x, y).right = true;
                }
            }
        }
    }

    /// Returns true if character move is possible, false else
    ///
    /// A move is possible if:
    /// * start position and end position are adjacent
    /// * there is no wall between them
    /// * end position is not outside the level
    pub fn is_move_possible(&self,
                            start_pos: (usize, usize),
                            end_pos: (usize, usize)) -> bool {
        if start_pos == end_pos {
            // Trivially true, though useless
            return true;
        }
        if end_pos.0 >= self.width || end_pos.1 >= self.height {
            return false;
        }
        
        let dx: isize = end_pos.0 as isize - start_pos.0 as isize;
        let dy: isize = end_pos.1 as isize - start_pos.1 as isize;
        if dx.abs() > 1 || dy.abs() > 1 {
            // Not adjacent
            false
        } else if dx.abs() + dy.abs() == 2 {
            // Diagonal
            // Move is possible if first dx then dy is possible OR
            // first dy then dx is possible

            let intermediate_x = (start_pos.0 as isize + dx) as usize;
            let intermediate = (intermediate_x, start_pos.1);
            if self.is_move_possible(start_pos, intermediate)
                && self.is_move_possible(intermediate, end_pos) {
                    true
                } else {
                    let intermediate_y = (start_pos.1 as isize + dy) as usize;
                    let intermediate = (start_pos.0, intermediate_y);
                    self.is_move_possible(start_pos, intermediate) && 
                        self.is_move_possible(intermediate, end_pos)
                }
        } else {
            let wall_start = self.get_wall_ref(start_pos.0, start_pos.1);
            let wall_end = self.get_wall_ref(end_pos.0, end_pos.1);
            match (dx, dy) {
                (1, 0) => !wall_start.right && !wall_end.left,
                (-1, 0) => !wall_start.left && !wall_end.right,
                (0, 1) => !wall_start.top && !wall_end.bottom,
                (0, -1) => !wall_start.bottom && !wall_end.top,
                (_, _) => unreachable!(),
            } 
        }
    }

    /// Returns a "visibility" matrix of bools centered on the pos view.
    ///
    /// This is computionally expensive if radius is a bit large (like O(n^3, not good at all))
    ///
    /// Center ((0,0) in relative position compared to pos) is at index `radius`
    pub fn visible_from(&self, pos: (usize, usize), radius: usize) -> Vec<Vec<bool>> {
        let mut res = vec![];
        for _ in 0..(2*radius + 1) {
            res.push(vec![false; 2 * radius + 1]);
        }
        // Special case of tile of the POV
        res[radius][radius] = true;

        let start_x = if radius > pos.0 { 0 } else { pos.0 - radius };
        let end_x = if pos.0 + radius > self.width - 1 { self.width - 1 } else { pos.0 + radius };
        let start_y = if radius > pos.1 { 0 } else { pos.1 - radius };
        let end_y = if pos.1 + radius > self.height - 1 { self.height - 1 } else { pos.1 + radius };

        let steps = 10 * radius;
        let dtheta: f32 = 2.0 * f32::consts::PI / (steps as f32);
        let two_pies = 2.0 * f32::consts::PI;

        let mut theta: f32 = 0.0;

        while theta <= two_pies {
            let dx = theta.cos();
            let dy = theta.sin();

            let max_dist = (radius * radius) as f32;
            let mut x = 0.0;
            let mut y = 0.0;
            let mut prev_x = pos.0;
            let mut prev_y = pos.1;

            while x * x + y * y  < max_dist {
                x += dx;
                y += dy;
                let abs_x = (x + pos.0 as f32).round();
                let abs_y = (y + pos.1 as f32).round();
                if abs_x < 0.0 || abs_y < 0.0 {
                    continue;
                }
                let abs_x = abs_x as usize;
                let abs_y = abs_y as usize;
                    
                if abs_x == prev_x && abs_y == prev_y {
                    // nothing to do, we already computed this tile
                    continue;
                }
                if self.is_move_possible((prev_x, prev_y), (abs_x, abs_y)) {
                    let i = (x as f32).round() as isize + radius as isize;
                    let j = (y as f32).round() as isize + radius as isize;
                    if i < 0 || i >= 2 * (radius as isize) + 1 {
                        continue;
                    }
                    if j < 0 || j >= 2 * (radius as isize) + 1 {
                        continue;
                    }
                    res[i as usize][j as usize] = true;
                    prev_x = abs_x;
                    prev_y = abs_y;
                } else {
                    let i = (x as f32).round() as isize + radius as isize;
                    let j = (y as f32).round() as isize + radius as isize;
                    break;
                }
            }
            theta += dtheta;
        }
        res
    }

    /// Text representation of a level, mostly for debugging purposes
    ///
    /// pos: the position of the point of view
    /// radius: the visibility radius
    pub fn to_ascii(&self, pos: (usize, usize), radius: usize) -> String {
        let mut res = String::new();
        let visible = self.visible_from(pos, radius);
        for y in 0..self.height {
            for x in 0..self.width {
                if (x, y) == pos {
                    res.push_str(" @ ");
                    continue;
                }
                let x_v = x as isize - pos.0 as isize + radius as isize;
                let y_v = y as isize - pos.1 as isize + radius as isize;
                if x_v < 0 || x_v > 2 * radius as isize || y_v < 0 || y_v > 2 * radius as isize {
                    res.push_str("###");
                    continue;
                }
                if !visible[x_v as usize][y_v as usize] {
                    res.push_str("###");
                    continue;
                }
                let wall = self.get_wall_ref(x, y);
                if wall.left {
                    res.push('|'); 
                } else {
                    res.push(' ');
                }
                if wall.top && wall.bottom {
                    res.push('=');
                } else if wall.top {
                    // reverted because display reverted
                    res.push('_');
                } else if wall.bottom {
                    // reverted because display reverted
                    res.push('-');
                } else {
                    res.push(' ');
                }
                if wall.right {
                    res.push('|'); 
                } else {
                    res.push(' ');
                }
            }
            res.push('\n');
        }
        res
    }

    /// Sets the value of custom floor data (e.g. the tile's representation)
    pub fn set_floor_data(&mut self, x: usize, y: usize, data: FT) {
        debug_assert!(x < self.width && y < self.height, "x and y must be in level's bounds");
        let i = self.get_index(x, y);
        self.floor_data[i] = data;
    }

    /// Gets a reference to custom floor data (e.g. the tile's representation)
    pub fn get_floor_data(&self, x: usize, y: usize) -> &FT {
        debug_assert!(x < self.width && y < self.height, "x and y must be in level's bounds");
        let i = self.get_index(x, y);
        &self.floor_data[i]
    }

    /// Sets the value of custom wall data (e.g. the wall's potential representation)
    pub fn set_wall_data(&mut self, x: usize, y: usize, data: WT) {
        debug_assert!(x < self.width && y < self.height, "x and y must be in level's bounds");
        let i = self.get_index(x, y);
        self.wall_data[i] = data;
    }

    /// Gets a reference to custom wall data (e.g. the wall's potential representation)
    pub fn get_wall_data(&self, x: usize, y: usize) -> &WT {
        debug_assert!(x < self.width && y < self.height, "x and y must be in level's bounds");
        let i = self.get_index(x, y);
        &self.wall_data[i]
    }
    }


#[test]
fn default_z() {
    let mut level:Level = Level::new(10, 10, 10.0);
    for x in 0..10 {
        level.set_z(x, x, 42.0);
    }
    for x in 0..10 {
        for y in 0..10 {
            if x != y {
                assert_eq!(level.get_z(x, y), 10.0);
            } else {
                assert_eq!(level.get_z(x, y), 42.0);
            }
        }
    }
}

#[test]
#[should_panic]
fn invalid_x() {
    let level: Level = Level::new(10, 10, 0.0);
    level.get_z(10, 0);
}

#[test]
#[should_panic]
fn invalid_y() {
    let level: Level = Level::new(10, 10, 0.0);
    level.get_z(0, 10);
}

#[test]
fn set_wall() {
    let mut level: Level = Level::new(20, 20, 0.0);
    {
        let mut wall = level.get_wall_mut(5,5);
        wall.top = true;
        wall.left = true;
    }
    assert!(level.get_wall_ref(2, 2).is_none());
    assert!(!level.get_wall_ref(5, 5).is_none());
}

#[test]
fn border_walls() {
    let mut level: Level = Level::new(20, 20, 0.0);
    level.add_border_walls();
    assert_eq!(level.get_wall_ref(4, 0).bottom, true);
    assert_eq!(level.get_wall_ref(6, 19).top, true);
    assert_eq!(level.get_wall_ref(0, 12).left, true);
    assert_eq!(level.get_wall_ref(19, 7).right, true);
    assert!(level.get_wall_ref(2, 2).is_none());
}

#[test]
fn move_self() {
    let mut level: Level = Level::new(10, 10, 0.0);
    assert_eq!(level.is_move_possible((0, 0), (0, 0)), true);
}

#[test]
fn moves() {
    let mut level: Level = Level::new(10, 10, 0.0);

    // Adjacent, ok
    assert_eq!(level.is_move_possible((0, 0), (1, 0)), true);
    assert_eq!(level.is_move_possible((1, 0), (1, 1)), true);
    assert_eq!(level.is_move_possible((1, 1), (0, 1)), true);
    assert_eq!(level.is_move_possible((0, 1), (0, 0)), true);

    // Diagonal, ok
    assert_eq!(level.is_move_possible((0, 0), (1, 1)), true);
    assert_eq!(level.is_move_possible((1, 0), (0, 1)), true);
    
    // Not adjacent
    assert_eq!(level.is_move_possible((0, 0), (2, 0)), false);

    // Dest is outside map
    assert_eq!(level.is_move_possible((9, 9), (9, 10)), false);

    // Add a wall, move no longer possible
    level.get_wall_mut(0, 0).right = true;
    assert_eq!(level.is_move_possible((0, 0), (1, 0)), false);
}

#[test]
fn test_cliffs() {
    let mut level: Level = Level::new(10, 10, 0.0);
    level.set_z(1, 1, 10.0);
    level.add_cliff_walls(1.0);

    // Move ok 
   assert_eq!(level.is_move_possible((0, 0), (1, 0)), true);
    // Too much of a step
    assert_eq!(level.is_move_possible((1, 0), (1, 1)), false);
    assert_eq!(level.is_move_possible((1, 1), (1, 2)), false);
    assert_eq!(level.is_move_possible((0, 1), (1, 1)), false);
    assert_eq!(level.is_move_possible((1, 1), (2, 1)), false);
    assert_eq!(level.is_move_possible((2, 1), (1, 1)), false);
}

#[test]
fn floor_data() {
    let mut level: Level<i32, i32> = Level::new(10, 10, 0.0);
    level.set_floor_data(4, 4, 42);
    assert_eq!(level.get_floor_data(4, 4), &42);
    assert_eq!(level.get_floor_data(0, 0), &0);
}

#[test]
fn wall_data() {
    let mut level: Level<i32, i32> = Level::new(10, 10, 0.0);
    level.set_wall_data(4, 4, 42);
    assert_eq!(level.get_wall_data(4, 4), &42);
    assert_eq!(level.get_wall_data(0, 0), &0);
}
