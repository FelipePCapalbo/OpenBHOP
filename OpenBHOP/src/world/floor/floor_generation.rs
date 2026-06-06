use macroquad::prelude::*;
use std::collections::HashSet;
use crate::config::CELL_SIZE;
use super::{VisitTracker, get_visited_color};

// Constantes para dimensionar a geração e otimização do chão
const CHUNK_SIZE: i32 = 8;
const CHUNK_VIEW_RADIUS: i32 = 5;

pub struct FloorGenerator {
    // Conjunto de coordenadas (cx, cz) dos chunks atualmente ativos na memória
    active_chunks: HashSet<(i32, i32)>,
}

impl FloorGenerator {
    pub fn new() -> Self {
        Self {
            active_chunks: HashSet::new(),
        }
    }

    pub fn update(&mut self, player_position: Vec3) {
        // Coordenadas da célula onde o jogador está pisando no grid
        let cell_x = (player_position.x / CELL_SIZE).floor() as i32;
        let cell_z = (player_position.z / CELL_SIZE).floor() as i32;

        // Chunk atual do jogador usando divisão euclidiana (evita falhas com negativos)
        let player_chunk_x = cell_x.div_euclid(CHUNK_SIZE);
        let player_chunk_z = cell_z.div_euclid(CHUNK_SIZE);

        // Atualiza os chunks ativos baseando-se unicamente no raio de visão circular
        self.active_chunks.clear();
        for dx in -CHUNK_VIEW_RADIUS..=CHUNK_VIEW_RADIUS {
            for dz in -CHUNK_VIEW_RADIUS..=CHUNK_VIEW_RADIUS {
                if dx * dx + dz * dz <= CHUNK_VIEW_RADIUS * CHUNK_VIEW_RADIUS {
                    self.active_chunks.insert((player_chunk_x + dx, player_chunk_z + dz));
                }
            }
        }
    }

    // Retorna se um objeto em determinada posicao tridimensional global esta dentro de um chunk ativo na memoria
    pub fn is_position_visible(&self, position: Vec3) -> bool {
        let cell_x = (position.x / CELL_SIZE).floor() as i32;
        let cell_z = (position.z / CELL_SIZE).floor() as i32;

        let chunk_x = cell_x.div_euclid(CHUNK_SIZE);
        let chunk_z = cell_z.div_euclid(CHUNK_SIZE);

        self.active_chunks.contains(&(chunk_x, chunk_z))
    }

    pub fn draw(&self, player_position: Vec3, player_direction: Vec3, visit_tracker: &VisitTracker) {
        let view_direction_h = vec2(player_direction.x, player_direction.z).normalize_or_zero();
        let player_pos_h = vec2(player_position.x, player_position.z);
        let near_distance_threshold_sq = (CELL_SIZE * 1.5).powi(2);

        // Renderiza cada célula pertencente aos chunks ativos na memória
        for &(cx, cz) in &self.active_chunks {
            for local_x in 0..CHUNK_SIZE {
                for local_z in 0..CHUNK_SIZE {
                    let cell_x = cx * CHUNK_SIZE + local_x;
                    let cell_z = cz * CHUNK_SIZE + local_z;

                    let cell_center_x = cell_x as f32 * CELL_SIZE + CELL_SIZE / 2.0;
                    let cell_center_z = cell_z as f32 * CELL_SIZE + CELL_SIZE / 2.0;

                    let cell_pos_h = vec2(cell_center_x, cell_center_z);
                    let to_cell_h = cell_pos_h - player_pos_h;
                    let distance_sq = to_cell_h.length_squared();

                    // Culling por FOV em células distantes (poupa renderização de costas para o jogador)
                    if distance_sq > near_distance_threshold_sq {
                        let dot_product = to_cell_h.normalize().dot(view_direction_h);
                        if dot_product < 0.15 {
                            continue;
                        }
                    }

                    // Define a cor obtendo a contagem do VisitTracker
                    let count = visit_tracker.get_count((cell_x, cell_z));
                    let color = get_visited_color(count);

                    let draw_position = vec3(cell_center_x, 0.0, cell_center_z);
                    let size = vec3(CELL_SIZE - 0.04, 0.01, CELL_SIZE - 0.04);

                    draw_cube(draw_position, size, None, color);
                }
            }
        }
    }
}
