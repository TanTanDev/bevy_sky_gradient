use bevy::prelude::*;
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_sky_gradient::{
    cycle::{SkyCyclePlugin, SunDriverPlugin},
    plugin::SkyGradientPlugin,
    sky_material::FullSkyMaterial,
};

use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::AssetInspectorPlugin};

// this example showcase our default cycle implementation
// check out custom_day_night_cycle.rs for customization
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // egui
        .add_plugins(EguiPlugin::default())
        .add_plugins(AssetInspectorPlugin::<FullSkyMaterial>::default())
        // camera
        .add_plugins(NoCameraPlayerPlugin)
        // SKY PLUGINS
        .add_plugins(SkyGradientPlugin::default()) // spawns skybox
        .add_plugins(SkyCyclePlugin::default()) // drives skybox colors in cycles
        .add_plugins(SunDriverPlugin::default()) // spawns sun
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // spawn a flat circular base.
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(3.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-0.4, 0.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        FlyCam,
    ));
}
