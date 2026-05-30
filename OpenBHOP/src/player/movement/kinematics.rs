use macroquad::prelude::*;

use super::locomotion::Locomotion;
use super::vertical_physics::VerticalPhysics;
use super::jump::JumpController;
use super::telemetry::Telemetry;

pub struct Kinematics {
    pub position: Vec3,
    pub velocity: Vec3,
    pub is_grounded: bool,
    pub locomotion: Locomotion,
    pub vertical_physics: VerticalPhysics,
    pub jump_controller: JumpController,
    pub telemetry: Telemetry,
}

impl Kinematics {
    pub fn new() -> Self {
        Self {
            position: vec3(0.0, 1.0, 0.0),
            velocity: Vec3::ZERO,
            is_grounded: true,
            locomotion: Locomotion::new(),
            vertical_physics: VerticalPhysics::new(),
            jump_controller: JumpController::new(),
            telemetry: Telemetry::new(),
        }
    }

    pub fn apply_movement(&mut self, input_movement: Vec3, camera_front: Vec3, delta_time: f32) {
        let displacement = self.locomotion.apply_horizontal_input(&mut self.position, input_movement, camera_front);
        
        self.vertical_physics.apply_gravity(&mut self.position, &mut self.velocity, &mut self.is_grounded);

        let has_input = input_movement != Vec3::ZERO;
        self.telemetry.update_speed(
            displacement,
            self.velocity.y,
            delta_time,
            self.is_grounded,
            has_input,
        );
    }

    pub fn jump(&mut self) -> bool {
        self.jump_controller.trigger_jump(&mut self.velocity, &mut self.is_grounded)
    }
}