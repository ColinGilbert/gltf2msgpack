use gltf::*;
use msgpacker::*;

#[derive(Debug, PartialEq)]
struct SerializedMesh {
    name: String,
    translation: [f32; 3],
    scale: [f32; 3],
    dimensions: [f32; 3],
    rotation: [f32; 4],
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    bone_indices: Vec<[u32; 4]>,
    bone_weights: Vec<[f32; 4]>,
    indices: Vec<u32>,
    bone_names: Vec<String>,
    material_index: u32,
}

impl SerializedMesh {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            translation: [0.0; 3],
            scale: [0.0; 3],
            dimensions: [0.0; 3],
            rotation: [0.0; 4],
            positions: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            bone_indices: Vec::new(),
            bone_weights: Vec::new(),
            indices: Vec::new(),
            bone_names: Vec::new(),
            material_index: 0,
        }
    }
}

#[derive(Debug, PartialEq)]
struct SerializedMaterial {
    name: String,
    diffuse_texture_path: String,
    normals_texture_path: String,
    specular_texture_path: String,
}

impl SerializedMaterial {
    pub fn new() -> Self {
        Self {
            name: "".to_owned(),
            diffuse_texture_path: "".to_owned(),
            normals_texture_path: "".to_owned(),
            specular_texture_path: "".to_owned(),
        }
    }
}

pub fn load(filename: String) {

}