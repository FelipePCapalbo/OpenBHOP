use macroquad::prelude::*;

use super::locomotion::Locomotion;
use super::vertical_physics::VerticalPhysics;
use super::jump::JumpController;
use super::telemetry::Telemetry;
use crate::config::BHOP_WINDOW_MS;

pub struct Kinematics {
    pub position: Vec3,
    pub speed: Vec3,
    pub is_grounded: bool,
    pub locomotion: Locomotion,
    pub vertical_physics: VerticalPhysics,
    pub jump_controller: JumpController,
    pub telemetry: Telemetry,
    
    // Variáveis de controle para o Bunny Hopping (BHOP)
    pub bhop_timer: f32,                       // Contador regressivo de tolerância para o pulo de preservação (em segundos)
    pub saved_horizontal_speed: Vec3,       // Velocidade horizontal preservada antes de pousar
}

impl Kinematics {
    pub fn new() -> Self {
        Self {
            position: vec3(0.0, 1.0, 0.0),
            speed: Vec3::ZERO,
            is_grounded: true,
            locomotion: Locomotion::new(),
            vertical_physics: VerticalPhysics::new(),
            jump_controller: JumpController::new(),
            telemetry: Telemetry::new(),
            bhop_timer: 0.0,
            saved_horizontal_speed: Vec3::ZERO,
        }
    }

    pub fn apply_movement(&mut self, input_movement: Vec3, camera_front: Vec3, delta_time: f32) {
        // Decrementa o timer do Bunny Hop se estiver ativo
        if self.bhop_timer > 0.0 {
            self.bhop_timer -= delta_time;
            if self.bhop_timer < 0.0 {
                self.bhop_timer = 0.0;
            }
        }

        // Guarda o estado de grounded do frame anterior antes do processamento físico do frame atual
        let was_grounded_prev = self.is_grounded;

        // Processa a movimentação horizontal (aceleração, atrito e integração de posição)
        self.locomotion.apply_horizontal_input(
            &mut self.position,
            &mut self.speed,
            input_movement,
            camera_front,
            self.is_grounded,
            delta_time,
        );
        
        // Aplica a gravidade e detecta colisão/aterrissagem com o chão
        self.vertical_physics.apply_gravity(&mut self.position, &mut self.speed, &mut self.is_grounded, delta_time);

        // Detecta se o jogador acabou de aterrissar (estava no ar e agora está no chão)
        if !was_grounded_prev && self.is_grounded {
            // Salva a velocidade horizontal mantida no ar imediatamente antes do impacto com o chão
            self.saved_horizontal_speed = vec3(self.speed.x, 0.0, self.speed.z);
            // Inicia o timer da janela de tolerância de BHOP (convertendo milissegundos para segundos)
            self.bhop_timer = BHOP_WINDOW_MS / 1000.0;
        }

        self.telemetry.update_speed(self.speed);
    }

    pub fn jump(&mut self) -> bool {
        if self.jump_controller.trigger_jump(&mut self.speed, &mut self.is_grounded) {
            // Se o pulo ocorrer dentro da janela de tolerância (bhop_timer ativo), preservamos a velocidade horizontal salva
            if self.bhop_timer > 0.0 {
                self.speed.x = self.saved_horizontal_speed.x;
                self.speed.z = self.saved_horizontal_speed.z;
                self.bhop_timer = 0.0; // Consome o timer para evitar múltiplas preservações no mesmo ciclo
            }
            true
        } else {
            false
        }
    }
}