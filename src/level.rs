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

use wall::WallPosition;

use std::default::Default;
use std::f32;

/// Represents a level.
///
/// Contains the floor, walls, and objects.
#[derive(Debug)]
pub struct Level<FT=(),
                 WT=()> {
    width: usize,
    depth: usize,
    floor: Vec<f32>,
    floor_data: Vec<FT>,
    walls_h: Vec<Option<WT>>,
    walls_v: Vec<Option<WT>>,
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
    /// y axis will go from 0 to `depth`.
    /// `default_z` is the default height in the world
    ///
    /// # Example
    ///
    /// ```
    /// use isometric::{Level, WallPosition};
    ///
    /// // Type annotation is necessary to use default parametric types (())
    /// let mut level: Level = Level::new(10, 10, 0.0);
    ///
    /// // Level is empty, so adjacent move is possible
    /// assert_eq!(level.is_move_possible((0, 2), (0, 3)), true);
    ///
    /// // Add a wall between (0, 2) and (0, 3)
    /// level.set_wall(0, 2, WallPosition::Top, Some(()));
    ///
    /// // Move between (0, 2) and (0, 3) is no longer possible
    /// assert_eq!(level.is_move_possible((0, 2), (0, 3)), false);
    /// ```

    pub fn new(width: usize, depth: usize, default_z: f32) -> Level<FT,WT> {
        Level {
            width: width,
            depth: depth,
            floor: vec![default_z ; width * depth],
            walls_h: vec![None; (depth + 1) * width],
            walls_v: vec![None; (width  + 1) * depth],
            floor_data: vec![FT::default() ; width * depth],
        }
    }

    /// Returns the width of a level
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the depth of a level
    pub fn depth(&self) -> usize {
        self.depth
    }

    /// Get the z value (height level in the world) of a tile
    ///
    /// x must be strictly less than level's width and
    /// y must be strictly less than level's height
    pub fn z(&self, x: usize, y: usize) -> f32 {
        debug_assert!(x < self.width && y < self.depth, "x and y must be in level's bounds");
        let i = self.get_index(x, y);
        self.floor[i]
    }

    /// Set the z value (height in the world) of a tile
    ///
    /// x must be strictly less than level's width and
    /// y must be strictly less than level's height
    pub fn set_z(&mut self, x: usize, y: usize, z: f32) -> &mut Self {
        debug_assert!(x < self.width && y < self.depth, "x and y must be in level's bounds");
        let i = self.get_index(x, y);
        self.floor[i] = z;
        self
    }

    /// Returns the height of a corner.
    ///
    /// A tile has four corners (yeah): (x, y) to (x+1, y +1). Therefore,
    /// the indice of corner can go one step further than the tile's index
    /// (max is width and depth instead of width - 1 and depth - 1).
    ///
    /// A corner's height is averaged from the neighboring tiles (if any),
    /// except if said tiles have a wall between them.
    // pub fn z_corner(&mut self, x: usize, y: usize) -> f32 {
    //         debug_assert!(x <= self.width && y <= self.depth, "x and y must be in level's bounds");
    //     let mut z = 0.0;
    //     let mut div_z = 0.0;
        
        
    //     return z / div_z;
    // }


    /// Returns the wall's data (if any) at a tile's position or None if there isn't.
    ///
    /// # Example
    ///
    /// ```
    /// use isometric::{Level, WallPosition};
    /// let level: Level = Level::new(10, 10, 0.0);
    /// assert!(level.wall(1, 1, WallPosition::Left).is_none());
    /// ```
    pub fn wall(&self, x: usize, y: usize, wall: WallPosition) -> &Option<WT> {
        debug_assert!(x < self.width && y < self.depth, "x and y must be in level's bounds");
        match wall {
            WallPosition::Bottom => &self.walls_h[x * (self.depth + 1) + y],
            WallPosition::Left => &self.walls_v[y * (self.width + 1) + x],
            WallPosition::Right => &self.walls_v[y * (self.width + 1) + x + 1],
            WallPosition::Top => &self.walls_h[x * (self.depth + 1) + y + 1],
        }
    }

    /// Sets the wall at tile x, y. To remove the wall, set it to `None`.
    pub fn set_wall(&mut self, x: usize, y: usize, wall: WallPosition, data: Option<WT>) {
        debug_assert!(x < self.width && y < self.depth, "x and y must be in level's bounds; ({}, {}) when bounds are ({}, {})", x, y, self.width, self.depth);
        match wall {
            WallPosition::Bottom => self.walls_h[x * (self.depth + 1) + y] = data,
            WallPosition::Left => self.walls_v[y * (self.width + 1) + x] = data,
            WallPosition::Right => self.walls_v[y * (self.width + 1) + x + 1] = data,
            WallPosition::Top => self.walls_h[x * (self.depth + 1) + y + 1] = data,
        }
    }

    /// Add walls to the border of the levels
    ///
    /// (bottom wall at y = 0, left wall at x = 0, and so on)
    pub fn add_border_walls(&mut self, data: WT) {
        let depth = self.depth;
        let width = self.width;
        for x in 0..self.width {
            self.set_wall(x, 0, WallPosition::Bottom, Some(data.clone()));
            self.set_wall(x, depth - 1, WallPosition::Top, Some(data.clone()));
        }
        for y in 0..self.depth {
            self.set_wall(0, y, WallPosition::Left, Some(data.clone()));
            self.set_wall(width - 1, y, WallPosition::Right, Some(data.clone()));
        }
    }

    /// Add walls between two tiles if the height (z) difference between
    /// the two is superior or equal to the given threshold.
    pub fn add_cliff_walls(&mut self, threshold: f32, data: WT) {
        for x in 0..(self.width - 1) {
            for y in 0..(self.depth - 1) {
                let z = self.z(x, y);
                let z_right = self.z(x + 1, y);
                let z_top = self.z(x, y + 1);

                if (z - z_top).abs() >= threshold {
                    self.set_wall(x, y, WallPosition::Top, Some(data.clone()));
                }
                if (z - z_right).abs() >= threshold {
                    self.set_wall(x, y, WallPosition::Right, Some(data.clone()));
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
        if end_pos.0 >= self.width || end_pos.1 >= self.depth {
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
            match (dx, dy) {
                (1, 0) => self.wall(start_pos.0, start_pos.1, WallPosition::Right).is_none(),
                (-1, 0) => self.wall(start_pos.0, start_pos.1, WallPosition::Left).is_none(),
                (0, 1) => self.wall(start_pos.0, start_pos.1, WallPosition::Top).is_none(),
                (0, -1) => self.wall(start_pos.0, start_pos.1, WallPosition::Bottom).is_none(),
                (_, _) => unreachable!(),
            } 
        }
    }

    /// Convenience method wrapping `visible_from`, returning a closure instead of
    /// a vector.
    ///
    /// # Example
    ///
    /// ```
    /// use isometric::Level;
    ///
    /// let mut level: Level = Level::new(20, 20, 0.0);
    /// let f = level.visibility((5, 5), 3);
    ///
    /// // You can now "ask" f whether some tile if visible from initial pos
    /// assert_eq!(f(4, 3), true);
    ///
    /// // Out of bounds range will simply return false
    /// assert_eq!(f(1000, 1000), false);
    /// ```
    pub fn visibility(&self, pos: (usize, usize), radius: usize) -> Box<Fn(usize, usize) -> bool> {
        let matrix = self.visible_from(pos, radius);

        let upper_bound = (if pos.0 + radius >= self.width() - 1 { self.width() - 1 } else { pos.0 + radius },
                           if pos.1 + radius >= self.width() - 1 { self.depth() - 1 } else { pos.1 + radius });
        let lower_bound = (if pos.0 > radius { pos.0 - radius } else { 0 },
                           if pos.1 > radius { pos.1 - radius } else { 0 });

        Box::new(move |x, y| {
            if x < lower_bound.0 || x > upper_bound.0 || y < lower_bound.1 || y > upper_bound.1 {
                false
            } else {
                matrix[x + radius - pos.0][y + radius - pos.1]
            }
        })
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
        for y in 0..self.depth {
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
                if self.wall(x, y, WallPosition::Left).is_some() {
                    res.push('|'); 
                } else {
                    res.push(' ');
                }
                if self.wall(x, y, WallPosition::Top).is_some() && self.wall(x, y, WallPosition::Bottom).is_some() {
                    res.push('=');
                } else if self.wall(x, y, WallPosition::Top).is_some() {
                    // reverted because display reverted
                    res.push('_');
                } else if self.wall(x, y, WallPosition::Bottom).is_some() {
                    // reverted because display reverted
                    res.push('-');
                } else {
                    res.push(' ');
                }
                if self.wall(x, y, WallPosition::Right).is_some() {
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
        debug_assert!(x < self.width && y < self.depth, "x and y must be in level's bounds");
        let i = self.get_index(x, y);
        self.floor_data[i] = data;
    }

    /// Gets a reference to custom floor data (e.g. the tile's representation)
    pub fn floor_data(&self, x: usize, y: usize) -> &FT {
        debug_assert!(x < self.width && y < self.depth, "x and y must be in level's bounds");
        let i = self.get_index(x, y);
        &self.floor_data[i]
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
                assert_eq!(level.z(x, y), 10.0);
            } else {
                assert_eq!(level.z(x, y), 42.0);
            }
        }
    }
}

#[test]
#[should_panic]
fn invalid_x() {
    let level: Level = Level::new(10, 10, 0.0);
    level.z(10, 0);
}

#[test]
#[should_panic]
fn invalid_y() {
    let level: Level = Level::new(10, 10, 0.0);
    level.z(0, 10);
}

#[test]
fn border_walls() {
    let mut level: Level = Level::new(20, 20, 0.0);
    level.add_border_walls(());
    assert!(level.wall(4, 0, WallPosition::Bottom).is_some());
    assert!(level.wall(6, 19, WallPosition::Top).is_some());
    assert!(level.wall(0, 12, WallPosition::Left).is_some());
    assert!(level.wall(19, 7, WallPosition::Right).is_some());
    assert!(level.wall(2, 2, WallPosition::Right).is_none());
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
    level.set_wall(0, 0, WallPosition::Right, Some(()));
    assert_eq!(level.is_move_possible((0, 0), (1, 0)), false);
}

#[test]
fn test_cliffs() {
    let mut level: Level = Level::new(10, 10, 0.0);
    level.set_z(1, 1, 10.0);
    level.add_cliff_walls(1.0, ());

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
    assert_eq!(level.floor_data(4, 4), &42);
    assert_eq!(level.floor_data(0, 0), &0);
}

#[test]
fn wall_data() {
    let mut level: Level<i32, i32> = Level::new(10, 10, 0.0);
    level.set_wall(4, 4, WallPosition::Right, Some(42));
    assert_eq!(level.wall(4, 4, WallPosition::Right).unwrap(), 42);
    assert_eq!(level.wall(5, 4, WallPosition::Left).unwrap(), 42);
    assert!(level.wall(0, 0, WallPosition::Right).is_none());
}
