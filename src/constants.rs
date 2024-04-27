use crate::bolt::Bolt;

/// The width, in pixels, of the image
pub const WIDTH: usize = 200;

/// The height, in pixels, of the image
pub const HEIGHT: usize = 400;

/// The chance that a barrier will spawn at a specific spot that stops travel in the x direction.
pub const X_THRESHOLD: f64 = 0.05;

/// The chance that a barrier will spawn at a specific spot that stops travel in the y direction.
pub const Y_THRESHOLD: f64 = 0.75;

/// A functions that returns a list of all bolts to use in the image. Each bolt will have its own lightning and maze
pub fn bolts() -> Vec<Bolt> {
    vec![
        // Bolt::blue(),
        // Bolt::red(),
        // Bolt::green(),
        // Bolt::white(),
        Bolt::new(55, 119, 247),
        Bolt::new(67, 49, 252),
        Bolt::new(49, 228, 252),
    ]
}
