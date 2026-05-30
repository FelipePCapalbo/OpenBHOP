use crate::config::GRAVITY;
use macroquad::prelude::Vec3;

pub struct VerticalPhysics {}

impl VerticalPhysics {
    pub fn new() -> Self {
        Self {}
    }

    pub fn apply_gravity(&self, position: &mut Vec3, velocity: &mut Vec3, is_grounded: &mut bool) {
        velocity.y += GRAVITY;
        position.y += velocity.y;

        if position.y <= 1.0 {
            position.y = 1.0;
            velocity.y = 0.0;
            *is_grounded = true;
        }
    }
}
