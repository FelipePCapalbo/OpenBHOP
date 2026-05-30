use macroquad::prelude::*;
use crate::config::LOOK_SPEED;

pub struct FirstPersonCamera {
    pub yaw: f32,
    pub pitch: f32,
    pub front: Vec3,
    pub up: Vec3,
}

impl FirstPersonCamera {
    pub fn new() -> Self {
        Self {
            yaw: 1.18,
            pitch: 0.0,
            front: vec3(0.0, 0.0, 1.0),
            up: vec3(0.0, 1.0, 0.0),
        }
    }

    pub fn update(&mut self, mouse_delta: Vec2) {
        self.yaw += mouse_delta.x * LOOK_SPEED;
        self.pitch += mouse_delta.y * -LOOK_SPEED;
        self.pitch = self.pitch.clamp(-1.5, 1.5);

        self.front = vec3(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        ).normalize();
        
        let world_up = vec3(0.0, 1.0, 0.0);
        let right = self.front.cross(world_up).normalize();
        self.up = right.cross(self.front).normalize();
    }
}