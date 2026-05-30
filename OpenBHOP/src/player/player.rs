use macroquad::prelude::*;
use crate::input::InputService;

use super::camera::FirstPersonCamera;
use super::movement::Kinematics;

pub enum PlayerAction {
    Jumped { speed: f32 },
}

pub struct Player {
    pub camera: FirstPersonCamera,
    pub kinematics: Kinematics,
}

impl Player {
    pub fn new() -> Self {
        Self {
            camera: FirstPersonCamera::new(),
            kinematics: Kinematics::new(),
        }
    }

    pub fn update(&mut self, input: &InputService, delta_time: f32) -> Option<PlayerAction> {
        self.camera.update(input.mouse_delta);

        let mut action = None;

        if is_key_pressed(KeyCode::Space) {
            let speed = self.kinematics.telemetry.current_speed;
            if self.kinematics.jump() {
                action = Some(PlayerAction::Jumped { speed });
            }
        }

        self.kinematics.apply_movement(input.movement, self.camera.front, delta_time);

        action
    }
}