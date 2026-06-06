use macroquad::prelude::*;
use crate::config::CELL_SIZE;
use crate::world::floor::{VisitTracker, get_visited_color};

pub const MAP_SIZE: f32 = 160.0;
const MAP_SCALE: f32 = 4.0; 

pub struct Minimap;

impl Minimap {
    pub fn new() -> Self {
        Self
    }

    pub fn draw(&self, player_position: Vec3, map_x: f32, map_y: f32, visit_tracker: &VisitTracker) {
        let center_x = map_x + MAP_SIZE / 2.0;
        let center_y = map_y + MAP_SIZE / 2.0;

        // Desenha o fundo do minimapa com opacidade para permitir enxergar o jogo atrás
        draw_rectangle(map_x, map_y, MAP_SIZE, MAP_SIZE, Color::new(0.08, 0.08, 0.08, 0.85));

        // Limites para clipping manual para que as células não extrapolem as bordas do minimapa
        let min_x = map_x;
        let max_x = map_x + MAP_SIZE;
        let min_y = map_y;
        let max_y = map_y + MAP_SIZE;

        let cell_screen_size = CELL_SIZE * MAP_SCALE;

        // Desenha cada célula visitada com a cor da escala de progresso obtida do VisitTracker
        for (&(cx, cz), &count) in visit_tracker.all_visited_cells() {
            let cell_center_x = cx as f32 * CELL_SIZE + CELL_SIZE / 2.0;
            let cell_center_z = cz as f32 * CELL_SIZE + CELL_SIZE / 2.0;

            // Diferença de posição entre a célula e o jogador
            let offset_x = cell_center_x - player_position.x;
            let offset_z = cell_center_z - player_position.z;

            // Coordenadas na tela centralizadas no minimapa
            let draw_x = center_x + offset_x * MAP_SCALE;
            let draw_y = center_y + offset_z * MAP_SCALE;

            // Retângulo de desenho original da célula
            let rect_min_x = draw_x - cell_screen_size / 2.0;
            let rect_max_x = draw_x + cell_screen_size / 2.0;
            let rect_min_y = draw_y - cell_screen_size / 2.0;
            let rect_max_y = draw_y + cell_screen_size / 2.0;

            // Realiza o clipping do retângulo limitando-o à área visível do minimapa
            let clip_min_x = rect_min_x.max(min_x);
            let clip_max_x = rect_max_x.min(max_x);
            let clip_min_y = rect_min_y.max(min_y);
            let clip_max_y = rect_max_y.min(max_y);

            // Se restar uma área visível após o clipping, renderiza o retângulo
            if clip_min_x < clip_max_x && clip_min_y < clip_max_y {
                draw_rectangle(
                    clip_min_x,
                    clip_min_y,
                    clip_max_x - clip_min_x,
                    clip_max_y - clip_min_y,
                    get_visited_color(count),
                );
            }
        }

        // Desenha o jogador como um marcador circular vermelho estático no centro
        draw_circle(center_x, center_y, 4.0, RED);

        // Desenha a borda fina branca ao redor do minimapa
        draw_rectangle_lines(map_x, map_y, MAP_SIZE, MAP_SIZE, 1.5, WHITE);
    }
}
