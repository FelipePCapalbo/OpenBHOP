use super::jump_audio::JumpAudio;
use super::metronome::Metronome;
use super::playlist::{Playlist, PlaylistLoadError};

pub struct AudioService {
    jump: JumpAudio,
    metronome: Metronome,
    pub playlist: Playlist,
}

impl AudioService {
    pub async fn load() -> Result<Self, PlaylistLoadError> {
        Ok(Self {
            jump: JumpAudio::load().await,
            metronome: Metronome::load().await,
            playlist: Playlist::load().await?,
        })
    }

    pub fn play_jump_sound(&self, speed: f32) {
        self.jump.play(speed);
    }

    /// Reinicia o ciclo do metrônomo e toca o downbeat imediatamente.
    /// Deve ser chamado no instante em que o jogador pula.
    pub fn on_jump_metronome(&mut self) {
        self.metronome.on_jump();
    }

    /// Avança o metrônomo e a playlist de música com o volume correspondente à velocidade do player.
    /// Deve ser chamado a cada frame.
    pub fn update(&mut self, player_speed: f32, delta_time: f32) {
        self.metronome.update(delta_time);
        self.playlist.update(player_speed, delta_time);
    }

    /// Finaliza o ciclo do metrônomo ao pousar (pouso sem novo pulo imediato).
    pub fn on_land_metronome(&mut self) {
        self.metronome.on_land();
    }

    /// Retorna o nome da track de música atualmente tocando, se houver.
    pub fn current_track_name(&self) -> Option<&str> {
        self.playlist.current_track_name()
    }
}

