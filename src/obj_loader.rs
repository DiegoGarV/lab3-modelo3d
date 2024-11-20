use tobj;
use nalgebra_glm::{Vec2, Vec3};
use crate::vertex::Vertex;

pub struct Obj {
    meshes: Vec<Mesh>,
}

struct Mesh {
    vertex_positions: Vec<Vec3>,
    vertex_normals: Vec<Vec3>,
    texture_coordinates: Vec<Vec2>,
    vertex_index: Vec<u32>,
}

impl Obj {
    pub fn load(file_path: &str) -> Result<Self, tobj::LoadError> {
        let (models, _) = tobj::load_obj(file_path, &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        })?;

        let meshes = models
            .into_iter()
            .map(|model| {
                let mesh = model.mesh;

                Mesh {
                    vertex_positions: mesh
                        .positions
                        .chunks(3)
                        .map(|vertex| Vec3::new(vertex[0], -vertex[1], -vertex[2]))
                        .collect(),

                    vertex_normals: mesh
                        .normals
                        .chunks(3)
                        .map(|normal| Vec3::new(normal[0], -normal[1], -normal[2]))
                        .collect(),

                    texture_coordinates: mesh
                        .texcoords
                        .chunks(2)
                        .map(|tex_coord| Vec2::new(tex_coord[0], 1.0 - tex_coord[1]))
                        .collect(),

                    vertex_index: mesh.indices,
                }
            })
            .collect();

        Ok(Obj { meshes })
    }

    pub fn get_vertex_array(&self) -> Vec<Vertex> {
        let mut vertices = Vec::new();

        for mesh in &self.meshes {
            for &index in &mesh.vertex_index {
                let position = mesh.vertex_positions[index as usize];
                let normal = mesh
                    .vertex_normals
                    .get(index as usize)
                    .cloned()
                    .unwrap_or(Vec3::new(0.0, 1.0, 0.0));

                let tex_coord = mesh
                    .texture_coordinates
                    .get(index as usize)
                    .cloned()
                    .unwrap_or(Vec2::new(0.0, 0.0));

                vertices.push(Vertex::new(position, normal, tex_coord));
            }
        }

        vertices
    }
}
