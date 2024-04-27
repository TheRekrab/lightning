use crate::lightning_path::LightningPath;
use std::collections::{HashMap, HashSet};

use rand::{rngs::ThreadRng, Rng};

use crate::{
    constants::{HEIGHT, WIDTH},
    coord::Coord,
    walls::Walls,
};

/// The maze that creates the shape of the lightning.
#[derive(Debug)]
pub struct Maze {
    corner: Coord,
    connections: HashMap<Coord, Vec<Coord>>,
}
impl Maze {
    /// Returns a new Maze, using the values from constants.rs
    pub fn new() -> Self {
        if WIDTH == 0 || HEIGHT == 0 {
            panic!("Invalid maze size received: ({WIDTH}, {HEIGHT})");
        }
        let corner: Coord = Coord::new(WIDTH - 1, HEIGHT - 1);
        let walls: Walls = Walls::new(&corner);
        let mut connections: HashMap<Coord, Vec<Coord>> = HashMap::new();

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let coord: Coord = Coord::new(x, y);
                connections.insert(coord.clone(), walls.connections(&coord));
            }
        }

        Self {
            corner,
            connections,
        }
    }
}

impl Maze {
    /// After the last spot is knows, goes back to find the shortest path to the start.
    fn backtrack(&self, distances: &HashMap<Coord, usize>, end: &Coord) -> LightningPath {
        let mut oldest_pose: Coord = end.clone();
        let mut path: Vec<Coord> = Vec::new();
        let mut best_dist: usize = *distances.get(end).unwrap();

        while distances.get(&oldest_pose).unwrap() != &0 {
            for adj in self.connections.get(&oldest_pose).unwrap() {
                if let Some(dist) = distances.get(&adj) {
                    if *dist < best_dist {
                        path.insert(0, adj.clone());
                        best_dist = *dist;
                        oldest_pose = adj.clone();
                    }
                }
            }
        }

        path.push(end.clone());

        LightningPath::new(path)
    }

    /// Returns a LightningPath that is the path that the lightning took. May return None if the maze is impossible to solve.
    pub fn solve(&self) -> Option<LightningPath> {
        let mut changed: HashSet<Coord> = HashSet::new();
        let mut distances: HashMap<Coord, usize> = HashMap::new();

        let mut rng: ThreadRng = rand::thread_rng();

        let final_location: Coord = Coord::new(rng.gen_range(0..=self.corner.x), 0);
        println!("target: {}", final_location);

        for x in 0..=self.corner.x {
            let coord: Coord = Coord::new(x, self.corner.y);
            changed.insert(coord.clone());
            distances.insert(coord.clone(), 0);
        }

        // time to solve the actual maze, if possible
        let mut dist: usize = 0;

        while !changed.is_empty() && !distances.contains_key(&final_location) {
            let mut new_changed: HashSet<Coord> = HashSet::new();
            for coord in &changed {
                for adj in self.connections.get(coord).unwrap() {
                    let mut update_distance: bool = true;
                    if distances.contains_key(&adj) {
                        if distances.get(&adj).unwrap() < &dist {
                            update_distance = false;
                        }
                    }

                    if update_distance {
                        distances.insert(adj.clone(), dist);
                        new_changed.insert(adj.clone());
                    }
                }
            }
            dist += 1;
            changed = new_changed;
        }

        if changed.is_empty() {
            return None; // the path is not solvable.
        }

        println!("took {dist} moves to solve");

        Some(self.backtrack(&distances, &final_location))
    }
}
