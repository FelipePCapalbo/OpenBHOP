use macroquad::prelude::*;
use crate::config::{MAX_SPEED, GROUND_ACCEL, AIR_ACCEL, FRICTION, AIR_WISH_SPEED_CAP};

pub struct Locomotion {}

impl Locomotion {
    pub fn new() -> Self {
        Self {}
    }

    pub fn apply_horizontal_input(
        &self,
        position: &mut Vec3,
        velocity: &mut Vec3,
        input_movement: Vec3,
        camera_front: Vec3,
        is_grounded: bool,
        delta_time: f32,
    ) -> Vec3 {
        // Direções relativas à câmera no plano horizontal
        let forward_dir = vec3(camera_front.x, 0.0, camera_front.z).normalize_or_zero();
        let world_up = vec3(0.0, 1.0, 0.0);
        let right_dir = camera_front.cross(world_up).normalize_or_zero();

        // Direção desejada do movimento baseada nas teclas pressionadas
        let wish_dir = (forward_dir * input_movement.z + right_dir * input_movement.x).normalize_or_zero();

        if is_grounded {
            // Aplicar atrito físico apenas quando estiver no chão
            let speed = vec2(velocity.x, velocity.z).length();
            if speed > 0.0 {
                // Impede desaceleração excessivamente lenta em velocidades quase paradas
                let control = if speed < 1.0 { 1.0 } else { speed };
                let drop = control * FRICTION * delta_time;
                let mut new_speed = speed - drop;
                if new_speed < 0.0 {
                    new_speed = 0.0;
                }
                let scale = new_speed / speed;
                velocity.x *= scale;
                velocity.z *= scale;
            }

            // Aceleração no chão
            self.accelerate(velocity, wish_dir, MAX_SPEED, MAX_SPEED, GROUND_ACCEL, delta_time);
        } else {
            // No ar, limitamos o wish_speed_cap para criar a clássica restrição de strafe do Half-Life,
            // mas mantemos o MAX_SPEED original para o cálculo da taxa de aceleração por segundo.
            self.accelerate(velocity, wish_dir, MAX_SPEED, AIR_WISH_SPEED_CAP, AIR_ACCEL, delta_time);
        }

        // Calcula o deslocamento horizontal baseado na velocidade atualizada
        let displacement = vec3(velocity.x * delta_time, 0.0, velocity.z * delta_time);

        // Aplica o deslocamento à posição do jogador
        position.x += displacement.x;
        position.z += displacement.z;

        displacement
    }

    // Função de aceleração genérica (Projeção e adição vetorial baseada no motor Source/GoldSrc)
    fn accelerate(
        &self,
        velocity: &mut Vec3,
        wish_dir: Vec3,
        wish_speed: f32,
        wish_speed_cap: f32,
        accel: f32,
        delta_time: f32,
    ) {
        // Projeção da velocidade horizontal atual na direção desejada
        let current_speed = velocity.x * wish_dir.x + velocity.z * wish_dir.z;

        // Quão abaixo da velocidade desejada limitada estamos
        let add_speed = wish_speed_cap - current_speed;

        // Se já atingimos ou superamos a velocidade de desejo na direção dada, não acelera mais
        if add_speed <= 0.0 {
            return;
        }

        // Calcula o incremento de aceleração para este frame (usando o wish_speed original para taxa de aceleração ideal)
        let mut accel_speed = accel * wish_speed * delta_time;
        if accel_speed > add_speed {
            accel_speed = add_speed;
        }

        // Adiciona a aceleração vetorial
        velocity.x += wish_dir.x * accel_speed;
        velocity.z += wish_dir.z * accel_speed;
    }
}