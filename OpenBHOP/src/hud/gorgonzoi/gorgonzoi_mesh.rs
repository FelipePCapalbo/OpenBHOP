use macroquad::prelude::*;
use macroquad::models::{Mesh, Vertex};

#[derive(Clone, Copy)]
pub struct GorgonzoiMeshParams {
    pub cheese_color: Color,
    pub mold_color_dark: Color,
    pub mold_color_light: Color,
    pub eye_sclera_color: Color,
    pub eye_iris_color: Color,
    pub eye_pupil_color: Color,
    pub eye_radius: f32,
}

impl Default for GorgonzoiMeshParams {
    fn default() -> Self {
        Self {
            cheese_color: Color::new(0.96, 0.93, 0.74, 1.0),
            mold_color_dark: Color::new(0.24, 0.38, 0.33, 1.0),
            mold_color_light: Color::new(0.42, 0.55, 0.48, 1.0),
            eye_sclera_color: Color::new(0.98, 0.98, 0.98, 1.0),
            eye_iris_color: Color::new(0.85, 0.45, 0.12, 1.0),
            eye_pupil_color: Color::new(0.05, 0.05, 0.05, 1.0),
            eye_radius: 0.09,
        }
    }
}

#[derive(Clone, Copy)]
pub enum VertexCategory {
    CheeseBase,
    MoldDark,
    MoldLight,
    LeftEyeSclera { local_dir: Vec3 },
    LeftEyeIris { local_dir: Vec3 },
    LeftEyePupil { local_dir: Vec3 },
    RightEyeSclera { local_dir: Vec3 },
    RightEyeIris { local_dir: Vec3 },
    RightEyePupil { local_dir: Vec3 },
    Static,
}

pub struct HudGorgonzoi {
    pub position: Vec3,
    pub rotation: Vec3, // Rotation in radians (pitch, yaw, roll)
    pub scale: Vec3,
    pub eye_left_local: Vec3,
    pub eye_right_local: Vec3,
    base_mesh: Mesh,
    categories: Vec<VertexCategory>,
}

