use crate::config::{GRAVITY, JUMP_FORCE};

pub struct VerticalPhysics {
    pub position_y: f32,
    pub speed_y: f32,
    pub is_grounded: bool,
}

impl VerticalPhysics {
    pub fn new(initial_y: f32) -> Self {
        Self {
            position_y: initial_y,
            speed_y: 0.0,
            is_grounded: true,
        }
    }

    pub fn apply_gravity(&mut self) {
        self.speed_y += GRAVITY;
        self.position_y += self.speed_y;

        if self.position_y <= 1.0 {
            self.position_y = 1.0;
            self.speed_y = 0.0;
            self.is_grounded = true;
        }
    }

    pub fn trigger_jump(&mut self) {
        if self.is_grounded {
            self.speed_y = JUMP_FORCE;
            self.is_grounded = false;
        }
    }
}