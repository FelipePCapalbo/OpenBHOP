use macroquad::prelude::*;

pub struct Hud {}

impl Hud {
    pub fn new() -> Self { Self {} }

    pub fn draw(&self, position: Vec3, speed: f32) {
        draw_text("Controls: WASD + MOUSE; SPACE to Jump; TAB to unlock mouse", 10.0, 20.0, 22.0, BLACK);
        
        let pos_text = format!("XYZ: {:.2}, {:.2}, {:.2}", position.x, position.y, position.z);
        draw_text(&pos_text, 10.0, 45.0, 22.0, DARKGRAY);

        let speed_text = format!("Speed: {:.1} u/s", speed);
        let speed_color = if speed > 0.1 { ORANGE } else { GRAY };
        draw_text(&speed_text, 10.0, 75.0, 26.0, speed_color);
    }
}