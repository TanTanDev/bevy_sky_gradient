use bevy::{
    mesh::{Indices, VertexAttributeValues},
    prelude::*,
};

pub fn default_sky_mesh() -> Mesh {
    let mut mesh = Cuboid::new(1.0, 1.0, 1.0).mesh().build();
    flip_mesh_normals(&mut mesh);
    mesh
}

pub fn flip_mesh_normals(mesh: &mut Mesh) {
    if let Some(normals) = mesh.attribute_mut(Mesh::ATTRIBUTE_NORMAL) {
        if let VertexAttributeValues::Float32x3(values) = normals {
            for n in values.iter_mut() {
                n[0] = -n[0];
                n[1] = -n[1];
                n[2] = -n[2];
            }
        }
    }

    if let Some(indices) = mesh.indices_mut() {
        match indices {
            Indices::U16(vec) => {
                for i in vec.chunks_exact_mut(3) {
                    i.swap(1, 2);
                }
            }
            Indices::U32(vec) => {
                for i in vec.chunks_exact_mut(3) {
                    i.swap(1, 2);
                }
            }
        }
    }
}

#[cfg(feature = "serde")]
pub fn path_relative_to_bevy_exe(path: &str) -> std::path::PathBuf {
    let current_dir = bevy::asset::io::file::FileAssetReader::get_base_path();
    let new_path = current_dir.join(path);
    new_path
}
