use bevy::prelude::*;
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_sky_gradient::{gradient_material::SkyGradientMaterial, plugin::SkyGradientPlugin};

use bevy_inspector_egui::{
    bevy_egui::EguiPlugin,
    quick::{AssetInspectorPlugin, WorldInspectorPlugin},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(AssetInspectorPlugin::<SkyGradientMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, sky_follow_camera)
        .add_plugins(NoCameraPlayerPlugin)
        .add_plugins(SkyGradientPlugin::default())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut sky_materials: ResMut<Assets<SkyGradientMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut mesh = Sphere::new(1.0).mesh().ico(8).unwrap();
    bevy_sky_gradient::utils::flip_mesh_normals(&mut mesh);

    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(sky_materials.add(SkyGradientMaterial {
            color_stops: [
                Vec4::new(0.2, 0.3, 0.6, 1.0),
                Vec4::new(0.4, 0.5, 1.0, 1.0),
                Vec4::new(0.35, 0.6, 0.8, 1.0),
                Vec4::new(0.5, 0.7, 1.0, 1.0),
            ],
            positions: Vec4::new(0.0, 0.43, 0.51, 1.0),
            // positions: Vec4::new(0.0, 0.3, 0.6, 1.0),
            num_stops: 4,
            // east
            // sun_dir: Vec3::new(0.0, 0.1, 1.0),
            sun_dir: Vec3::new(0.0, 0.1, -1.0),
            sun_color: Vec4::new(1.0, 1.0, 0.2, 1.0),
            sun_strength: 1.5,
            sun_sharpness: 164.0,
            night_time_distance: 0.0,
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-0.4, 0.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        FlyCam,
    ));
}
fn sky_follow_camera(
    camera_query: Query<&Transform, With<Camera>>,
    mut sky_query: Query<
        &mut Transform,
        (Without<Camera>, With<MeshMaterial3d<SkyGradientMaterial>>),
    >,
) {
    if let Ok(cam_tf) = camera_query.single() {
        for mut tf in &mut sky_query {
            tf.translation = cam_tf.translation;
        }
    }
}
