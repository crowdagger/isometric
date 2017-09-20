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
/// Represents the position of a wall
pub enum Wall {
    /// Wall is at the left (wall with tile: (x - 1, y))
    Left,
    /// Wall is at the right (wall with tile: (x + 1, y))
    Right,
    /// Wall is at the top (wall with tile: (x, y + 1))
    Top,
    /// Wall is at the bottom (wall with tile: (x, y - 1))
    Bottom,
}
