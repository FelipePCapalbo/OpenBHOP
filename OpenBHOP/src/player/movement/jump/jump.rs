use crate::config::JUMP_FORCE;
use macroquad::prelude::Vec3;

pub struct JumpController {}

impl JumpController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn trigger_jump(&self, speed: &mut Vec3, is_grounded: &mut bool) -> bool {
        if *is_grounded {
            speed.y = JUMP_FORCE;
            *is_grounded = false;
            true
        } else {
            false
        }
    }
}
