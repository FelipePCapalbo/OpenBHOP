use macroquad::prelude::*;
use crate::input::InputService;
use crate::player::{Player, PlayerAction};
use crate::audio::AudioService;
use crate::world::Environment;
use crate::hud::Hud;

pub struct GameState {
    pub player: Player,
    pub environment: Environment,
    pub input: InputService,
    pub hud: Hud,
    pub audio: AudioService,
}

impl GameState {
    pub async fn new() -> Self {
        Self {
            player: Player::new(),
            environment: Environment::new(),
            input: InputService::new(),
            hud: Hud::new(),
            audio: AudioService::load().await,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.input.handle_input(delta_time);
        
        if let Some(action) = self.player.update(&self.input, delta_time) {
            match action {
                PlayerAction::Jumped { speed } => {
                    self.audio.play_jump_sound(speed);
                }
            }
        }

        self.environment.update(self.player.kinematics.position);
        self.hud.update(self.player.kinematics.position);
    }

    pub fn draw(&self) {
        clear_background(LIGHTGRAY);

        set_camera(&Camera3D {
            position: self.player.kinematics.position,
            up: self.player.camera.up,
            target: self.player.kinematics.position + self.player.camera.front,
            ..Default::default()
        });

        self.environment.draw(self.player.kinematics.position, self.player.camera.front);

        set_default_camera();
        
        self.hud.draw(
            self.player.kinematics.position, 
            self.player.kinematics.telemetry.current_speed
        );
    }
}