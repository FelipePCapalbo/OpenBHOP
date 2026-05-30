use macroquad::prelude::*;
use macroquad::audio::Sound;
use crate::input::InputService;
use crate::player::Player;
use crate::world::Environment;
use crate::hud::Hud;

pub struct GameState {
    pub player: Player,
    pub environment: Environment,
    pub input: InputService,
    pub hud: Hud,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            environment: Environment::new(),
            input: InputService::new(),
            hud: Hud::new(),
        }
    }

    pub fn update(&mut self, delta_time: f32, jump_sound: &Sound) {
        self.input.handle_input(delta_time);
        self.player.update(&self.input, delta_time, jump_sound);
    }

    pub fn draw(&self) {
        clear_background(LIGHTGRAY);

        set_camera(&Camera3D {
            position: self.player.kinematics.position,
            up: self.player.camera.up,
            target: self.player.kinematics.position + self.player.camera.front,
            ..Default::default()
        });

        self.environment.draw();

        set_default_camera();
        
        self.hud.draw(
            self.player.kinematics.position, 
            self.player.kinematics.telemetry.current_speed
        );
    }
}