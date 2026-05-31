use crate::config::GRAVITY;
use macroquad::prelude::Vec3;

pub struct VerticalPhysics {}

impl VerticalPhysics {
    pub fn new() -> Self {
        Self {}
    }

    pub fn apply_gravity(&self, position: &mut Vec3, speed: &mut Vec3, is_grounded: &mut bool, delta_time: f32) {
        speed.y += GRAVITY * delta_time;
        position.y += speed.y * delta_time;

        if position.y <= 1.0 {
            position.y = 1.0;
            speed.y = 0.0;
            *is_grounded = true;
        }
    }
}
