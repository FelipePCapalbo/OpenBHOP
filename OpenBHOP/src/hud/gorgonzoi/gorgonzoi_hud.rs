use macroquad::prelude::*;
use super::gorgonzoi_mesh::{HudGorgonzoi, GorgonzoiMeshParams};
use super::particles_stage::ParticlesStage;
use super::bhop_combo::ComboLevelConfig;

pub struct GorgonzoiHud {
    gorgonzoi_mesh: HudGorgonzoi,
    particles: ParticlesStage,
    current_params: ComboLevelConfig,
}

impl GorgonzoiHud {
    pub fn new() -> Self {
        let current_params = ComboLevelConfig::from_jumps(0);

        Self {
            gorgonzoi_mesh: HudGorgonzoi::new(vec3(0.0, 0.0, 0.0)),
            particles: ParticlesStage::new(),
            current_params,
        }
    }

    pub fn draw(&mut self, x: f32, y: f32, bhop_combo_count: u32) {
        let target_params = ComboLevelConfig::from_jumps(bhop_combo_count);
        let dt = get_frame_time();
        
        // Smooth transition (Lerp) for all parameters over time
        self.current_params = self.current_params.lerp(&target_params, dt * 2.0);

        // Prepare the mesh parameters for this frame
        let mesh_params = GorgonzoiMeshParams {
            cheese_color: self.current_params.cheese_color,
            mold_color_dark: self.current_params.mold_color_dark,
            mold_color_light: self.current_params.mold_color_light,
            eye_sclera_color: self.current_params.eye_sclera_color,
            eye_iris_color: self.current_params.eye_iris_color,
            eye_pupil_color: self.current_params.eye_pupil_color,
            
            // Real-time eye oscillation based on continuous game time
            eye_radius: self.current_params.eye_radius_base 
                        + (get_time() as f32 * self.current_params.eye_oscillation_speed).sin() * 0.02,
        };
        
        // Update mesh attributes in-place (colors and oscillating positions)
        self.gorgonzoi_mesh.update_mesh(&mesh_params);

        // Accumulate rotation based on the current combo level rotation speed
        self.gorgonzoi_mesh.rotation.y += self.current_params.rotation_speed * dt;
        self.gorgonzoi_mesh.rotation.x = 0.2; // Slight tilt
                
        // Setup temporary Camera3D for rendering the 3D Gorgonzoi mesh in the HUD viewport
        let camera_pos = vec3(0.0, 0.0, 1.5);
        let size = 120.0;
        let viewport_rect = Rect::new(x - size/2.0, y - size/2.0, size, size);
        
        let hud_camera = Camera3D {
            position: camera_pos,
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            fovy: 45.0,
            aspect: Some(1.0),
            projection: Projection::Perspective,
            render_target: None,
            viewport: Some((
                viewport_rect.x as i32,
                (screen_height() - viewport_rect.y - viewport_rect.h) as i32,
                viewport_rect.w as i32,
                viewport_rect.h as i32,
            )),
            ..Default::default()
        };

        // Render background particles (general body aura) in 2D
        self.particles.draw_background(vec2(x, y));

        // Apply temporary camera and draw the 3D mesh
        set_camera(&hud_camera);
        self.gorgonzoi_mesh.draw();
        set_default_camera();

        // Project the 3D local eye positions to 2D HUD screen coordinates
        // so that eye flame particles can be drawn in the foreground on top of the mesh
        let model_matrix = self.gorgonzoi_mesh.get_model_matrix();
        let view_matrix = Mat4::look_at_rh(camera_pos, vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0));
        let proj_matrix = Mat4::perspective_rh_gl(45.0f32.to_radians(), 1.0, 0.01, 100.0);
        let mvp = proj_matrix * view_matrix * model_matrix;

        let project_to_screen = |local_pos: Vec3| -> Vec2 {
            let clip = mvp * vec4(local_pos.x, local_pos.y, local_pos.z, 1.0);
            if clip.w > 0.0 {
                let ndc = clip.xyz() / clip.w;
                // Convert NDC to screen coordinates based on the viewport
                vec2(
                    viewport_rect.x + (ndc.x * 0.5 + 0.5) * viewport_rect.w,
                    viewport_rect.y + (0.5 - ndc.y * 0.5) * viewport_rect.h
                )
            } else {
                vec2(x, y) // Fallback to center if behind camera
            }
        };

        // Reduce eye particle emissions when Gorgonzoi is turned away
        let mut left_amount = self.current_params.eye_particle_amount;
        let mut right_amount = self.current_params.eye_particle_amount;
        
        let face_normal = (model_matrix * vec4(0.0, 0.0, 1.0, 0.0)).xyz();
        let view_dir = vec3(0.0, 0.0, -1.0); // Camera view direction
        if face_normal.dot(view_dir) < -0.2 { 
            left_amount /= 5;
            right_amount /= 5;
        }

        // Update particle emitter configuration
        self.particles.update_configs(
            self.current_params.particle_amount, 
            self.current_params.particle_color_start, 
            self.current_params.particle_color_mid, 
            self.current_params.particle_color_end,
            left_amount,
            right_amount,
            self.current_params.eye_particle_color,
        );

        let left_screen = project_to_screen(self.gorgonzoi_mesh.eye_left_local);
        let right_screen = project_to_screen(self.gorgonzoi_mesh.eye_right_local);

        self.particles.draw_foreground_eyes(left_screen, right_screen);
    }
}
