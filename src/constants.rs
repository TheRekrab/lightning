use crate::bolt::Bolt;
pub const WIDTH: usize = 300;
pub const HEIGHT: usize = 300;

pub const X_THRESHOLD: f64 = 0.05;
pub const Y_THRESHOLD: f64 = 0.65;

pub fn bolts() -> Vec<Bolt> {
    vec![
        // Bolt::blue(),
        // Bolt::red(),
        // Bolt::green(),
        // Bolt::white(),
        Bolt::new(55, 119, 247),
    ]
}
