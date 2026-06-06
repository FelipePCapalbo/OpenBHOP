use std::collections::HashMap;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{Read, Write};
use crate::config::{CELL_SIZE, VISITED_CELLS_FILE};
use macroquad::prelude::Vec3;

pub struct VisitTracker {
    visited_cells: HashMap<(i32, i32), u32>,
    last_cell: Option<(i32, i32)>,
}

impl VisitTracker {
    pub fn new() -> Self {
        let mut visited_cells = HashMap::new();

        // Carrega as coordenadas visitadas do histórico binário.
        // A contagem é o número total de ocorrências de cada coordenada no arquivo.
        if let Ok(mut file) = File::open(VISITED_CELLS_FILE) {
            let mut buffer = [0u8; 8];
            while file.read_exact(&mut buffer).is_ok() {
                let cell_x = i32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                let cell_z = i32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
                *visited_cells.entry((cell_x, cell_z)).or_insert(0) += 1;
            }
        }

        Self {
            visited_cells,
            last_cell: None,
        }
    }

    pub fn update(&mut self, player_position: Vec3) {
        let cell_x = (player_position.x / CELL_SIZE).floor() as i32;
        let cell_z = (player_position.z / CELL_SIZE).floor() as i32;
        let current_cell = (cell_x, cell_z);

        // Se o jogador mudou de célula, incrementa a contagem e persiste a coordenada
        if self.last_cell != Some(current_cell) {
            *self.visited_cells.entry(current_cell).or_insert(0) += 1;
            self.persist_cell(current_cell);
            self.last_cell = Some(current_cell);
        }
    }

    pub fn get_count(&self, cell: (i32, i32)) -> u32 {
        *self.visited_cells.get(&cell).unwrap_or(&0)
    }

    pub fn visited_cells_count(&self) -> usize {
        self.visited_cells.len()
    }

    pub fn all_visited_cells(&self) -> &HashMap<(i32, i32), u32> {
        &self.visited_cells
    }

    fn persist_cell(&self, cell: (i32, i32)) {
        let _ = create_dir_all("bin");
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(VISITED_CELLS_FILE)
        {
            let mut buffer = [0u8; 8];
            buffer[0..4].copy_from_slice(&cell.0.to_le_bytes());
            buffer[4..8].copy_from_slice(&cell.1.to_le_bytes());
            let _ = file.write_all(&buffer);
        }
    }
}
