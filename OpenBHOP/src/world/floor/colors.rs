use macroquad::prelude::Color;
use std::sync::atomic::{AtomicU8, AtomicU32, Ordering};

pub static COLOR_MODE: AtomicU8 = AtomicU8::new(0); // 0 = Normal, 1 = Inverted, 2 = SpeedBased
pub static CURRENT_SPEED: AtomicU32 = AtomicU32::new(0); // Velocidade do jogador * 1000

pub fn get_visited_color(count: u32) -> Color {
    if count == 0 {
        return Color::new(1.0, 1.0, 1.0, 1.0); // Não visitado: branco
    }

    let mode = COLOR_MODE.load(Ordering::Relaxed);

    match mode {
        1 => {
            if count >= 31 {
                return Color::new(1.0, 1.0, 1.0, 1.0); // Acima de 30 visitas (30 incrementos adicionais): branco
            }
            let steps = count - 1;
            let b = (steps as f32 * 0.1).min(1.0);
            let g = (((steps as f32 - 10.0) * 0.1).max(0.0)).min(1.0);
            let r = (((steps as f32 - 20.0) * 0.1).max(0.0)).min(1.0);
            Color::new(r, g, b, 1.0)
        }
        2 => {
            // Terceiro modo: varia conforme a velocidade do usuário
            let speed = CURRENT_SPEED.load(Ordering::Relaxed) as f32 / 1000.0;
            let steps = count - 1;

            if steps == 0 {
                return Color::new(0.0, 0.0, 0.0, 1.0); // Imediatamente visitado: preto
            }

            // O hue (matiz) progride com os steps e desloca-se suavemente com a velocidade
            let hue = (steps as f32 * 0.04 + speed * 0.015) % 1.0;

            // Saturação e luminosidade aumentam progressivamente a partir do preto nos primeiros níveis,
            // depois estabilizam em valores vibrantes enquanto o hue continua mudando infinitamente.
            let saturation = (steps as f32 * 0.1).min(0.85);
            let lightness = (steps as f32 * 0.05).min(0.5);

            hsl_to_rgb(hue, saturation, lightness)
        }
        _ => {
            if count >= 31 {
                return Color::new(1.0, 1.0, 1.0, 1.0); // Acima de 30 visitas (30 incrementos adicionais): branco
            }
            let steps = count - 1;
            let r = (steps as f32 * 0.1).min(1.0);
            let g = (((steps as f32 - 10.0) * 0.1).max(0.0)).min(1.0);
            let b = (((steps as f32 - 20.0) * 0.1).max(0.0)).min(1.0);
            Color::new(r, g, b, 1.0)
        }
    }
}

pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> Color {
    if s == 0.0 {
        Color::new(l, l, l, 1.0)
    } else {
        let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
        let p = 2.0 * l - q;
        let r = hue_to_rgb(p, q, h + 1.0 / 3.0);
        let g = hue_to_rgb(p, q, h);
        let b = hue_to_rgb(p, q, h - 1.0 / 3.0);
        Color::new(r, g, b, 1.0)
    }
}

fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 { t += 1.0; }
    if t > 1.0 { t -= 1.0; }
    if t < 1.0 / 6.0 { return p + (q - p) * 6.0 * t; }
    if t < 1.0 / 2.0 { return q; }
    if t < 2.0 / 3.0 { return p + (q - p) * (2.0 / 3.0 - t) * 6.0; }
    p
}
