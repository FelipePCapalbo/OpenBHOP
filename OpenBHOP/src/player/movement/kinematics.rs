use macroquad::prelude::*;

use super::locomotion::Locomotion;
use super::gravity::VerticalPhysics;
use super::telemetry::Telemetry;

pub struct Kinematics {
    pub position: Vec3,
    pub locomotion: Locomotion,
    pub vertical_physics: VerticalPhysics,
    pub telemetry: Telemetry,
}

impl Kinematics {
    pub fn new() -> Self {
        Self {
            position: vec3(0.0, 1.0, 0.0),
            locomotion: Locomotion::new(0.0, 0.0),
            vertical_physics: VerticalPhysics::new(1.0),
            telemetry: Telemetry::new(),
        }
    }

    pub fn apply_movement(&mut self, input_movement: Vec3, camera_front: Vec3, delta_time: f32) {
        let displacement = self.locomotion.compute_displacement(input_movement, camera_front);
        self.vertical_physics.apply_gravity();

        self.position = vec3(
            self.locomotion.position_xz.x,
            self.vertical_physics.position_y,
            self.locomotion.position_xz.y,
        );

        let has_input = input_movement != Vec3::ZERO;
        self.telemetry.measure_speed(
            displacement,
            self.vertical_physics.speed_y,
            delta_time,
            self.vertical_physics.is_grounded,
            has_input,
        );
    }

    pub fn jump(&mut self) {
        self.vertical_physics.trigger_jump();
    }
}