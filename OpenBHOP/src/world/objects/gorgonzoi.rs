use macroquad::prelude::*;
use macroquad::models::{Mesh, Vertex};

pub struct Gorgonzoi {
    pub position: Vec3,
    pub rotation: Vec3, // Rotacao em radianos (pitch, yaw, roll)
    pub scale: Vec3,
    base_mesh: Mesh,
}

impl Gorgonzoi {
    pub fn new(position: Vec3) -> Self {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();

        // A base triangular maior fica paralela ao chao, com altura/espessura total height
        let height = 0.6;
        let half_height = height / 2.0;

        // Vertices da base inferior (Y = -0.5)
        let vertex_bottom_a = vec3(-0.5, -half_height, -0.6);
        let vertex_bottom_b = vec3(0.5, -half_height, -0.6);
        let vertex_bottom_c = vec3(0.0, -half_height, 0.6);

        // Vertices do topo superior (Y = +0.5)
        let vertex_top_a = vec3(-0.5, half_height, -0.6);
        let vertex_top_b = vec3(0.5, half_height, -0.6);
        let vertex_top_c = vec3(0.0, half_height, 0.6);

        // Funcao de cor procedural para simular o queijo mofado em todo o entorno do gorgonzoi
        let get_gorgonzola_color = |x: f32, y: f32, z: f32| -> Color {
            // Ruido deterministico baseado em senos e cossenos para posicionamento do mofo
            let noise = (x * 12.5).sin() * (y * 14.2).cos() + (z * 17.3).sin() * (x * 8.9).cos();
            
            if noise > 0.22 {
                // Mofo verde-azulado escuro caracteristico do gorgonzoi
                Color::new(0.24, 0.38, 0.33, 1.0)
            } else if noise > 0.05 {
                // Mancha de transicao entre o queijo e o mofo
                Color::new(0.42, 0.55, 0.48, 1.0)
            } else {
                // Queijo creme padrao
                Color::new(0.96, 0.93, 0.74, 1.0)
            }
        };

        // Tampa Superior (Y = 0.25)
        let cap_subdivisions = 6;
        let base_idx_top = vertices.len() as u16;
        for i in 0..=cap_subdivisions {
            for j in 0..=i {
                let weight_a = 1.0 - (i as f32 / cap_subdivisions as f32);
                let weight_b = (i - j) as f32 / cap_subdivisions as f32;
                let weight_c = j as f32 / cap_subdivisions as f32;
                
                let point = vertex_top_a * weight_a + vertex_top_b * weight_b + vertex_top_c * weight_c;
                let color = get_gorgonzola_color(point.x, point.y, point.z);
                
                vertices.push(Vertex::new2(point, vec2(weight_b, weight_c), color));
            }
        }
        
        for i in 0..cap_subdivisions {
            for j in 0..=i {
                let idx_ij = base_idx_top + (i * (i + 1) / 2 + j) as u16;
                let idx_next_j = base_idx_top + ((i + 1) * (i + 2) / 2 + j) as u16;
                let idx_next_j1 = base_idx_top + ((i + 1) * (i + 2) / 2 + j + 1) as u16;
                
                // Triangulo 1 (anti-horario para frente para que a normal aponte para +Y)
                indices.push(idx_ij);
                indices.push(idx_next_j);
                indices.push(idx_next_j1);
                
                if j < i {
                    let idx_ij1 = base_idx_top + (i * (i + 1) / 2 + j + 1) as u16;
                    // Triangulo 2 (anti-horario)
                    indices.push(idx_ij);
                    indices.push(idx_next_j1);
                    indices.push(idx_ij1);
                }
            }
        }

        // Tampa Inferior (Y = -0.25)
        let base_idx_bottom = vertices.len() as u16;
        for i in 0..=cap_subdivisions {
            for j in 0..=i {
                let weight_a = 1.0 - (i as f32 / cap_subdivisions as f32);
                let weight_b = (i - j) as f32 / cap_subdivisions as f32;
                let weight_c = j as f32 / cap_subdivisions as f32;
                
                let point = vertex_bottom_a * weight_a + vertex_bottom_b * weight_b + vertex_bottom_c * weight_c;
                let color = get_gorgonzola_color(point.x, point.y, point.z);
                
                vertices.push(Vertex::new2(point, vec2(weight_b, weight_c), color));
            }
        }
        
        for i in 0..cap_subdivisions {
            for j in 0..=i {
                let idx_ij = base_idx_bottom + (i * (i + 1) / 2 + j) as u16;
                let idx_next_j = base_idx_bottom + ((i + 1) * (i + 2) / 2 + j) as u16;
                let idx_next_j1 = base_idx_bottom + ((i + 1) * (i + 2) / 2 + j + 1) as u16;
                
                // Triangulo 1 (horario para inverter sentido na traseira para que a normal aponte para -Y)
                indices.push(idx_ij);
                indices.push(idx_next_j1);
                indices.push(idx_next_j);
                
                if j < i {
                    let idx_ij1 = base_idx_bottom + (i * (i + 1) / 2 + j + 1) as u16;
                    // Triangulo 2
                    indices.push(idx_ij);
                    indices.push(idx_ij1);
                    indices.push(idx_next_j1);
                }
            }
        }

        // Faces Laterais e Traseira (grades retangulares que conectam as bases)
        let side_subdivisions = 6;
        let depth_subdivisions = 4;
        
        let generate_quad_face = |p00: Vec3, p10: Vec3, p01: Vec3, p11: Vec3, order_reverse: bool, vertices_list: &mut Vec<Vertex>, indices_list: &mut Vec<u16>| {
            let base_idx = vertices_list.len() as u16;
            
            for i in 0..=side_subdivisions {
                let u = i as f32 / side_subdivisions as f32;
                for j in 0..=depth_subdivisions {
                    let v = j as f32 / depth_subdivisions as f32;
                    
                    let point = p00 * (1.0 - u) * (1.0 - v)
                              + p10 * u * (1.0 - v)
                              + p01 * (1.0 - u) * v
                              + p11 * u * v;
                          
                    let color = get_gorgonzola_color(point.x, point.y, point.z);
                    vertices_list.push(Vertex::new2(point, vec2(u, v), color));
                }
            }
            
            let stride = (depth_subdivisions + 1) as u16;
            for i in 0..side_subdivisions {
                for j in 0..depth_subdivisions {
                    let idx00 = base_idx + i * stride + j;
                    let idx10 = base_idx + (i + 1) * stride + j;
                    let idx01 = base_idx + i * stride + (j + 1);
                    let idx11 = base_idx + (i + 1) * stride + (j + 1);
                    
                    if order_reverse {
                        // Triangulo 1
                        indices_list.push(idx00);
                        indices_list.push(idx01);
                        indices_list.push(idx11);
                        // Triangulo 2
                        indices_list.push(idx00);
                        indices_list.push(idx11);
                        indices_list.push(idx10);
                    } else {
                        // Triangulo 1
                        indices_list.push(idx00);
                        indices_list.push(idx11);
                        indices_list.push(idx01);
                        // Triangulo 2
                        indices_list.push(idx00);
                        indices_list.push(idx10);
                        indices_list.push(idx11);
                    }
                }
            }
        };

        // Lateral Esquerda (vertex_bottom_a -> vertex_bottom_c conectado a vertex_top_a -> vertex_top_c)
        generate_quad_face(vertex_bottom_a, vertex_bottom_c, vertex_top_a, vertex_top_c, true, &mut vertices, &mut indices);
        
        // Lateral Direita (vertex_bottom_b -> vertex_bottom_c conectado a vertex_top_b -> vertex_top_c)
        generate_quad_face(vertex_bottom_b, vertex_bottom_c, vertex_top_b, vertex_top_c, false, &mut vertices, &mut indices);
        
        // Face Traseira (vertex_bottom_a -> vertex_bottom_b conectado a vertex_top_a -> vertex_top_b)
        generate_quad_face(vertex_bottom_a, vertex_bottom_b, vertex_top_a, vertex_top_b, true, &mut vertices, &mut indices);

        let generate_eye = |center: Vec3, radius: f32, vertices_list: &mut Vec<Vertex>, indices_list: &mut Vec<u16>| {
            let base_idx = vertices_list.len() as u16;
            let latitude_subdivisions = 8;
            let longitude_subdivisions = 8;
            
            // Direcao do olhar: apontada para frente, perpendicular ao chao (plano horizontal)
            let look_dir = vec3(0.0, 0.0, 1.0);
            
            for i in 0..=latitude_subdivisions {
                let theta = -std::f32::consts::FRAC_PI_2 + std::f32::consts::PI * (i as f32 / latitude_subdivisions as f32);
                for j in 0..=longitude_subdivisions {
                    let phi = 2.0 * std::f32::consts::PI * (j as f32 / longitude_subdivisions as f32);
                    
                    // Esfera 3D padrao
                    let px = radius * theta.cos() * phi.sin();
                    let py = radius * theta.sin();
                    let pz = radius * theta.cos() * phi.cos();
                    
                    let point = center + vec3(px, py, pz);
                    
                    // Normal local do vertice na esfera
                    let local_normal = vec3(px, py, pz).normalize_or_zero();
                    let dot = local_normal.dot(look_dir);
                    
                    // Determinacao de pupila, iris e esclera com base na direcao do olhar
                    let color = if dot > 0.88 {
                        Color::new(0.05, 0.05, 0.05, 1.0)   // Pupila preta
                    } else if dot > 0.70 {
                        Color::new(0.85, 0.45, 0.12, 1.0)   // Iris laranja/marrom
                    } else {
                        Color::new(0.98, 0.98, 0.98, 1.0)   // Esclera branca
                    };
                    
                    vertices_list.push(Vertex::new2(point, vec2(i as f32 / latitude_subdivisions as f32, j as f32 / longitude_subdivisions as f32), color));
                }
            }
            
            let stride = (longitude_subdivisions + 1) as u16;
            for i in 0..latitude_subdivisions {
                for j in 0..longitude_subdivisions {
                    let idx00 = base_idx + i * stride + j;
                    let idx10 = base_idx + (i + 1) * stride + j;
                    let idx01 = base_idx + i * stride + (j + 1);
                    let idx11 = base_idx + (i + 1) * stride + (j + 1);
                    
                    // Triangulo 1
                    indices_list.push(idx00);
                    indices_list.push(idx11);
                    indices_list.push(idx01);
                    // Triangulo 2
                    indices_list.push(idx00);
                    indices_list.push(idx10);
                    indices_list.push(idx11);
                }
            }
        };

        // Olhos posicionados no topo (Y = 0.50). Adicionamos +0.04 em Y para assenta-los na superficie.
        let eye_y = half_height + 0.04;
        let eye_z = -0.15;
        // Olho Esquerdo
        generate_eye(vec3(-0.15, eye_y, eye_z), 0.09, &mut vertices, &mut indices);
        // Olho Direito
        generate_eye(vec3(0.15, eye_y, eye_z), 0.09, &mut vertices, &mut indices);

        let base_idx_mouth = vertices.len() as u16;
        let mouth_subdivisions = 12;
        let tube_subdivisions = 6;
        let radius_x = 0.16;
        let radius_y = 0.07;
        let tube_radius = 0.026;
        
        let mouth_y = -0.02; // Posicionado ligeiramente abaixo do meio vertical da quina
        
        for i in 0..=mouth_subdivisions {
            let alpha = 2.0 * std::f32::consts::PI * (i as f32 / mouth_subdivisions as f32);
            
            let xc = radius_x * alpha.cos();
            let mut yc = radius_y * alpha.sin();
            
            // Curvatura do arco do cupido nos labios superiores
            if alpha > 0.0 && alpha < std::f32::consts::PI {
                let angle = alpha * 2.0;
                yc -= 0.015 * angle.cos().max(0.0);
            }
            
            // A coordenada Z do centro do labio adapta-se a quina confluente em formato de ponta
            let zc = 0.6 - xc.abs() * 2.4;
            
            // Normal a superficie do queijo no ponto correspondente das faces laterais
            let normal = if xc < -0.001 {
                vec3(-0.92, 0.0, 0.38).normalize()
            } else if xc > 0.001 {
                vec3(0.92, 0.0, 0.38).normalize()
            } else {
                vec3(0.0, 0.0, 1.0)
            };
            
            // Gera a secao circular do tubo do labio extrudada para fora da quina
            for j in 0..=tube_subdivisions {
                let beta = 2.0 * std::f32::consts::PI * (j as f32 / tube_subdivisions as f32);
                
                let px = xc + tube_radius * beta.cos() * normal.x;
                let py = mouth_y + yc + tube_radius * beta.sin();
                let pz = zc + tube_radius * beta.cos() * normal.z;
                
                let p = vec3(px, py, pz);
                let color = Color::new(0.85, 0.08, 0.12, 1.0); // Vermelho carmim labio
                
                vertices.push(Vertex::new2(p, vec2(i as f32 / mouth_subdivisions as f32, j as f32 / tube_subdivisions as f32), color));
            }
        }
        
        let mouth_stride = (tube_subdivisions + 1) as u16;
        for i in 0..mouth_subdivisions {
            for j in 0..tube_subdivisions {
                let idx00 = base_idx_mouth + i * mouth_stride + j;
                let idx10 = base_idx_mouth + (i + 1) * mouth_stride + j;
                let idx01 = base_idx_mouth + i * mouth_stride + (j + 1);
                let idx11 = base_idx_mouth + (i + 1) * mouth_stride + (j + 1);
                
                // Triangulo 1
                indices.push(idx00);
                indices.push(idx11);
                indices.push(idx01);
                // Triangulo 2
                indices.push(idx00);
                indices.push(idx10);
                indices.push(idx11);
            }
        }
        
        // Fundo escuro da cavidade bucal para dar sensacao de profundidade interior na quina
        let base_idx_cavity = vertices.len() as u16;
        let cavity_center = vec3(0.0, mouth_y, 0.52); // Recuado para dentro da fatia de queijo
        vertices.push(Vertex::new2(cavity_center, vec2(0.5, 0.5), Color::new(0.06, 0.01, 0.01, 1.0)));
        
        for i in 0..=mouth_subdivisions {
            let alpha = 2.0 * std::f32::consts::PI * (i as f32 / mouth_subdivisions as f32);
            let xc = radius_x * alpha.cos();
            let mut yc = radius_y * alpha.sin();
            if alpha > 0.0 && alpha < std::f32::consts::PI {
                let angle = alpha * 2.0;
                yc -= 0.015 * angle.cos().max(0.0);
            }
            
            let zc = 0.6 - xc.abs() * 2.4 - 0.01; // Levemente para dentro do plano das laterais
            let p = vec3(xc, mouth_y + yc, zc);
            vertices.push(Vertex::new2(p, vec2(alpha.cos(), alpha.sin()), Color::new(0.08, 0.01, 0.02, 1.0)));
        }
        
        for i in 0..mouth_subdivisions {
            let idx_curr = base_idx_cavity + 1 + i;
            let idx_next = base_idx_cavity + 1 + (i + 1);
            
            indices.push(base_idx_cavity);
            indices.push(idx_curr);
            indices.push(idx_next);
        }

        let base_mesh = Mesh {
            vertices,
            indices,
            texture: None,
        };

        Self {
            position,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
            base_mesh,
        }
    }

    pub fn draw(&self) {
        // Constroi a matriz de transformacao do modelo
        let rotation_quat = Quat::from_euler(
            EulerRot::YXZ,
            self.rotation.y,
            self.rotation.x,
            self.rotation.z,
        );
        let model_matrix = Mat4::from_scale_rotation_translation(self.scale, rotation_quat, self.position);

        // Aplica a matriz de transformacao a todos os vertices locais
        let transformed_vertices: Vec<Vertex> = self.base_mesh.vertices.iter().map(|v| {
            let transformed_pos = model_matrix.transform_point3(v.position);
            Vertex {
                position: transformed_pos,
                uv: v.uv,
                color: v.color,
                normal: v.normal,
            }
        }).collect();

        // Constroi a malha temporaria para o desenho deste frame
        let transformed_mesh = Mesh {
            vertices: transformed_vertices,
            indices: self.base_mesh.indices.clone(),
            texture: None,
        };

        draw_mesh(&transformed_mesh);
    }
}
