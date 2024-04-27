use crate::coord::Coord;

pub struct LightningPath {
    pub path: Vec<Coord>,
}
impl LightningPath {
    pub fn new(path: Vec<Coord>) -> Self {
        Self { path }
    }
}
