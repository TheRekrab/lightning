use crate::coord::Coord;
use crate::walls::Walls;
mod coord;
mod walls;

fn main() {
    let corner: Coord = Coord::new(5, 5);
    let walls: Walls = Walls::new(&corner);

    println!("{walls:#?}");
}
