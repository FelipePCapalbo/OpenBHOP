use macroquad::prelude::*;

pub struct Environment {
}

impl Environment {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self) {
        draw_grid(20, 1.0, BLACK, GRAY);
    }
}