use crate::config::{MAX_SPEED, HUD_BOBBLE_AMPLITUDE};

pub fn calculate_bubble_scale(speed: f32) -> f32 {
    ((speed / MAX_SPEED) - 1.0).max(0.0) * (HUD_BOBBLE_AMPLITUDE * 0.01)
}

pub fn apply_bubble(x: f32, y: f32, center: (f32, f32), scale: f32) -> (f32, f32) {
    (x + (x - center.0) * scale, y + (y - center.1) * scale)
}
