use macroquad::prelude::*;

pub struct World {
}

impl World {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self) {
        draw_grid(20, 1.0, BLACK, GRAY);
    }
}