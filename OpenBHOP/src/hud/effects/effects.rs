use macroquad::prelude::Vec3;
use crate::hud::effects::sway;

pub struct HudEffects {
    sway_offset: (f32, f32),
}

impl HudEffects {
    pub fn new(speed: Vec3, front: Vec3) -> Self {
        Self {
            sway_offset: sway::calculate_sway(speed, front),
        }
    }

    pub fn apply(&self, x: f32, y: f32) -> (f32, f32) {
        sway::apply_sway(x, y, self.sway_offset)
    }
}
