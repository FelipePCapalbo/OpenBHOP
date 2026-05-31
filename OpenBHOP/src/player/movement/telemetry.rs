use macroquad::prelude::*;

pub struct Telemetry {
    pub current_speed: f32,
}

impl Telemetry {
    pub fn new() -> Self {
        Self { current_speed: 0.0 }
    }

    pub fn update_speed(&mut self, player_speed: Vec3) {
        self.current_speed = player_speed.length();
    }
}