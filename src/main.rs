use msgpacker::prelude::*;
use std::{cmp, fs::*, io::*};
// use std::boxed::Box;
// use std::error::Error as StdError;
use noobwerkz::serialized_model::*;

fn traverse_node_recursive(
    node: &gltf::Node,
    meshes: &mut Vec<SerializedMesh>,
    buffers: &Vec<gltf::buffer::Data>,
) {
    // Process the current node (e.g., access its mesh, transform, etc.)
    // ...
    println!("Node name {}", node.name().unwrap_or_default());
    let mesh = node.mesh();
    if mesh.is_some() {
        println!("Node {} has mesh", node.name().unwrap_or_default());
        let prims = mesh.unwrap().primitives();
        let mut serialized_mesh = noobwerkz::serialized_model::SerializedMesh::new();
        let mut min_extents = [0.0 as f32; 3];
        let mut max_extents = [0.0 as f32; 3];
        for p in prims {
            let mode = p.mode();
            match mode {
                gltf::mesh::Mode::Points => {}
                gltf::mesh::Mode::Lines => {}
                gltf::mesh::Mode::LineLoop => {}
                gltf::mesh::Mode::LineStrip => {}
                gltf::mesh::Mode::Triangles => {
                    let reader = p.reader(|buffer| Some(&buffers[buffer.index()]));
                    if let Some(positions) = reader.read_positions() {
                        for p in positions {
                            // Process vertex positions (e.g., store them in a Vec)
                            serialized_mesh.positions.push(p);
                            let mut i = 0;
                            while i < 3 {
                                let biggest: f32;
                                if max_extents[i] > p[i] {
                                    biggest = max_extents[i];
                                } else {
                                    biggest = p[i];
                                }
                                max_extents[i] = biggest;

                                let smallest: f32;
                                if min_extents[i] < p[i] {
                                    smallest = min_extents[i];
                                } else {
                                    smallest = p[i];
                                }
                                min_extents[i] = smallest;

                                i += 1;
                            }
                        }
                    }
                    if let Some(normals) = reader.read_normals() {
                        for n in normals {
                            serialized_mesh.normals.push(n);
                        }
                    }
                    if let Some(uvs) = reader.read_tex_coords(0) {
                        for u in uvs.into_f32() {
                            serialized_mesh.uvs.push(u);
                        }
                    }
                    if let Some(bone_indices) = reader.read_joints(0) {
                        for bi in bone_indices.into_u16() {
                            let bi_u32 = [bi[0] as u32, bi[1] as u32, bi[2] as u32, bi[3] as u32];
                            serialized_mesh.bone_indices.push(bi_u32);
                        }
                    }
                    if let Some(weights) = reader.read_weights(0) {
                        for w in weights.into_f32() {
                            serialized_mesh.bone_weights.push(w);
                        }
                    }
                    if let Some(indices) = reader.read_indices() {
                        for i in indices.into_u32() {
                            serialized_mesh.indices.push(i);
                        }
                    }
                }
                gltf::mesh::Mode::TriangleStrip => {}
                gltf::mesh::Mode::TriangleFan => {}
            }
            let mat = p.material();
            match mat.index() {
                Some(index) => {
                    serialized_mesh.material_index = index as u32;
                }
                None => {}
            }
        }
        //println!("{:#?}", node.transform());
        let decomposed = node.transform().decomposed();
        serialized_mesh.translation = decomposed.0;
        serialized_mesh.rotation = decomposed.1;
        serialized_mesh.scale = decomposed.2;

        let mut dims = [0.0 as f32; 3];
        let mut i = 0;
        while i < 3 {
            dims[i] = max_extents[i] - min_extents[i];
            i += 1;
        }
        serialized_mesh.dimensions = dims;
        meshes.push(serialized_mesh);
    }
    // Recursively visit all children
    for child in node.children() {
        traverse_node_recursive(&child, meshes, buffers);
    }
}

fn run(path: &str) {
    //let file = fs::File::open(path);
    // let reader = io::BufReader::new(file);
    let (document, buffers, images) = gltf::import(path).unwrap();
    // println!("{:#?}", gltf);
    let mut scenes = document.scenes();
    let scene = scenes.next().unwrap();
    let scene_name = scene.name().unwrap_or_default();
    println!("Scene name: {}", scene_name);
    let roots = scene.nodes();
    let mut serialized_meshes = Vec::<SerializedMesh>::new();
    let mut serialized_materials = Vec::<SerializedMaterial>::new();
    for r in roots {
        traverse_node_recursive(&r, &mut serialized_meshes, &buffers);
    }
    for mat in document.materials() {
        let mut serialized_material = SerializedMaterial::new();
        println!("Material name: {} ", mat.name().unwrap_or_default());

        if let Some(pbr) = mat.pbr_metallic_roughness().base_color_texture() {
            let texture = document
                .textures()
                .nth(pbr.texture().index())
                .expect("Texture not found");
            //println!("Texture name {}", texture.name().unwrap_or_default());
            //let tex = document.textures().nth(tex_data.index()).expect("texture not found");//.ok_or("texture not found");
            let img = texture.source();
            match img.source() {
                gltf::image::Source::View { view, mime_type } => {
                    println!("Embedded diffuse texture MIME type: {}", mime_type);
                    // TODO: Find out if correct
                    let _image_data = &images[view.index()].pixels;
                }
                gltf::image::Source::Uri { uri, mime_type } => {
                    println!(
                        "External diffuse texture URI: {}, MIME type: {}",
                        uri,
                        mime_type.unwrap_or_default()
                    );
                    serialized_material.diffuse_texture_path = uri.to_owned();
                }
            }
        }

        if let Some(normal_texture) = mat.normal_texture() {
            let temp = normal_texture.texture();
            let image = temp.source();
            match image.source() {
                gltf::image::Source::View { view, mime_type } => {
                    println!("Embedded normal texture MIME type: {}", mime_type);
                    // let image_data = &images[source.index()].pixels;
                }
                gltf::image::Source::Uri { uri, mime_type } => {
                    println!(
                        "External normals texture URI: {}, MIME type: {}",
                        uri,
                        mime_type.unwrap_or_default()
                    );
                    serialized_material.normals_texture_path = uri.to_owned();
                }
            }
        }
        serialized_materials.push(serialized_material);
    }
    let mut serialized_model = noobwerkz::serialized_model::SerializedModel::new();
    serialized_model.meshes = serialized_meshes;
    serialized_model.materials = serialized_materials;
    let mut buf = Vec::new();
    let n = serialized_model.pack(&mut buf);
    let mut file = File::create("model.bin").unwrap();
    let results = file.write_all(&buf);
    match results {
        Ok(data)=> { println!("Writing file", data)}
        Err(e) => { println!("Error writing file: {}", e)}
    }
}

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        run(&path);
    } else {
        println!("usage: gltf2msgpack <FILE>");
    }
}
