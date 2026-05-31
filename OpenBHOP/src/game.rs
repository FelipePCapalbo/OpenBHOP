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

    pub fn pre_frame_update(&mut self) {
        self.input.update_frame_events();

        if is_key_pressed(KeyCode::F) {
            self.player.auto_bhop = !self.player.auto_bhop;
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.input.handle_input(delta_time);

        // Avança o metrônomo e a playlist de música antes de processar ações do jogador
        self.audio.update(self.player.kinematics.telemetry.current_speed, delta_time);
        
        for action in self.player.update(&self.input, delta_time) {
            match action {
                PlayerAction::Jumped { speed } => {
                    self.audio.play_jump_sound(speed);
                    self.audio.on_jump_metronome();
                }
                PlayerAction::Landed => {
                    // Pouso sem BHOP: finaliza o ciclo do metrônomo.
                    // Se o jogador pular dentro da janela BHOP, on_jump reiniciará o ciclo.
                    self.audio.on_land_metronome();
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
            self.player.kinematics.speed,
            self.player.camera.front,
            self.player.auto_bhop,
            self.audio.current_track_name(),
            self.player.kinematics.bhop_combo_count,
        );
    }
}