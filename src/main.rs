use crate::maze::Maze;
mod coord;
mod maze;
mod walls;

fn main() {
    let maze: Maze = Maze::new(30, 40);
    let _res = maze.solve();
}
