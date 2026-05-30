use macroquad::prelude::*;
use std::collections::HashSet;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{Read, Write};

// Constantes para dimensionar a geração e otimização do chão
const CELL_SIZE: f32 = 2.0;
const CHUNK_SIZE: i32 = 8;
const CHUNK_VIEW_RADIUS: i32 = 3;
const PERSISTENCE_FILE: &str = "bin/floor_history.bin";

pub struct FloorGenerator {
    // Conjunto de coordenadas (cx, cz) dos chunks atualmente ativos na memória
    active_chunks: HashSet<(i32, i32)>,
    // Conjunto global persistente de células visitadas pelo jogador
    visited_cells: HashSet<(i32, i32)>,
}

impl FloorGenerator {
    pub fn new() -> Self {
        let mut visited_cells = HashSet::new();

        // Carrega as coordenadas visitadas do histórico binário
        if let Ok(mut file) = File::open(PERSISTENCE_FILE) {
            let mut buffer = [0u8; 8];
            while file.read_exact(&mut buffer).is_ok() {
                let cell_x = i32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                let cell_z = i32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
                visited_cells.insert((cell_x, cell_z));
            }
        }

        Self {
            active_chunks: HashSet::new(),
            visited_cells,
        }
    }

    pub fn update(&mut self, player_position: Vec3) {
        // Coordenadas da célula onde o jogador está pisando no grid
        let cell_x = (player_position.x / CELL_SIZE).floor() as i32;
        let cell_z = (player_position.z / CELL_SIZE).floor() as i32;

        // Se o jogador acabou de pisar numa nova célula, persiste-a em disco
        if self.visited_cells.insert((cell_x, cell_z)) {
            self.persist_cell((cell_x, cell_z));
        }

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

    pub fn draw(&self, player_position: Vec3, player_direction: Vec3) {
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

                    // Define tom escuro para visitado e cinza claro para novo
                    let is_visited = self.visited_cells.contains(&(cell_x, cell_z));
                    let color = if is_visited {
                        Color::new(0.35, 0.35, 0.35, 1.0)
                    } else {
                        Color::new(0.80, 0.80, 0.80, 1.0)
                    };

                    let draw_position = vec3(cell_center_x, 0.0, cell_center_z);
                    let size = vec3(CELL_SIZE - 0.04, 0.01, CELL_SIZE - 0.04);

                    draw_cube(draw_position, size, None, color);
                }
            }
        }
    }

    fn persist_cell(&self, cell: (i32, i32)) {
        // Garante que o diretório 'bin' exista antes de salvar o arquivo
        let _ = create_dir_all("bin");
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(PERSISTENCE_FILE)
        {
            let mut buffer = [0u8; 8];
            buffer[0..4].copy_from_slice(&cell.0.to_le_bytes());
            buffer[4..8].copy_from_slice(&cell.1.to_le_bytes());
            let _ = file.write_all(&buffer);
        }
    }
}
