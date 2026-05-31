use macroquad::prelude::*;
use crate::input::InputService;

use super::camera::FirstPersonCamera;
use super::movement::Kinematics;

pub enum PlayerAction {
    Jumped { speed: f32 },
    /// Disparado quando o jogador aterrissa após um pulo.
    Landed,
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

    pub fn update(&mut self, input: &InputService, delta_time: f32) -> Vec<PlayerAction> {
        self.camera.update(input.mouse_delta);

        let mut actions = Vec::new();

        if is_key_pressed(KeyCode::Space) {
            let speed = self.kinematics.telemetry.current_speed;
            if self.kinematics.jump() {
                actions.push(PlayerAction::Jumped { speed });
            }
        }

        // Captura o estado de grounded antes de aplicar a física deste frame
        let was_grounded = self.kinematics.is_grounded;

        self.kinematics.apply_movement(input.movement, self.camera.front, delta_time);

        // Detecta pouso: estava no ar e agora está no chão
        if !was_grounded && self.kinematics.is_grounded {
            actions.push(PlayerAction::Landed);
        }

        actions
    }
}