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

#[derive(Debug, PartialEq, Clone, Copy)]
/// Represents a wall inside the level.
///
/// A wall prevents the player from moving into adjacent tiles,
/// and should usually be displayed in some manner.
pub struct Wall {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
}

impl Wall {
    /// Creates a new wall representation with no wall at all
    pub fn none() -> Self {
        Wall {
            top: false,
            bottom: false,
            left: false,
            right: false,
        }
    }

    /// Returns true if there is no wall at all, false else
    pub fn is_none(&self) -> bool {
        *self == Wall::none()
    }
}

#[test]
fn wall_none() {
    let mut wall = Wall::none();
    wall.top = true;
    wall.top = false;
    assert!(wall.is_none());
}
