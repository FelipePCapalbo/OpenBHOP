use macroquad::prelude::*;
use super::gorgonzoi_mesh::GorgonzoiMesh;

pub struct Gorgonzoi {
    pub position: Vec3,
    pub rotation: Vec3, // Rotation in radians (pitch, yaw, roll)
    pub scale: Vec3,
    mesh: GorgonzoiMesh,
}

impl Gorgonzoi {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
            mesh: GorgonzoiMesh::new(),
        }
    }

    pub fn draw(&self) {
        self.mesh.draw(self.position, self.rotation, self.scale);
    }
}
