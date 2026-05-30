use macroquad::prelude::*;
use crate::input::InputService;
use crate::player::Player;
use crate::world::World;

pub struct GameState {
    pub player: Player,
    pub world: World,
    pub input: InputService,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            world: World::new(),
            input: InputService::new(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.input.handle_input(delta_time);
        self.player.update(&self.input, delta_time);
    }

    pub fn draw(&self) {
        clear_background(LIGHTGRAY);

        set_camera(&Camera3D {
            position: self.player.kinematics.position,
            up: self.player.camera.up,
            target: self.player.kinematics.position + self.player.camera.front,
            ..Default::default()
        });

        self.world.draw();

        set_default_camera();
        self.draw_hud();
    }

    fn draw_hud(&self) {
        draw_text("Controls: WASD + MOUSE; SPACE to Jump; TAB to unlock mouse", 10.0, 20.0, 22.0, BLACK);
        
        let pos_text = format!(
            "XYZ: {:.2}, {:.2}, {:.2}", 
            self.player.kinematics.position.x, 
            self.player.kinematics.position.y, 
            self.player.kinematics.position.z
        );
        draw_text(&pos_text, 10.0, 45.0, 22.0, DARKGRAY);

        let speed_text = format!("Speed: {:.1} u/s", self.player.kinematics.telemetry.current_speed);
        let speed_color = if self.player.kinematics.telemetry.current_speed > 0.1 { ORANGE } else { GRAY };
        draw_text(&speed_text, 10.0, 75.0, 26.0, speed_color);
    }
}