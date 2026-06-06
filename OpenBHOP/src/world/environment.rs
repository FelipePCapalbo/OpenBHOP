use macroquad::prelude::*;
use crate::world::floor::{FloorGenerator, VisitTracker};
use crate::world::Gorgonzoi;

pub struct Environment {
    pub floor_generator: FloorGenerator,
    pub gorgonzois: Vec<Gorgonzoi>,
    pub visit_tracker: VisitTracker,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            floor_generator: FloorGenerator::new(),
            gorgonzois: vec![
                // Instancia o Gorgonzoi principal um pouco à frente do ponto de spawn do jogador (0.0, 1.0, 0.0)
                Gorgonzoi::new(vec3(0.0, 0.35, -4.0)),
            ],
            visit_tracker: VisitTracker::new(),
        }
    }

    pub fn update(&mut self, player_position: Vec3) {
        self.floor_generator.update(player_position);
        self.visit_tracker.update(player_position);

        // Faz com que os Gorgonzois flutuem e rotacionem lentamente para um efeito visual refinado
        let time = get_time() as f32;
        for gorgonzoi in &mut self.gorgonzois {
            gorgonzoi.rotation.y = time * 0.8;
            gorgonzoi.position.y = 0.35 + (time * 1.5).sin() * 0.04;
        }
    }

    pub fn draw(&self, player_position: Vec3, player_direction: Vec3) {
        self.floor_generator.draw(player_position, player_direction, &self.visit_tracker);
        for gorgonzoi in &self.gorgonzois {
            if self.floor_generator.is_position_visible(gorgonzoi.position) {
                gorgonzoi.draw();
            }
        }
    }
}