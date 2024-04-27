/// A Bolt is a configuration for the color of a single lightning bolt. Look in constants.rs to find its use
pub struct Bolt {
    pub color: [u8; 3],
}
impl Bolt {
    /// Create a new bolt with the specified rgb colors
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { color: [r, g, b] }
    }
}

// basic colors
impl Bolt {
    /// Creates a Bolt with rgb of (255, 0, 0)
    pub const fn red() -> Self {
        Self { color: [255, 0, 0] }
    }
    /// Creates a Bolt with rgb of (0, 0, 255)
    pub const fn blue() -> Self {
        Self { color: [0, 0, 255] }
    }
    /// Creates a bolt with rgb of (0, 255, 0)
    pub const fn green() -> Self {
        Self { color: [0, 255, 0] }
    }
    /// Creates a bolt with rgb of (255, 255, 255)
    pub const fn white() -> Self {
        Self { color: [255; 3] }
    }
}
