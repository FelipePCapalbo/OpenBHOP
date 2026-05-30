use macroquad::prelude::*;

pub struct Telemetry {
    pub current_speed: f32,
}

impl Telemetry {
    pub fn new() -> Self {
        Self { current_speed: 0.0 }
    }

    pub fn update_speed(&mut self, displacement: Vec3, velocity_y: f32, delta_time: f32, is_grounded: bool, has_input: bool) {
        let horizontal_speed = displacement.length() / delta_time;
        
        if is_grounded {
            self.current_speed = if !has_input { 0.0 } else { horizontal_speed };
        } else {
            let vertical_speed = velocity_y / delta_time;
            self.current_speed = vec2(horizontal_speed, vertical_speed).length();
        }
    }
}