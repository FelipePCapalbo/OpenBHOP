use macroquad::prelude::Conf;

// ─── Parâmetros ajustáveis (1.0 = padrão) ───────────────────────────────────
pub const SENSITIVITY: f32 = 1.0;       // Sensibilidade do mouse (ex: 0.5 = metade, 2.0 = dobro)
pub const GRAVITY_SCALE: f32 = 0.8;     // Intensidade da gravidade
pub const JUMP_SCALE: f32 = 1.0;        // Força do pulo

// ─── Valores base internos ──────────────────────────────────────────────────
// Fator de conversão entre pixels de delta do mouse e radianos de rotação da câmera.
// Empiricamente tunado: com ~1333 pixels de largura de tela e DPI padrão, uma volta
// de 360° exige ~2500px de movimento — sensação equivalente ao padrão de jogos FPS.
const BASE_LOOK_SPEED: f32 = 0.00075;

// Gravidade em unidades/s². ~3.7× a gravidade terrestre (9.8 m/s²), tunado para
// replicar o feeling do motor GoldSrc (Half-Life/CS 1.6), onde a gravidade padrão
// é 800 ups² na escala de unidades daquele engine (1 unidade ≈ 1.9 cm).
const BASE_GRAVITY: f32 = -36.0;

// Velocidade vertical inicial do pulo em unidades/s. Com BASE_GRAVITY = -36,
// a altura máxima é v²/(2g) = 144/72 ≈ 2.0 unidades e o tempo de queda é
// 2v/g ≈ 0.67s — proporcional ao pulo do GoldSrc e adequado para BHOP.
const BASE_JUMP_FORCE: f32 = 12.0;

// ─── Constantes derivadas (usadas pela engine) ──────────────────────────────
pub const LOOK_SPEED: f32 = BASE_LOOK_SPEED * SENSITIVITY;
pub const GRAVITY: f32 = BASE_GRAVITY * GRAVITY_SCALE;
pub const JUMP_FORCE: f32 = BASE_JUMP_FORCE * JUMP_SCALE;

pub const TICK_RATE: f32 = 128.0;
pub const TICK_DELTA: f32 = 1.0 / TICK_RATE;

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

// ─── Volume dos sons  ───────────────────────────────────────────────────────
pub const JUMP_SOUND_VOLUME: f32 = 0.30;         // Volume do som de pulo (escala aplicada ao volume calculado por pitch)
pub const METRONOME_VOLUME: f32 = 0.10;         // Volume do metrônomo relativo aos demais sons (10% por padrão)


pub fn window_config() -> Conf {
    Conf {
        window_title: String::from("OpenBHOP"),
        window_width: 1260,
        window_height: 768,
        ..Default::default()
    }
}