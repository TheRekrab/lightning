
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Coord {
    x: isize,
    y: isize,
}
impl Coord {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn some_adjs(
        &self,
        rng: &mut ThreadRng,
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