impl HudGorgonzoi {
    pub fn new(position: Vec3) -> Self {
        let height = 0.6;
        let half_height = height / 2.0;

        // Eye positions are static relative to the center of the cheese slice
        let eye_y = half_height + 0.04;
        let eye_z = -0.15;
        let eye_left_local = vec3(-0.15, eye_y, eye_z);
        let eye_right_local = vec3(0.15, eye_y, eye_z);

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();
        let mut categories: Vec<VertexCategory> = Vec::new();

        // 1. Procedural mesh vertices & indices generation (done once)
        // Base triangle vertices
        let vertex_bottom_a = vec3(-0.5, -half_height, -0.6);
        let vertex_bottom_b = vec3(0.5, -half_height, -0.6);
        let vertex_bottom_c = vec3(0.0, -half_height, 0.6);

        let vertex_top_a = vec3(-0.5, half_height, -0.6);
        let vertex_top_b = vec3(0.5, half_height, -0.6);
        let vertex_top_c = vec3(0.0, half_height, 0.6);

        // Helper to determine the initial cheese/mold pattern category for a point
        let get_gorgonzola_color_category = |x: f32, y: f32, z: f32| -> VertexCategory {
            let noise = (x * 12.5).sin() * (y * 14.2).cos() + (z * 17.3).sin() * (x * 8.9).cos();
            if noise > 0.22 {
                VertexCategory::MoldDark
            } else if noise > 0.05 {
                VertexCategory::MoldLight
            } else {
                VertexCategory::CheeseBase
            }
        };

        // Top Cap (Y = 0.25)
        let cap_subdivisions = 6;
        let base_idx_top = vertices.len() as u16;
        for i in 0..=cap_subdivisions {
            for j in 0..=i {
                let weight_a = 1.0 - (i as f32 / cap_subdivisions as f32);
                let weight_b = (i - j) as f32 / cap_subdivisions as f32;
                let weight_c = j as f32 / cap_subdivisions as f32;
                
                let point = vertex_top_a * weight_a + vertex_top_b * weight_b + vertex_top_c * weight_c;
                let cat = get_gorgonzola_color_category(point.x, point.y, point.z);
                
                vertices.push(Vertex::new2(point, vec2(weight_b, weight_c), WHITE));
                categories.push(cat);
            }
        }
        
        for i in 0..cap_subdivisions {
            for j in 0..=i {
                let idx_ij = base_idx_top + (i * (i + 1) / 2 + j) as u16;
                let idx_next_j = base_idx_top + ((i + 1) * (i + 2) / 2 + j) as u16;
                let idx_next_j1 = base_idx_top + ((i + 1) * (i + 2) / 2 + j + 1) as u16;
                
                indices.push(idx_ij);
                indices.push(idx_next_j);
                indices.push(idx_next_j1);
                
                if j < i {
                    let idx_ij1 = base_idx_top + (i * (i + 1) / 2 + j + 1) as u16;
                    indices.push(idx_ij);
                    indices.push(idx_next_j1);
                    indices.push(idx_ij1);
                }
            }
        }

        // Bottom Cap (Y = -0.25)
        let base_idx_bottom = vertices.len() as u16;
        for i in 0..=cap_subdivisions {
            for j in 0..=i {
                let weight_a = 1.0 - (i as f32 / cap_subdivisions as f32);
                let weight_b = (i - j) as f32 / cap_subdivisions as f32;
                let weight_c = j as f32 / cap_subdivisions as f32;
                
                let point = vertex_bottom_a * weight_a + vertex_bottom_b * weight_b + vertex_bottom_c * weight_c;
                let cat = get_gorgonzola_color_category(point.x, point.y, point.z);
                
                vertices.push(Vertex::new2(point, vec2(weight_b, weight_c), WHITE));
                categories.push(cat);
            }
        }
        
        for i in 0..cap_subdivisions {
            for j in 0..=i {
                let idx_ij = base_idx_bottom + (i * (i + 1) / 2 + j) as u16;
                let idx_next_j = base_idx_bottom + ((i + 1) * (i + 2) / 2 + j) as u16;
                let idx_next_j1 = base_idx_bottom + ((i + 1) * (i + 2) / 2 + j + 1) as u16;
                
                indices.push(idx_ij);
                indices.push(idx_next_j1);
                indices.push(idx_next_j);
                
                if j < i {
                    let idx_ij1 = base_idx_bottom + (i * (i + 1) / 2 + j + 1) as u16;
                    indices.push(idx_ij);
                    indices.push(idx_ij1);
                    indices.push(idx_next_j1);
                }
            }
        }

        // Side & Back Faces
        let side_subdivisions = 6;
        let depth_subdivisions = 4;
        
        let generate_quad_face = |p00: Vec3, p10: Vec3, p01: Vec3, p11: Vec3, order_reverse: bool, vertices_list: &mut Vec<Vertex>, indices_list: &mut Vec<u16>, categories_list: &mut Vec<VertexCategory>| {
            let base_idx = vertices_list.len() as u16;
            
            for i in 0..=side_subdivisions {
                let u = i as f32 / side_subdivisions as f32;
                for j in 0..=depth_subdivisions {
                    let v = j as f32 / depth_subdivisions as f32;
                    
                    let point = p00 * (1.0 - u) * (1.0 - v)
                              + p10 * u * (1.0 - v)
                              + p01 * (1.0 - u) * v
                              + p11 * u * v;
                          
                    let cat = get_gorgonzola_color_category(point.x, point.y, point.z);
                    vertices_list.push(Vertex::new2(point, vec2(u, v), WHITE));
                    categories_list.push(cat);
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
                        indices_list.push(idx00);
                        indices_list.push(idx01);
                        indices_list.push(idx11);
                        indices_list.push(idx00);
                        indices_list.push(idx11);
                        indices_list.push(idx10);
                    } else {
                        indices_list.push(idx00);
                        indices_list.push(idx11);
                        indices_list.push(idx01);
                        indices_list.push(idx00);
                        indices_list.push(idx10);
                        indices_list.push(idx11);
                    }
                }
            }
        };

        // Left Face
        generate_quad_face(vertex_bottom_a, vertex_bottom_c, vertex_top_a, vertex_top_c, true, &mut vertices, &mut indices, &mut categories);
        
        // Right Face
        generate_quad_face(vertex_bottom_b, vertex_bottom_c, vertex_top_b, vertex_top_c, false, &mut vertices, &mut indices, &mut categories);
        
        // Back Face
        generate_quad_face(vertex_bottom_a, vertex_bottom_b, vertex_top_a, vertex_top_b, true, &mut vertices, &mut indices, &mut categories);

