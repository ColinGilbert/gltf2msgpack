use std::{fs, io};
// use std::boxed::Box;
// use std::error::Error as StdError;
use gltf::*;
use noobwerkz::serialized_model::*;

fn traverse_node_recursive(
    node: &gltf::Node,
    meshes: &mut Vec<SerializedMesh>,
    materials: &mut Vec<SerializedMaterial>,
    buffers: &Vec<Data>,
) {
    // Process the current node (e.g., access its mesh, transform, etc.)
    // ...
    println!("Node name {}", node.name().unwrap_or_default());
    let mesh = node.mesh();
    if mesh.is_some() {
        println!("Node {} has mesh", node.name().unwrap_or_default());
        let prims = mesh.unwrap().primitives();
        for p in prims {
            let mode = p.mode();
            match mode {
                Points => {}
                Lines => {}
                LineLoop => {}
                LineStrip => {}
                Triangles => {
                    let reader = p.reader(|buffer| Some(&buffers[buffer.index()]));
                    if let Some(positions) = reader.read_positions() {
                        for p in positions {
                            // Process vertex positions (e.g., store them in a Vec)
                            println!("Position: {:?}", p);
                        }
                    }
                    if let Some(normals) = reader.read_normals() {
                        for n in normals {

                        }
                    }
                    if let Some(weights) = reader.read_weights(0) {
                        for w in weights.into_f32() {

                        }
                    }
                }
                TriangleStrip => {}
                TriangleFan => {}
            }
        }
    }

    // Recursively visit all children
    for child in node.children() {
        traverse_node_recursive(&child, meshes, materials, buffers);
    }
}

fn run(path: &str) {
    let file = fs::File::open(path)?;
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
        traverse_node_recursive(
            &r,
            &mut serialized_meshes,
            &mut serialized_materials,
            buffers,
        );
    }
}

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        run(&path);
    } else {
        println!("usage: gltf2msgpack <FILE>");
    }
}
