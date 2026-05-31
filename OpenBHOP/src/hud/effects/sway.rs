use macroquad::prelude::*;
use crate::config::{MAX_SPEED, HUD_MAX_SWAY};

pub fn calculate_sway(speed: Vec3, front: Vec3) -> (f32, f32) {
    let right = front.cross(vec3(0.0, 1.0, 0.0)).normalize_or_zero();
    let flat_front = vec3(front.x, 0.0, front.z).normalize_or_zero();

    // Projeta a velocidade nos eixos locais da câmera
    let local_x = speed.dot(right);
    let local_y = speed.y + speed.dot(flat_front); // Soma inércia de frente/trás com pulo/queda

    // Mapeia linearmente a velocidade local para o deslocamento máximo
    let sway_x = (-local_x / MAX_SPEED).clamp(-1.0, 1.0) * HUD_MAX_SWAY;
    let sway_y = (local_y / MAX_SPEED).clamp(-1.0, 1.0) * HUD_MAX_SWAY;

    (sway_x, sway_y)
}

pub fn apply_sway(x: f32, y: f32, offset: (f32, f32)) -> (f32, f32) {
    (x + offset.0, y + offset.1)
}
