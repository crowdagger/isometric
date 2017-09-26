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

use std::default::Default;

#[derive(Debug, PartialEq, Clone, Copy)]
/// Represents the position of a wall
pub enum WallPosition {
    /// Wall is at the left (wall with tile: (x - 1, y))
    Left,
    /// Wall is at the right (wall with tile: (x + 1, y))
    Right,
    /// Wall is at the top (wall with tile: (x, y + 1))
    Top,
    /// Wall is at the bottom (wall with tile: (x, y - 1))
    Bottom,
}


/// Trait that must be implemented by Wall data.
///
/// These functions are implemented for `()`, but not in a useful manner, obviously.
pub trait Wall: Default + Clone {
    /// Should return true if the wall is a cliff wall
    fn is_cliff(&self) -> bool;
    /// Should return true if the wall is a border wall
    fn is_border(&self) -> bool;
}

impl Wall for () {
    fn is_cliff(&self) -> bool {
        return false;
    }

    fn is_border(&self) -> bool {
        return false;
    }
}

/// A very simple implementation of wall
#[derive(Debug, Copy, Clone)]
pub enum SimpleWall {
    /// Cliff wall, where there isn't really a wall, more of a slope
    Cliff,
    /// Normal wall
    Normal,
    /// Border wall
    Border,
}

impl Wall for SimpleWall {
    fn is_cliff(&self) -> bool {
        match *self {
            SimpleWall::Cliff => true,
            _ => false,
        }
    }

    fn is_border(&self) -> bool {
        match *self {
            SimpleWall::Border => true,
            _ => false,
        }
    }
}

impl Default for SimpleWall {
    fn default() -> Self {
        SimpleWall::Normal
    }
}