        // Eyes
        let generate_eye = |center: Vec3, is_left: bool, vertices_list: &mut Vec<Vertex>, indices_list: &mut Vec<u16>, categories_list: &mut Vec<VertexCategory>| {
            let base_idx = vertices_list.len() as u16;
            let latitude_subdivisions = 8;
            let longitude_subdivisions = 8;
            let look_dir = vec3(0.0, 0.0, 1.0);
            
            for i in 0..=latitude_subdivisions {
                let theta = -std::f32::consts::FRAC_PI_2 + std::f32::consts::PI * (i as f32 / latitude_subdivisions as f32);
                for j in 0..=longitude_subdivisions {
                    let phi = 2.0 * std::f32::consts::PI * (j as f32 / longitude_subdivisions as f32);
                    
                    let local_dir = vec3(
                        theta.cos() * phi.sin(),
                        theta.sin(),
                        theta.cos() * phi.cos(),
                    ).normalize_or_zero();
                    
                    let dot = local_dir.dot(look_dir);
                    
                    let category = if dot > 0.88 {
                        if is_left { VertexCategory::LeftEyePupil { local_dir } } else { VertexCategory::RightEyePupil { local_dir } }
                    } else if dot > 0.70 {
                        if is_left { VertexCategory::LeftEyeIris { local_dir } } else { VertexCategory::RightEyeIris { local_dir } }
                    } else {
                        if is_left { VertexCategory::LeftEyeSclera { local_dir } } else { VertexCategory::RightEyeSclera { local_dir } }
                    };
                    
                    // Initial position with default radius = 0.09
                    let point = center + local_dir * 0.09;
                    vertices_list.push(Vertex::new2(point, vec2(i as f32 / latitude_subdivisions as f32, j as f32 / longitude_subdivisions as f32), WHITE));
                    categories_list.push(category);
                }
            }
            
            let stride = (longitude_subdivisions + 1) as u16;
            for i in 0..latitude_subdivisions {
                for j in 0..longitude_subdivisions {
                    let idx00 = base_idx + i * stride + j;
                    let idx10 = base_idx + (i + 1) * stride + j;
                    let idx01 = base_idx + i * stride + (j + 1);
                    let idx11 = base_idx + (i + 1) * stride + (j + 1);
                    
                    indices_list.push(idx00);
                    indices_list.push(idx11);
                    indices_list.push(idx01);
                    
                    indices_list.push(idx00);
                    indices_list.push(idx10);
                    indices_list.push(idx11);
                }
            }
        };

        generate_eye(eye_left_local, true, &mut vertices, &mut indices, &mut categories);
        generate_eye(eye_right_local, false, &mut vertices, &mut indices, &mut categories);

        // Mouth Tube
        let base_idx_mouth = vertices.len() as u16;
        let mouth_subdivisions = 12;
        let tube_subdivisions = 6;
        let radius_x = 0.16;
        let radius_y = 0.07;
        let tube_radius = 0.026;
        let mouth_y = -0.02;
        
        for i in 0..=mouth_subdivisions {
            let alpha = 2.0 * std::f32::consts::PI * (i as f32 / mouth_subdivisions as f32);
            
            let xc = radius_x * alpha.cos();
            let mut yc = radius_y * alpha.sin();
            
            if alpha > 0.0 && alpha < std::f32::consts::PI {
                let angle = alpha * 2.0;
                yc -= 0.015 * angle.cos().max(0.0);
            }
            
            let zc = 0.6 - xc.abs() * 2.4;
            
            let normal = if xc < -0.001 {
                vec3(-0.92, 0.0, 0.38).normalize()
            } else if xc > 0.001 {
                vec3(0.92, 0.0, 0.38).normalize()
            } else {
                vec3(0.0, 0.0, 1.0)
            };
            
            for j in 0..=tube_subdivisions {
                let beta = 2.0 * std::f32::consts::PI * (j as f32 / tube_subdivisions as f32);
                
                let px = xc + tube_radius * beta.cos() * normal.x;
                let py = mouth_y + yc + tube_radius * beta.sin();
                let pz = zc + tube_radius * beta.cos() * normal.z;
                
                let p = vec3(px, py, pz);
                let color = Color::new(0.85, 0.08, 0.12, 1.0); // Mouth red
                
                vertices.push(Vertex::new2(p, vec2(i as f32 / mouth_subdivisions as f32, j as f32 / tube_subdivisions as f32), color));
                categories.push(VertexCategory::Static);
            }
        }
        
        let mouth_stride = (tube_subdivisions + 1) as u16;
        for i in 0..mouth_subdivisions {
            for j in 0..tube_subdivisions {
                let idx00 = base_idx_mouth + i * mouth_stride + j;
                let idx10 = base_idx_mouth + (i + 1) * mouth_stride + j;
                let idx01 = base_idx_mouth + i * mouth_stride + (j + 1);
                let idx11 = base_idx_mouth + (i + 1) * mouth_stride + (j + 1);
                
                indices.push(idx00);
                indices.push(idx11);
                indices.push(idx01);
                indices.push(idx00);
                indices.push(idx10);
                indices.push(idx11);
            }
        }
        
