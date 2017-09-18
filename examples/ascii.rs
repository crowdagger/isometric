extern crate isometric;

use isometric::Level;

fn main() {
    let mut level = Level::new(10, 10, 0.0);
    level.set_z(2, 2, 5.0);
    level.set_z(2, 3, 5.0);
    level.set_z(3, 2, 5.0);
    level.set_z(3, 3, 5.0);
    level.add_border_walls();
    level.add_cliff_walls(1.0);
    println!("{}", level.to_ascii());
}
