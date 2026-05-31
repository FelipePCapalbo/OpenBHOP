use crate::config::{JUMP_FORCE, GRAVITY, BHOP_WINDOW_MS};

/// Número de cliques intermediários entre o pulo e o pouso.
const SUBDIVISIONS: usize = 2;

/// Representa um tick pendente no ciclo do metrônomo.
#[derive(Clone, Copy)]
pub struct PendingTick {
    /// Tempo restante até o disparo (em segundos). Decrementado por `update`.
    pub time_remaining: f32,
    /// true = downbeat (pulo / pouso), false = subdivisão intermediária.
    pub is_downbeat: bool,
}

pub struct MetronomeDynamics {
    /// Duração total do voo calculada a partir da física: 2v₀ / |g|
    flight_time: f32,
    /// Janela BHOP convertida para segundos — usada apenas para documentação
    /// do comportamento; o ciclo sempre reinicia no pulo independentemente.
    #[allow(dead_code)]
    bhop_window_s: f32,
    /// Fila de eventos futuros ordenados por `time_remaining` crescente.
    pending: Vec<PendingTick>,
}

impl MetronomeDynamics {
    pub fn new() -> Self {

        let flight_time = 2.0 * JUMP_FORCE / GRAVITY.abs();
        let bhop_window_s = BHOP_WINDOW_MS / 1000.0;

        Self {
            flight_time,
            bhop_window_s,
            pending: Vec::new(),
        }
    }

    /// Chamado no instante do pulo. Reinicia o ciclo e retorna o downbeat
    /// imediato para que o chamador o reproduza agora.
    ///
    /// Sempre reinicia — qualquer ciclo anterior é descartado. Isso garante
    /// que, no BHOP, o beat 1 do novo ciclo coincida exatamente com o pulo.
    pub fn on_jump(&mut self) -> PendingTick {
        self.pending.clear();

        let interval = self.flight_time / (SUBDIVISIONS + 1) as f32;

        // Agenda subdivisões intermediárias
        for i in 1..=SUBDIVISIONS {
            self.pending.push(PendingTick {
                time_remaining: i as f32 * interval,
                is_downbeat: false,
            });
        }

        // Agenda o downbeat do pouso
        self.pending.push(PendingTick {
            time_remaining: self.flight_time,
            is_downbeat: true,
        });

        // Retorna o downbeat imediato (beat 1 do ciclo)
        PendingTick { time_remaining: 0.0, is_downbeat: true }
    }

    /// Chamado ao pousar sem pular (fim de ciclo sem BHOP).
    /// Descarta ticks pendentes — o ciclo termina aqui.
    pub fn on_land(&mut self) {
        self.pending.clear();
    }

    /// Avança o relógio interno em `delta_time` segundos.
    /// Retorna todos os ticks que venceram neste frame (pode ser vazio).
    pub fn update(&mut self, delta_time: f32) -> Vec<PendingTick> {
        if self.pending.is_empty() {
            return Vec::new();
        }

        for tick in self.pending.iter_mut() {
            tick.time_remaining -= delta_time;
        }

        let mut fired = Vec::new();
        self.pending.retain(|tick| {
            if tick.time_remaining <= 0.0 {
                fired.push(*tick);
                false
            } else {
                true
            }
        });

        fired
    }
}
