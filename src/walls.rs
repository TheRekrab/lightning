use crate::coord::Coord;
mod coord;

struct SingleAxisWall {
    wall: HashMap<Coord, bool>,
    tolerance: f64,
}
impl SingleAxisWall {
    pub fn new(width: isize, height: isize, tolerance: f64, rng: &mut ThreadRng) -> Self {
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

pub struct Walls {
    x_walls: SingleAxisWall,
    y_walls: SingleAxisWall,
}
impl Walls {
    fn new()
}
