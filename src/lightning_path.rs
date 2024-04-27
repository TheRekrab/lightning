use crate::coord::Coord;

/// A wrapper data type for a Vec<Coord> so that it's easier to read.
pub struct LightningPath {
    pub path: Vec<Coord>,
}
impl LightningPath {
    /// Creates a new LightningPath that stores the given path
    pub fn new(path: Vec<Coord>) -> Self {
        Self { path }
    }
}
