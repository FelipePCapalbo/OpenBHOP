use macroquad::prelude::*;
use crate::world::floor::FloorGenerator;

pub struct Environment {
    pub floor_generator: FloorGenerator,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            floor_generator: FloorGenerator::new(),
        }
    }

    pub fn update(&mut self, player_position: Vec3) {
        self.floor_generator.update(player_position);
    }

    pub fn draw(&self, player_position: Vec3, player_direction: Vec3) {
        self.floor_generator.draw(player_position, player_direction);
    }
}