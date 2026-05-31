use macroquad::prelude::Vec3;
use crate::hud::effects::{sway, bubble};

pub struct HudEffects {
    sway_offset: (f32, f32),
    bubble_scale: f32,
    center: (f32, f32),
}

impl HudEffects {
    pub fn new(velocity: Vec3, front: Vec3, width: f32, height: f32) -> Self {
        Self {
            sway_offset: sway::calculate_sway(velocity, front),
            bubble_scale: bubble::calculate_bubble_scale(velocity.length()),
            center: (width / 2.0, height / 2.0),
        }
    }

    pub fn apply(&self, x: f32, y: f32) -> (f32, f32) {
        let (bx, by) = bubble::apply_bubble(x, y, self.center, self.bubble_scale);
        sway::apply_sway(bx, by, self.sway_offset)
    }
}
