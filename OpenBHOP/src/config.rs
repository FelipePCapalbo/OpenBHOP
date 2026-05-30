use macroquad::prelude::Conf;

pub const MOVE_SPEED: f32 = 0.15;
pub const LOOK_SPEED: f32 = 0.1;
pub const GRAVITY: f32 = -0.01;
pub const JUMP_FORCE: f32 = 0.2;

pub fn window_config() -> Conf {
    Conf {
        window_title: String::from("OpenBHOP"),
        window_width: 1260,
        window_height: 768,
        ..Default::default()
    }
}