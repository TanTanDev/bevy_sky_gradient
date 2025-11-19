use bevy::prelude::*;

pub fn default_sky_mesh() -> Mesh {
    // safety: unwrap is okay, because ico shape only fails
    // if there are to many vertices
    let mut mesh = Sphere::new(1.0).mesh().ico(0).unwrap();
    flip_mesh_normals(&mut mesh);
    mesh
}

pub fn flip_mesh_normals(mesh: &mut Mesh) {
    if let Some(normals) = mesh.attribute_mut(Mesh::ATTRIBUTE_NORMAL) {
        if let bevy::render::mesh::VertexAttributeValues::Float32x3(values) = normals {
            for n in values.iter_mut() {
                n[0] = -n[0];
                n[1] = -n[1];
                n[2] = -n[2];
            }
        }
    }

    if let Some(indices) = mesh.indices_mut() {
        use bevy::render::mesh::Indices;
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
