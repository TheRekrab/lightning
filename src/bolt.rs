pub struct Bolt {
    pub color: [u8; 3],
}
impl Bolt {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { color: [r, g, b] }
    }
}

// basic colors
impl Bolt {
    pub const fn red() -> Self {
        Self { color: [255, 0, 0] }
    }
    pub const fn blue() -> Self {
        Self { color: [0, 0, 255] }
    }
    pub const fn green() -> Self {
        Self { color: [0, 255, 0] }
    }
    pub const fn white() -> Self {
        Self { color: [255; 3] }
    }
}
