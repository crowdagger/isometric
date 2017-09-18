extern crate isometric;

use isometric::Level;
use std::io;
use std::io::Read;

fn main() {
    let mut level: Level = Level::new(10, 10, 0.0);
    level.set_z(2, 2, 5.0);
    level.set_z(2, 3, 5.0);
    level.set_z(3, 2, 5.0);
    level.set_z(3, 3, 5.0);
    level.add_border_walls();
    level.add_cliff_walls(1.0);
    let mut pos = (0, 0);
    let mut new_pos = (0, 0);
    while true {
        if level.is_move_possible(pos, new_pos) {
            pos = new_pos;
        }
        println!("{}", level.to_ascii(pos, 5));
        println!("");
        println!("Input? (l/r/t/b, or other to quit)");

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        if buffer.is_empty() {
            break;
        } 
        let char = buffer.chars().next().unwrap();
        new_pos = pos;
        match char {
            'l' => if new_pos.0 > 0 { new_pos.0 -= 1; },
            'r' => new_pos.0 += 1,
            't' => if new_pos.1 > 0 { new_pos.1 -= 1; },
            'b' => new_pos.1 += 1,
            _ => break,
        }
    }
}
