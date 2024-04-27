use crate::image_gen::generate_image;
use crate::maze::Maze;
mod coord;
mod image_gen;
mod maze;
mod walls;

const WIDTH: usize = 130;
const HEIGHT: usize = 300;

fn main() {
    let maze: Maze = Maze::new(WIDTH, HEIGHT);
    let res = maze.solve();
    if let Some(path) = res {
        generate_image(WIDTH, HEIGHT, &path);
    }
}
