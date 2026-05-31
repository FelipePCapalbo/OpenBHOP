use macroquad::prelude::Conf;

pub const LOOK_SPEED: f32 = 0.1;
pub const GRAVITY: f32 = -0.01;
pub const JUMP_FORCE: f32 = 0.2;

// Constantes físicas para movimentação no estilo Half-Life (GoldSrc)
pub const MAX_SPEED: f32 = 9.0;             // Velocidade máxima de caminhada por segundo
pub const GROUND_ACCEL: f32 = 10.0;         // Coeficiente de aceleração no chão
pub const AIR_ACCEL: f32 = 100.0;           // Coeficiente de aceleração no ar (facilita air strafing)
pub const FRICTION: f32 = 4.0;              // Coeficiente de atrito no chão para desaceleração
pub const AIR_WISH_SPEED_CAP: f32 = 1.2;    // Limite de projeção de velocidade desejada no ar (impede aceleração infinita para frente)
pub const BHOP_WINDOW_MS: f32 = 200.0;      // Janela de tolerância em milissegundos para manter a velocidade no pulo

pub const CELL_SIZE: f32 = 2.0;
pub const VISITED_CELLS_FILE: &str = "bin/visited_cells.bin";

// Parâmetros do efeito bolha e distorção lateral da HUD
pub const HUD_MAX_SWAY: f32 = 10.0;             // Deslocamento máximo lateral/vertical da HUD (em pixels)
pub const HUD_BOBBLE_AMPLITUDE: f32 = 2.0;      // Amplitude máxima do efeito bolha (em porcentagem de expansão)

pub fn window_config() -> Conf {
    Conf {
        window_title: String::from("OpenBHOP"),
        window_width: 1260,
        window_height: 768,
        ..Default::default()
    }
}