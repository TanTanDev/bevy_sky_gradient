use bevy::prelude::*;

use crate::{gradient_material::SkyGradientMaterial, utils};

///! sets up all you need to show a gradient skybox
pub struct SkyGradientPlugin {
    ///! if true, an entity skybox will spawn
    pub spawn_default_skybox: bool,
}

impl Plugin for SkyGradientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, crate::assets::initialize_shaders);
        app.add_plugins(MaterialPlugin::<SkyGradientMaterial>::default());
        if self.spawn_default_skybox {
            app.add_systems(Startup, spawn_default_skybox);
        }
        app.add_systems(
            PostUpdate,
            sky_follow_camera.before(TransformSystem::TransformPropagate),
        );
    }
}

impl Default for SkyGradientPlugin {
    fn default() -> Self {
        Self {
            spawn_default_skybox: true,
        }
    }
}

fn spawn_default_skybox(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut sky_materials: ResMut<Assets<SkyGradientMaterial>>,
) {
    commands.spawn((
        Name::new("sky_gradient_skybox"),
        Mesh3d(meshes.add(utils::default_sky_mesh())),
        MeshMaterial3d(sky_materials.add(SkyGradientMaterial::default())),
    ));
}

fn sky_follow_camera(
    camera_query: Query<(&Transform, &Camera)>,
    mut sky_query: Query<
        &mut Transform,
        (Without<Camera>, With<MeshMaterial3d<SkyGradientMaterial>>),
    >,
) {
    // find active camera
    for (cam_tf, _camera) in camera_query.iter().filter(|cam| cam.1.is_active) {
        for mut tf in &mut sky_query {
            tf.translation = cam_tf.translation;
        }
    }
}
