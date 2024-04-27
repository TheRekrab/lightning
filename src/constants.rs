use crate::bolt::Bolt;
pub const WIDTH: usize = 300;
pub const HEIGHT: usize = 300;

pub const X_THRESHOLD: f64 = 0.05;
pub const Y_THRESHOLD: f64 = 0.65;

pub const BOLTS: [Bolt; 4] = [Bolt::blue(), Bolt::red(), Bolt::green(), Bolt::white()];
