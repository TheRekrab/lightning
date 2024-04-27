use rand::prelude::*;
use std::collections::HashMap;

/// The probability that a wall will form that BLOCKS travel along the x axis. Between 0 and 1
static X_THRESHOLD: f64 = 0.1;

/// The probability that a wall will form that BLOCKS travel along the y axis. Between 0 and 1
static Y_THRESHOLD: f64 = 0.1;

struct SingleAxisWall {
    wall: HashMap<Coord, bool>,
    tolerance: f64,
}
impl SingleAxisWall {
    fn new(width: isize, height: isize, tolerance: f64, rng: &mut ThreadRng) -> Self {
        let mut wall: HashMap<Coord, bool> = HashMap::new();

        for x in 0..width {
            for y in 0..height {
                let coord: Coord = Coord::new(x, y);

                wall.insert(coord, rng.gen::<f64>() < tolerance);
            }
        }

        Self {
            wall,
            tolerance
        }
    }
}

struct Walls {
    x_walls: SingleAxisWall,
    y_walls: SingleAxisWall,
}
impl Walls {
    fn new(width: isize, height: isize) -> Self {
        let mut rng: ThreadRng = rand::thread_rng();
        let x_walls: SingleAxisWall = SingleAxisWall::new(width - 1, height, X_THRESHOLD, &mut rng);
        let y_walls: SingleAxisWall = SingleAxisWall::new(width, height - 1, Y_THRESHOLD, &mut rng);

        Self {
            x_walls,
            y_walls,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Coord {
    x: isize,
    y: isize,
}
impl Coord {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn some_adjs(
        &self,
        stops: &(HashMap<Coord, bool>, HashMap<Coord, bool>)
    ) -> Vec<Coord> {
        let left = Coord::new(self.x - 1, self.y);
        let right = Coord::new(self.x + 1, self.y);
        let up = Coord::new(self.x, self.y - 1);
        let down = Coord::new(self.x, self.y + 1);

        let mut some: Vec<Coord> = Vec::new();

        if (stops.)

        some
    }

    fn fits(&self, width: &isize, height: &isize) -> bool {
        if 0 > self.x || 0 > self.y || *width <= self.x || *height <= self.y {
            return false;
        }
        true
    }
}

pub struct Maze {
    connections: HashMap<Coord, Vec<Coord>>,
    width: isize,
    height: isize,
}
impl Maze {
    fn generate_single_stops(
        width: isize,
        height: isize,
        rng: &mut ThreadRng,
        threshold: f64,
    ) -> HashMap<Coord, bool> {
        let mut stops: HashMap<Coord, bool> = HashMap::new();
        for x in 0..width {
            for y in 0..height {
                let coord: Coord = Coord::new(x, y);
                if rng.gen::<f64>() < threshold {
                    stops.insert(coord, true);
                } else {
                    stops.insert(coord, false);
                }
            }
        }

        stops
    }

    fn generate_stops(
        width: &isize,
        height: &isize,
        rng: &mut ThreadRng,
    ) -> (HashMap<Coord, bool>, HashMap<Coord, bool>) {
        (
            Self::generate_single_stops(*width - 1, *height, rng, X_THRESHOLD), // x stops
            Self::generate_single_stops(*width, *height - 1, rng, Y_THRESHOLD), // y stops
        )
    }

    fn generate_board(width: &isize, height: &isize) -> HashMap<Coord, Vec<Coord>> {
        let mut connections: HashMap<Coord, Vec<Coord>> = HashMap::new();
        let mut rng: ThreadRng = rand::thread_rng();
        let stops: (HashMap<Coord, bool>, HashMap<Coord, bool>) =
            Self::generate_stops(&width, &height, &mut rng);

        // i can deref here because it implements the copy trait, and will not touch the original data
        for x in 0..*width {
            for y in 0..*height {
                let coord: Coord = Coord::new(x, y);
                let mut this_connections: Vec<Coord> = Vec::new();
                for possible_adj in coord.some_ajds(&mut rng, &stops) {
                    if possible_adj.fits(&width, &height) {
                        this_connections.push(possible_adj);
                    }
                }
                connections.insert(coord, this_connections);
            }
        }

        connections
    }

    pub fn new(width: isize, height: isize) -> Self {
        Self {
            width,
            height,
            connections: Self::generate_board(&width, &height),
        }
    }
}
impl std::fmt::Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut full_output: String = String::new();
        for x in 0..self.width {
            for y in 0..self.height {
                let coord: Coord = Coord::new(x, y);
                let connections = self.connections.get(&coord).unwrap(); // we know this will exist
                full_output.push_str(&format!("{:?}: {:?}\n", coord, connections));
            }
        }
        write!(f, "{}", full_output)
    }
}
