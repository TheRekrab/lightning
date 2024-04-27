/// +X right, -X is left. +Y is up, -Y is down. (0, 0) is bottom left corner.
#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// contructor function
impl Coord {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

// adjacency methods
impl Coord {
    pub fn is_left(&self, other: &Self) -> bool {
        if let Some(left) = self.left() {
            return &left == other;
        }
        false
    }

    pub fn is_right(&self, other: &Self) -> bool {
        if let Some(right) = self.right() {
            return &right == other;
        }
        false
    }

    pub fn is_up(&self, other: &Self) -> bool {
        if let Some(up) = self.up() {
            return &up == other;
        }
        false
    }

    pub fn is_down(&self, other: &Self) -> bool {
        if let Some(down) = self.down() {
            return &down == other;
        }
        false
    }

    pub fn left(&self) -> Option<Self> {
        if self.x == 0 {
            return None;
        }

        Some(Self::new(self.x - 1, self.y))
    }

    pub fn right(&self) -> Option<Self> {
        Some(Self::new(self.x + 1, self.y))
    }

    pub fn up(&self) -> Option<Self> {
        Some(Self::new(self.x, self.y + 1))
    }

    pub fn down(&self) -> Option<Self> {
        if self.y == 0 {
            return None;
        }
        Some(Self::new(self.x, self.y - 1))
    }

    fn fits(&self, corner: &Self) -> bool {
        self.x <= corner.x && self.y <= corner.y
    }

    pub fn adjacents(&self, corner: &Self) -> Vec<Self> {
        let possible: Vec<Option<Self>> = vec![self.left(), self.right(), self.up(), self.down()];
        possible
            .iter()
            .filter(|o| o.is_some()) // removes any none
            .map(|o| o.clone().unwrap()) // unwraps all values, because they all contain something
            .filter(|coord| coord.fits(&corner)) // anything out of bounds of the corner is bad
            .collect() // into Vec<Self>
    }
}

#[cfg(test)]
mod coord_tests {
    use super::Coord;

    #[test]
    fn adj_test_1() {
        let corner: Coord = Coord::new(5, 5);

        let c1: Coord = Coord::new(3, 3);
        assert_eq!(
            c1.adjacents(&corner),
            vec![
                Coord::new(2, 3),
                Coord::new(4, 3),
                Coord::new(3, 4),
                Coord::new(3, 2)
            ]
        );

        let c2: Coord = Coord::new(2, 0);
        assert_eq!(
            c2.adjacents(&corner),
            vec![Coord::new(1, 0), Coord::new(3, 0), Coord::new(2, 1)]
        );

        let c3: Coord = Coord::new(0, 2);
        assert_eq!(
            c3.adjacents(&corner),
            vec![Coord::new(1, 2), Coord::new(0, 3), Coord::new(0, 1)]
        );

        let c4: Coord = Coord::new(5, 4);
        assert_eq!(
            c4.adjacents(&corner),
            vec![Coord::new(4, 4), Coord::new(5, 5), Coord::new(5, 3)]
        );

        let c5: Coord = Coord::new(6, 6);
        assert_eq!(c5.adjacents(&corner), vec![])
    }

    #[test]
    fn adj_test_2() {
        let center: Coord = Coord::new(1, 1);

        let left: Coord = Coord::new(0, 1);
        assert!(center.is_left(&left));

        let right: Coord = Coord::new(2, 1);
        assert!(center.is_right(&right));

        let up: Coord = Coord::new(1, 2);
        assert!(center.is_up(&up));

        let down: Coord = Coord::new(1, 0);
        assert!(center.is_down(&down));
    }
}
