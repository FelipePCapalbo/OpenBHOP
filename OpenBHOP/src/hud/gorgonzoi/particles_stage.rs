use macroquad::prelude::*;
use macroquad_particles::{Emitter, EmitterConfig, BlendMode, Curve, EmissionShape};

pub struct ParticlesStage {
    emitter: Emitter,
    pub left_eye_emitter: Emitter,
    pub right_eye_emitter: Emitter,
}

impl ParticlesStage {
    pub fn new() -> Self {
        let config = EmitterConfig {
            amount: 0,
            lifetime: 1.0,
            lifetime_randomness: 0.3,
            blend_mode: BlendMode::Additive,
            initial_velocity: 80.0,
            initial_velocity_randomness: 0.5,
            size: 16.0,
            emission_shape: EmissionShape::Sphere { radius: 30.0 },
            size_curve: Some(Curve {
                points: vec![(0.0, 1.0), (1.0, 0.0)],
                ..Default::default()
            }),
            ..Default::default()
        };
        
        let eye_config = EmitterConfig {
            amount: 0,
            lifetime: 0.5,
            lifetime_randomness: 0.2,
            blend_mode: BlendMode::Additive,
            initial_velocity: 40.0,
            initial_velocity_randomness: 0.4,
            size: 8.0,
            emission_shape: EmissionShape::Point, // Emitted directly from the eye position
            size_curve: Some(Curve {
                points: vec![(0.0, 1.0), (1.0, 0.0)],
                ..Default::default()
            }),
            ..Default::default()
        };
        
        Self {
            emitter: Emitter::new(config),
            left_eye_emitter: Emitter::new(eye_config.clone()),
            right_eye_emitter: Emitter::new(eye_config),
        }
    }

    pub fn update_configs(
        &mut self, 
        amount: u32, 
        start_color: Color, 
        mid_color: Color, 
        end_color: Color,
        left_eye_amount: u32,
        right_eye_amount: u32, 
        eye_color: Color,
    ) {
        self.emitter.config.amount = amount;
        if amount > 0 {
            self.emitter.config.colors_curve.start = start_color;
            self.emitter.config.colors_curve.mid = mid_color;
            self.emitter.config.colors_curve.end = end_color;
        }

        self.left_eye_emitter.config.amount = left_eye_amount;
        self.right_eye_emitter.config.amount = right_eye_amount;

        if left_eye_amount > 0 || right_eye_amount > 0 {
            let transparent_eye_color = Color::new(eye_color.r, eye_color.g, eye_color.b, 0.0);
            
            self.left_eye_emitter.config.colors_curve.start = eye_color;
            self.left_eye_emitter.config.colors_curve.mid = eye_color;
            self.left_eye_emitter.config.colors_curve.end = transparent_eye_color;
            
            self.right_eye_emitter.config.colors_curve.start = eye_color;
            self.right_eye_emitter.config.colors_curve.mid = eye_color;
            self.right_eye_emitter.config.colors_curve.end = transparent_eye_color;
        }
    }

    pub fn draw_background(&mut self, position: Vec2) {
        self.emitter.draw(position);
    }
    
    pub fn draw_foreground_eyes(&mut self, left_eye_pos: Vec2, right_eye_pos: Vec2) {
        self.left_eye_emitter.draw(left_eye_pos);
        self.right_eye_emitter.draw(right_eye_pos);
    }
}
