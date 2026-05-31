use macroquad::prelude::*;
use crate::hud::minimap::Minimap;
use crate::hud::effects::HudEffects;

pub struct HudFrame {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl HudFrame {
    pub fn new(width: f32, height: f32) -> Self {
        let padding = crate::config::HUD_FRAME_PADDING;
        Self {
            left: padding,
            right: width - padding,
            top: padding,
            bottom: height - padding,
        }
    }
}

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

    pub fn draw(&self, position: Vec3, player_speed_vec: Vec3, camera_front: Vec3, auto_bhop: bool) {
        let width = screen_width();
        let height = screen_height();
        
        let speed_scalar = player_speed_vec.length();

        // Inicializa o gerenciador encapsulado para cuidar das transformações espaciais
        let effects = HudEffects::new(player_speed_vec, camera_front, width, height);

        // Define a moldura (frame) delimitadora da HUD
        let frame = HudFrame::new(width, height);

        // Renderização completamente sanitizada, aplicando posições puras
        let (cx, cy) = effects.apply(frame.left, frame.top);
        let bhop_status = if auto_bhop { "ON" } else { "OFF" };
        let controls_text = format!(
            "Controls: WASD + MOUSE; SPACE to Jump; TAB to unlock mouse; F to Toggle Auto-BHOP [{}]",
            bhop_status
        );
        draw_text(&controls_text, cx, cy, 22.0, BLACK);

        let (px, py) = effects.apply(frame.left, frame.top + 25.0);
        let pos_text = format!("XYZ: {:.2}, {:.2}, {:.2}", position.x, position.y, position.z);
        draw_text(&pos_text, px, py, 22.0, DARKGRAY);

        let (sx, sy) = effects.apply(frame.left, frame.top + 55.0);
        let speed_text = format!("Speed: {:.1} u/s", speed_scalar);
        let speed_color = if speed_scalar > 0.1 { ORANGE } else { GRAY };
        draw_text(&speed_text, sx, sy, 26.0, speed_color);

        let (fx, fy) = effects.apply(frame.right - 80.0, frame.top);
        let fps_text = format!("FPS: {}", get_fps());
        draw_text(&fps_text, fx, fy, 22.0, GREEN);

        let map_base_x = frame.right - crate::hud::minimap::minimap::MAP_SIZE;
        let map_base_y = frame.top + 30.0;
        let (mx, my) = effects.apply(map_base_x, map_base_y);
        self.minimap.draw(position, mx, my);

        // Contador de células visitadas alinhado ao canto inferior direito do frame (e limite direito do minimapa)
        let visited_text = format!("Visited cells: {}", self.minimap.visited_cells_count());
        let text_dim = measure_text(&visited_text, None, 22, 1.0);
        let cells_base_x = frame.right - text_dim.width;
        let cells_base_y = frame.bottom;
        let (vx, vy) = effects.apply(cells_base_x, cells_base_y);
        draw_text(&visited_text, vx, vy, 22.0, BLACK);
    }
}