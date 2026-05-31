use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct ComboLevelConfig {
    pub jumps_required: u32,
    pub rotation_speed: f32,
    pub particle_amount: u32,
    pub particle_color_start: Color,
    pub particle_color_mid: Color,
    pub particle_color_end: Color,
    
    pub cheese_color: Color,
    pub mold_color_dark: Color,
    pub mold_color_light: Color,
    
    pub eye_sclera_color: Color,
    pub eye_iris_color: Color,
    pub eye_pupil_color: Color,
    
    pub eye_radius_base: f32,
    pub eye_oscillation_speed: f32,
    
    pub eye_particle_amount: u32,
    pub eye_particle_color: Color,
}

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    Color::new(
        a.r + (b.r - a.r) * t,
        a.g + (b.g - a.g) * t,
        a.b + (b.b - a.b) * t,
        a.a + (b.a - a.a) * t,
    )
}

impl ComboLevelConfig {
    pub fn from_jumps(jumps: u32) -> Self {
        // Linear progress interpolation factor, capped at 200 jumps
        let t = (jumps as f32 / 200.0).min(1.0);
        
        // Initial colors (0 jumps)
        let base_cheese = Color::new(0.96, 0.93, 0.74, 1.0);
        let base_mold_dark = Color::new(0.24, 0.38, 0.33, 1.0);
        let base_mold_light = Color::new(0.42, 0.55, 0.48, 1.0);
        let base_sclera = Color::new(0.98, 0.98, 0.98, 1.0);
        let base_iris = Color::new(0.85, 0.45, 0.12, 1.0);
        let base_pupil = Color::new(0.05, 0.05, 0.05, 1.0);
        
        let base_part_start = Color::new(0.2, 0.8, 0.2, 1.0);
        let base_part_mid = Color::new(0.0, 0.4, 0.6, 1.0);
        
        // Final colors (200 jumps)
        let end_cheese = Color::new(0.4, 0.05, 0.1, 1.0);       // Dark magma
        let end_mold_dark = Color::new(0.02, 0.0, 0.0, 1.0);     // Pure black
        let end_mold_light = Color::new(0.8, 0.2, 0.0, 1.0);     // Incandescent veins
        let end_sclera = Color::new(1.0, 0.1, 0.1, 1.0);
        let end_iris = Color::new(1.0, 0.8, 0.0, 1.0);
        let end_pupil = Color::new(1.0, 1.0, 0.5, 1.0);          // Bright glow
        
        let end_part_start = Color::new(1.0, 0.2, 0.2, 1.0);
        let end_part_mid = Color::new(0.8, 0.1, 0.1, 1.0);

        Self {
            jumps_required: jumps,
            rotation_speed: 1.0 + 9.0 * t,
            particle_amount: (t * 500.0) as u32,
            particle_color_start: lerp_color(base_part_start, end_part_start, t),
            particle_color_mid: lerp_color(base_part_mid, end_part_mid, t),
            particle_color_end: Color::new(0.1, 0.1, 0.1, 0.0),
            
            cheese_color: lerp_color(base_cheese, end_cheese, t),
            mold_color_dark: lerp_color(base_mold_dark, end_mold_dark, t),
            mold_color_light: lerp_color(base_mold_light, end_mold_light, t),
            
            eye_sclera_color: lerp_color(base_sclera, end_sclera, t),
            eye_iris_color: lerp_color(base_iris, end_iris, t),
            eye_pupil_color: lerp_color(base_pupil, end_pupil, t),
            
            eye_radius_base: 0.09 + (0.06 * t),
            eye_oscillation_speed: 15.0 * t,
            
            // Eye flame emission starts after 20% progress (40 jumps)
            eye_particle_amount: if t > 0.2 { ((t - 0.2) * 400.0) as u32 } else { 0 },
            eye_particle_color: lerp_color(Color::new(1.0, 0.5, 0.0, 1.0), Color::new(0.1, 0.5, 1.0, 1.0), t),
        }
    }

    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        Self {
            jumps_required: self.jumps_required,
            rotation_speed: self.rotation_speed + (other.rotation_speed - self.rotation_speed) * t,
            particle_amount: (self.particle_amount as f32 + (other.particle_amount as f32 - self.particle_amount as f32) * t) as u32,
            particle_color_start: lerp_color(self.particle_color_start, other.particle_color_start, t),
            particle_color_mid: lerp_color(self.particle_color_mid, other.particle_color_mid, t),
            particle_color_end: lerp_color(self.particle_color_end, other.particle_color_end, t),
            
            cheese_color: lerp_color(self.cheese_color, other.cheese_color, t),
            mold_color_dark: lerp_color(self.mold_color_dark, other.mold_color_dark, t),
            mold_color_light: lerp_color(self.mold_color_light, other.mold_color_light, t),
            
            eye_sclera_color: lerp_color(self.eye_sclera_color, other.eye_sclera_color, t),
            eye_iris_color: lerp_color(self.eye_iris_color, other.eye_iris_color, t),
            eye_pupil_color: lerp_color(self.eye_pupil_color, other.eye_pupil_color, t),
            
            eye_radius_base: self.eye_radius_base + (other.eye_radius_base - self.eye_radius_base) * t,
            eye_oscillation_speed: self.eye_oscillation_speed + (other.eye_oscillation_speed - self.eye_oscillation_speed) * t,
            
            eye_particle_amount: (self.eye_particle_amount as f32 + (other.eye_particle_amount as f32 - self.eye_particle_amount as f32) * t) as u32,
            eye_particle_color: lerp_color(self.eye_particle_color, other.eye_particle_color, t),
        }
    }
}