        // Mouth Cavity
        let base_idx_cavity = vertices.len() as u16;
        let cavity_center = vec3(0.0, mouth_y, 0.52);
        vertices.push(Vertex::new2(cavity_center, vec2(0.5, 0.5), Color::new(0.06, 0.01, 0.01, 1.0)));
        categories.push(VertexCategory::Static);
        
        for i in 0..=mouth_subdivisions {
            let alpha = 2.0 * std::f32::consts::PI * (i as f32 / mouth_subdivisions as f32);
            let xc = radius_x * alpha.cos();
            let mut yc = radius_y * alpha.sin();
            if alpha > 0.0 && alpha < std::f32::consts::PI {
                let angle = alpha * 2.0;
                yc -= 0.015 * angle.cos().max(0.0);
            }
            
            let zc = 0.6 - xc.abs() * 2.4 - 0.01;
            let p = vec3(xc, mouth_y + yc, zc);
            vertices.push(Vertex::new2(p, vec2(alpha.cos(), alpha.sin()), Color::new(0.08, 0.01, 0.02, 1.0)));
            categories.push(VertexCategory::Static);
        }
        
        for i in 0..mouth_subdivisions {
            let idx_curr = base_idx_cavity + 1 + i;
            let idx_next = base_idx_cavity + 1 + (i + 1);
            
            indices.push(base_idx_cavity);
            indices.push(idx_curr);
            indices.push(idx_next);
        }

        let mut gorgonzoi = Self {
            position,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
            eye_left_local,
            eye_right_local,
            base_mesh: Mesh {
                vertices,
                indices,
                texture: None,
            },
            categories,
        };

        // Populate colors/positions for the first time
        gorgonzoi.update_mesh(&GorgonzoiMeshParams::default());
        gorgonzoi
    }

    pub fn update_mesh(&mut self, params: &GorgonzoiMeshParams) {
        for (v, cat) in self.base_mesh.vertices.iter_mut().zip(&self.categories) {
            match *cat {
                VertexCategory::CheeseBase => {
                    v.color = params.cheese_color.into();
                }
                VertexCategory::MoldDark => {
                    v.color = params.mold_color_dark.into();
                }
                VertexCategory::MoldLight => {
                    v.color = params.mold_color_light.into();
                }
                VertexCategory::LeftEyeSclera { local_dir } => {
                    v.position = self.eye_left_local + local_dir * params.eye_radius;
                    v.color = params.eye_sclera_color.into();
                }
                VertexCategory::LeftEyeIris { local_dir } => {
                    v.position = self.eye_left_local + local_dir * params.eye_radius;
                    v.color = params.eye_iris_color.into();
                }
                VertexCategory::LeftEyePupil { local_dir } => {
                    v.position = self.eye_left_local + local_dir * params.eye_radius;
                    v.color = params.eye_pupil_color.into();
                }
                VertexCategory::RightEyeSclera { local_dir } => {
                    v.position = self.eye_right_local + local_dir * params.eye_radius;
                    v.color = params.eye_sclera_color.into();
                }
                VertexCategory::RightEyeIris { local_dir } => {
                    v.position = self.eye_right_local + local_dir * params.eye_radius;
                    v.color = params.eye_iris_color.into();
                }
                VertexCategory::RightEyePupil { local_dir } => {
                    v.position = self.eye_right_local + local_dir * params.eye_radius;
                    v.color = params.eye_pupil_color.into();
                }
                VertexCategory::Static => {}
            }
        }
    }

    pub fn get_model_matrix(&self) -> Mat4 {
        let rotation_quat = Quat::from_euler(
            EulerRot::YXZ,
            self.rotation.y,
            self.rotation.x,
            self.rotation.z,
        );
        Mat4::from_scale_rotation_translation(self.scale, rotation_quat, self.position)
    }

    pub fn draw(&self) {
        let model_matrix = self.get_model_matrix();

        let transformed_vertices: Vec<Vertex> = self.base_mesh.vertices.iter().map(|v| {
            let transformed_pos = model_matrix.transform_point3(v.position);
            Vertex {
                position: transformed_pos,
                uv: v.uv,
                color: v.color,
                normal: v.normal,
            }
        }).collect();

        let transformed_mesh = Mesh {
            vertices: transformed_vertices,
            indices: self.base_mesh.indices.clone(),
            texture: None,
        };

        draw_mesh(&transformed_mesh);
    }
}
