use macroquad::prelude::*;
use macroquad::audio::load_sound;

mod config;
mod input;
mod player;
mod world;
mod hud;
mod game;

use game::GameState;

fn get_game_config() -> Conf {
    config::window_config()
}

#[macroquad::main(get_game_config)]
async fn main() {
    let jump_sound = load_sound("assets/audio/jump.wav").await.unwrap();
    
    let mut game = GameState::new();

    loop {
        if is_key_pressed(KeyCode::Escape) { break; }

        let delta_time = get_frame_time();

        game.update(delta_time, &jump_sound);
        game.draw();

        next_frame().await
    }
}