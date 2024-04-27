use crate::constants::{X_THRESHOLD, Y_THRESHOLD};
use std::collections::HashSet;

use rand::prelude::*;

use crate::coord::Coord;

/// A wall that blocks movement a single direction
#[derive(Debug)]
struct Wall {
    stops: HashSet<Coord>,
}
impl Wall {
    /// Creates a new Wall. Tolerance is the chance that there will be a wall at some given point.
    fn new(x: usize, y: usize, tolerance: f64) -> Self {
        let mut stops: HashSet<Coord> = HashSet::new();
        let mut rng: ThreadRng = rand::thread_rng();

        for x in 0..=x {
            for y in 0..=y {
                let generated: f64 = rng.gen();
                if generated < tolerance {
                    stops.insert(Coord::new(x, y));
                }
            }
        }

        Self { stops }
    }

    /// Returns if there is a stop at some given point
    fn has_stop(&self, coord: &Coord) -> bool {
        self.stops.contains(coord)
    }
}

/// A group of Wall s that make up the x and y axis walls.
#[derive(Debug)]
pub struct Walls {
    x_wall: Wall,
    y_wall: Wall,
    corner: Coord,
}
// constructor function
impl Walls {
    /// Creates a new Walls
    pub fn new(corner: &Coord) -> Self {
        let x_wall: Wall = Wall::new(corner.x - 1, corner.y, X_THRESHOLD);
        let y_wall: Wall = Wall::new(corner.x, corner.y - 1, Y_THRESHOLD);

        Self {
            x_wall,
            y_wall,
            corner: corner.clone(),
        }
    }
}
// functions to tell if multiple coordinates can touch
impl Walls {
    /// Tells whether from the given point, the right spot is directly accessible.
    fn touches_right(&self, center: &Coord) -> bool {
        !self.x_wall.has_stop(center)
    }

    /// Tells whether from the given point the left spot is directly accessible.
    fn touches_left(&self, center: &Coord) -> bool {
        let left_spot_maybe: Option<Coord> = center.left();
        if let Some(left_spot) = left_spot_maybe {
            return self.touches_right(&left_spot);
        }
        true
    }

    /// Tells whether from the given point the above spot is directly accessible.
    fn touches_up(&self, center: &Coord) -> bool {
        !self.y_wall.has_stop(&center)
    }
    /// Tells whether from the given point the below spot is directly accessible.
    fn touches_down(&self, center: &Coord) -> bool {
        let down_spot_maybe: Option<Coord> = center.down();
        if let Some(down_spot) = down_spot_maybe {
            return self.touches_up(&down_spot);
        }
        false
    }
    /// Returns true if two spots are touching and direct travel from one to the other is possible
    fn touches(&self, center: &Coord, other: &Coord) -> bool {
        if center.is_left(&other) {
            return self.touches_left(&center);
        }
        if center.is_right(&other) {
            return self.touches_right(&center);
        }
        if center.is_up(&other) {
            return self.touches_up(&center);
        }
        if center.is_down(&other) {
            return self.touches_down(&center);
        }
        false
    }
    /// Returns a list of all spots that are touching a given Coord that are also directly accessible.
    pub fn connections(&self, center: &Coord) -> Vec<Coord> {
        let adjacents: Vec<Coord> = center.adjacents(&self.corner);
        adjacents
            .iter()
            .filter(|coord| self.touches(center, coord))
            .map(|coord_ref| coord_ref.clone())
            .collect()
    }
}
