use super::jump_audio::JumpAudio;
use super::metronome::Metronome;

pub struct AudioService {
    jump: JumpAudio,
    metronome: Metronome,
}

impl AudioService {
    pub async fn load() -> Self {
        Self {
            jump: JumpAudio::load().await,
            metronome: Metronome::load().await,
        }
    }

    pub fn play_jump_sound(&self, speed: f32) {
        self.jump.play(speed);
    }

    /// Reinicia o ciclo do metrônomo e toca o downbeat imediatamente.
    /// Deve ser chamado no instante em que o jogador pula.
    pub fn on_jump_metronome(&mut self) {
        self.metronome.on_jump();
    }

    /// Avança o metrônomo em `delta_time` segundos, disparando cliques agendados.
    /// Deve ser chamado a cada frame.
    pub fn update_metronome(&mut self, delta_time: f32) {
        self.metronome.update(delta_time);
    }

    /// Finaliza o ciclo do metrônomo ao pousar (pouso sem novo pulo imediato).
    pub fn on_land_metronome(&mut self) {
        self.metronome.on_land();
    }
}
