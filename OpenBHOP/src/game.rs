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

        if is_key_pressed(KeyCode::E) {
            let player_pos = self.player.kinematics.position;
            let is_near = self.environment.gorgonzois.iter().any(|g| {
                (g.position - player_pos).length() < 2.5
            });
            if is_near {
                let current = crate::world::floor::colors::COLOR_MODE.load(std::sync::atomic::Ordering::Relaxed);
                crate::world::floor::colors::COLOR_MODE.store((current + 1) % 3, std::sync::atomic::Ordering::Relaxed);
            }
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

        let speed = self.player.kinematics.speed.length();
        crate::world::floor::colors::CURRENT_SPEED.store(
            (speed * 1000.0) as u32,
            std::sync::atomic::Ordering::Relaxed,
        );
    }

    pub fn draw(&self) {
        let mode = crate::world::floor::colors::COLOR_MODE.load(std::sync::atomic::Ordering::Relaxed);
        match mode {
            1 => clear_background(WHITE),
            2 => {
                let speed = crate::world::floor::colors::CURRENT_SPEED.load(std::sync::atomic::Ordering::Relaxed) as f32 / 1000.0;
                // O matiz (hue) do céu varia continuamente com a velocidade
                let sky_hue = (speed * 0.03) % 1.0;
                // A luminosidade aumenta com a velocidade (começando escura e indo até um limite agradável de 0.25)
                let lightness = (speed / 12.0).min(1.0) * 0.25;
                
                let sky_color = crate::world::floor::colors::hsl_to_rgb(sky_hue, 0.8, lightness);
                clear_background(sky_color);
            }
            _ => clear_background(BLACK),
        }

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
            &self.environment.visit_tracker,
        );
    }
}