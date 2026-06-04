use macroquad::prelude::*;

mod config;
mod input;
mod player;
mod world;
mod hud;
mod audio;
mod game;

use config::TICK_DELTA;
use game::GameState;

fn get_game_config() -> Conf {
    config::window_config()
}

#[macroquad::main(get_game_config)]
async fn main() {
    let mut game = match GameState::new().await {
        Ok(game) => game,
        Err(error) => {
            eprintln!("failed to start OpenBHOP: {error}");
            return;
        }
    };
    let mut accumulator = 0.0_f32;

    loop {
        if is_key_pressed(KeyCode::Escape) { break; }

        game.pre_frame_update();

        accumulator += get_frame_time();

        // Limita acúmulo para evitar spiral of death em lag spikes
        if accumulator > 0.1 {
            accumulator = 0.1;
        }

        while accumulator >= TICK_DELTA {
            game.update(TICK_DELTA);
            accumulator -= TICK_DELTA;
        }

        game.draw();
        next_frame().await
    }
}