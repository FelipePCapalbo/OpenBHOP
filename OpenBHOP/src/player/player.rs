use macroquad::prelude::*;
use macroquad::audio::{play_sound_once, Sound};
use crate::input::InputService;

use super::camera::FirstPersonCamera;
use super::movement::Kinematics;

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

    pub fn update(&mut self, input: &InputService, delta_time: f32, jump_sound: &Sound) {
        self.camera.update(input.mouse_delta);

        if is_key_pressed(KeyCode::Space) {
            if self.kinematics.jump() {
                play_sound_once(jump_sound);
            }
        }

        self.kinematics.apply_movement(input.movement, self.camera.front, delta_time);
    }
}