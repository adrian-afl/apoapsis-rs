use glam::{DQuat, DVec3};

#[derive(Debug, Clone)]
pub struct QuatDirections {
    pub up: DVec3,
    pub down: DVec3,
    pub left: DVec3,
    pub right: DVec3,
    pub forwards: DVec3,
    pub backwards: DVec3,
}

static FORWARDS: DVec3 = DVec3::new(0.0, 0.0, -1.0);
static BACKWARDS: DVec3 = DVec3::new(0.0, 0.0, 1.0);
static UP: DVec3 = DVec3::new(0.0, 1.0, 0.0);
static DOWN: DVec3 = DVec3::new(0.0, -1.0, 0.0);
static LEFT: DVec3 = DVec3::new(-1.0, 0.0, 0.0);
static RIGHT: DVec3 = DVec3::new(1.0, 0.0, 0.0);

pub fn get_quat_directions(quat: DQuat) -> QuatDirections {
    QuatDirections {
        forwards: quat * FORWARDS,
        backwards: quat * BACKWARDS,
        up: quat * UP,
        down: quat * DOWN,
        left: quat * LEFT,
        right: quat * RIGHT,
    }
}
