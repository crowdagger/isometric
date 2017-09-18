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

/// Represents a level.
///
/// Contains the floor, walls, and objects.
#[derive(Debug)]
pub struct Level {
    width: usize,
    height: usize,
    floor: Vec<f32>,
    walls: Vec<Wall>,
}

impl Level {
    /// Helper function that returns the index from x and y coordinates
    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
    
    /// Creates a new `Level` with default z set to 0.0.
    ///
    /// x axis will go from 0 to `width`.
    /// y axis will go from 0 to `height`.
    /// `default_z` is the default height in the world
    pub fn new(width: usize, height: usize, default_z: f32) -> Level {
        Level {
            width: width,
            height: height,
            floor: vec![default_z ; width * height],
            walls: vec![Wall::none() ; width * height],
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
        if end_pos.0 >= self.width || end_pos.1 >= self.height {
            return false;
        }
        
        let dx: isize = end_pos.0 as isize - start_pos.0 as isize;
        let dy: isize = end_pos.1 as isize - start_pos.1 as isize;
        if dx.abs() + dy.abs() != 1 {
            false
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

    /// Text representation of a level, mostly for debugging purposes
    pub fn to_ascii(&self) -> String {
        let mut res = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
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
}


#[test]
fn default_z() {
    let mut level = Level::new(10, 10, 10.0);
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
    Level::new(10, 10, 0.0).get_z(10, 0);
}

#[test]
#[should_panic]
fn invalid_y() {
    Level::new(10, 10, 0.0).get_z(0, 10);
}

#[test]
fn set_wall() {
    let mut level = Level::new(20, 20, 0.0);
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
    let mut level = Level::new(20, 20, 0.0);
    level.add_border_walls();
    assert_eq!(level.get_wall_ref(4, 0).bottom, true);
    assert_eq!(level.get_wall_ref(6, 19).top, true);
    assert_eq!(level.get_wall_ref(0, 12).left, true);
    assert_eq!(level.get_wall_ref(19, 7).right, true);
    assert!(level.get_wall_ref(2, 2).is_none());
}

#[test]
fn moves() {
    let mut level = Level::new(10, 10, 0.0);

    // Adjacent, ok
    assert_eq!(level.is_move_possible((0, 0), (1, 0)), true);
    assert_eq!(level.is_move_possible((1, 0), (1, 1)), true);
    assert_eq!(level.is_move_possible((1, 1), (0, 1)), true);
    assert_eq!(level.is_move_possible((0, 1), (0, 0)), true);

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
    let mut level = Level::new(10, 10, 0.0);
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
