use macroquad::prelude::*;
use crate::hud::minimap::Minimap;
use crate::hud::effects::HudEffects;

pub struct Hud {
    minimap: Minimap,
}

impl Hud {
    pub fn new() -> Self {
        Self { minimap: Minimap::new() }
    }

    pub fn update(&mut self, player_position: Vec3) {
        self.minimap.update(player_position);
    }

    pub fn draw(&self, position: Vec3, player_speed_vec: Vec3, camera_front: Vec3) {
        let width = screen_width();
        let height = screen_height();
        
        let speed_scalar = player_speed_vec.length();

        // Inicializa o gerenciador encapsulado para cuidar das transformações espaciais
        let effects = HudEffects::new(player_speed_vec, camera_front, width, height);

        // Renderização completamente sanitizada, aplicando posições puras
        let (cx, cy) = effects.apply(30.0, 30.0);
        draw_text("Controls: WASD + MOUSE; SPACE to Jump; TAB to unlock mouse", cx, cy, 22.0, BLACK);

        let (px, py) = effects.apply(30.0, 55.0);
        let pos_text = format!("XYZ: {:.2}, {:.2}, {:.2}", position.x, position.y, position.z);
        draw_text(&pos_text, px, py, 22.0, DARKGRAY);

        let (sx, sy) = effects.apply(30.0, 85.0);
        let speed_text = format!("Speed: {:.1} u/s", speed_scalar);
        let speed_color = if speed_scalar > 0.1 { ORANGE } else { GRAY };
        draw_text(&speed_text, sx, sy, 26.0, speed_color);

        let (fx, fy) = effects.apply(width - 110.0, 30.0);
        let fps_text = format!("FPS: {}", get_fps());
        draw_text(&fps_text, fx, fy, 22.0, GREEN);

        let map_base_x = width - 160.0 - 30.0;
        let map_base_y = 60.0;
        let (mx, my) = effects.apply(map_base_x, map_base_y);
        self.minimap.draw(position, mx - map_base_x, my - map_base_y);
    }
}