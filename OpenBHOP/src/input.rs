use macroquad::prelude::*;

pub struct InputService {
    pub movement: Vec3,
    pub mouse_delta: Vec2,
    pub cursor_grabbed: bool,
    last_mouse_position: Vec2,
}

impl InputService {
    pub fn new() -> Self {
        let initial_mouse = mouse_position().into();
        set_cursor_grab(true);
        show_mouse(false);

        Self {
            movement: Vec3::ZERO,
            mouse_delta: Vec2::ZERO,
            cursor_grabbed: true,
            last_mouse_position: initial_mouse,
        }
    }

    pub fn handle_input(&mut self, _delta_time: f32) {

        if is_key_pressed(KeyCode::Tab) {
            self.cursor_grabbed = !self.cursor_grabbed;
            set_cursor_grab(self.cursor_grabbed);
            show_mouse(!self.cursor_grabbed);
        }

        let current_mouse: Vec2 = mouse_position().into();
        if self.cursor_grabbed {
            self.mouse_delta = current_mouse - self.last_mouse_position;
        } else {
            self.mouse_delta = Vec2::ZERO;
        }
        self.last_mouse_position = current_mouse;

        let mut move_vec = Vec3::ZERO;
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up)    { move_vec.z += 1.0; }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down)  { move_vec.z -= 1.0; }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left)  { move_vec.x -= 1.0; }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) { move_vec.x += 1.0; }
        
        self.movement = move_vec.normalize_or_zero();
    }
}