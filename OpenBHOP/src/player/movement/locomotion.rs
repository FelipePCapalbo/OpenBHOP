use macroquad::prelude::*;
use crate::config::MOVE_SPEED;

pub struct Locomotion {}

impl Locomotion {
    pub fn new() -> Self {
        Self {}
    }

    pub fn apply_horizontal_input(&self, position: &mut Vec3, input_movement: Vec3, camera_front: Vec3) -> Vec3 {
        let forward_dir = vec3(camera_front.x, 0.0, camera_front.z).normalize_or_zero();
        let world_up = vec3(0.0, 1.0, 0.0);
        let right_dir = camera_front.cross(world_up).normalize();

        let mut displacement = Vec3::ZERO;
        displacement += forward_dir * input_movement.z * MOVE_SPEED;
        displacement += right_dir * input_movement.x * MOVE_SPEED;

        position.x += displacement.x;
        position.z += displacement.z;

        displacement
    }
}