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
        speed: &mut Vec3,
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
            let speed_scalar = vec2(speed.x, speed.z).length();
            if speed_scalar > 0.0 {
                // Impede desaceleração excessivamente lenta em velocidades quase paradas
                let control = if speed_scalar < 1.0 { 1.0 } else { speed_scalar };
                let drop = control * FRICTION * delta_time;
                let mut new_speed = speed_scalar - drop;
                if new_speed < 0.0 {
                    new_speed = 0.0;
                }
                let scale = new_speed / speed_scalar;
                speed.x *= scale;
                speed.z *= scale;
            }

            // Aceleração no chão
            self.accelerate(speed, wish_dir, MAX_SPEED, MAX_SPEED, GROUND_ACCEL, delta_time);
        } else {
            // No ar, limitamos o wish_speed_cap para criar a clássica restrição de strafe do Half-Life,
            // mas mantemos o MAX_SPEED original para o cálculo da taxa de aceleração por segundo.
            self.accelerate(speed, wish_dir, MAX_SPEED, AIR_WISH_SPEED_CAP, AIR_ACCEL, delta_time);
        }

        // Calcula o deslocamento horizontal baseado na velocidade atualizada
        let displacement = vec3(speed.x * delta_time, 0.0, speed.z * delta_time);

        // Aplica o deslocamento à posição do jogador
        position.x += displacement.x;
        position.z += displacement.z;

        displacement
    }

    // Função de aceleração genérica (Projeção e adição vetorial baseada no motor Source/GoldSrc)
    fn accelerate(
        &self,
        speed: &mut Vec3,
        wish_dir: Vec3,
        wish_speed: f32,
        wish_speed_cap: f32,
        accel: f32,
        delta_time: f32,
    ) {
        // Projeção da velocidade horizontal atual na direção desejada
        let current_speed = speed.x * wish_dir.x + speed.z * wish_dir.z;

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
        speed.x += wish_dir.x * accel_speed;
        speed.z += wish_dir.z * accel_speed;
    }
}