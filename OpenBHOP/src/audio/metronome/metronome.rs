use super::metronome_dynamics::MetronomeDynamics;
use super::metronome_sound::MetronomeSound;

pub struct Metronome {
    dynamics: MetronomeDynamics,
    sound: MetronomeSound,
}

impl Metronome {
    pub async fn load() -> Self {
        Self {
            dynamics: MetronomeDynamics::new(),
            sound: MetronomeSound::load().await,
        }
    }

    /// Reinicia o ciclo e toca o downbeat imediatamente.
    pub fn on_jump(&mut self) {
        let downbeat = self.dynamics.on_jump();
        self.sound.play(downbeat.is_downbeat);
    }

    /// Avança o metrônomo e dispara os ticks que venceram neste frame.
    pub fn update(&mut self, delta_time: f32) {
        for tick in self.dynamics.update(delta_time) {
            self.sound.play(tick.is_downbeat);
        }
    }

    /// Finaliza o ciclo atual sem pulo (pouso sem BHOP).
    pub fn on_land(&mut self) {
        self.dynamics.on_land();
    }
}
