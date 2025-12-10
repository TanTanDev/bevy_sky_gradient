use bevy::{
    prelude::*,
    render::{
        render_resource::{AsBindGroup, ShaderRef, ShaderType},
        view::RenderLayers,
    },
};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_inspector_egui::{
    bevy_egui::{EguiGlobalSettings, EguiPlugin, PrimaryEguiContext},
    quick::WorldInspectorPlugin,
};
use bevy_sky_gradient::plugin::{GradientTextureHandle, SkyPlugin, SkyboxMagnetTag};
use rand::Rng;

// this example illustrates, how you can correctly blend the skycolor into your shaders
// by rendering the sky to a texture, then sampling that in your custom shader.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, setup_egui_render_layer))
        .add_plugins((EguiPlugin::default(), WorldInspectorPlugin::default()))
        .add_plugins(NoCameraPlayerPlugin)
        .add_plugins(MaterialPlugin::<FogMaterial>::default())
        .add_systems(Update, force_material_update)
        .add_plugins(SkyPlugin::default())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut fog_materials: ResMut<Assets<FogMaterial>>,
    gradient_texture: Res<GradientTextureHandle>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(90.0))),
        MeshMaterial3d(fog_materials.add(FogMaterial {
            settings: FogSettings {
                color: vec3(0.0, 1.0, 0.0),
                ..default()
            },
            sky_texture: gradient_texture.render_target.clone(),
        })),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    let mut rng = rand::rng();
    for _ in 0..100 {
        let x = rng.random_range(-80.0..80.0);
        let z = rng.random_range(-80.0..80.0);
        let scale = rng.random_range(1.0..8.0);
        // cube
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(fog_materials.add(FogMaterial {
                settings: FogSettings {
                    color: vec3(1.0, 0.0, 0.0),
                    ..default()
                },
                sky_texture: gradient_texture.render_target.clone(),
            })),
            Transform::from_xyz(x, scale * 0.5, z).with_scale(Vec3::splat(scale)),
        ));
    }

    // camera
    commands.spawn((
        Name::new("game primary camera"),
        // tell SkyPlugin we want the skybox centered on this camera
        SkyboxMagnetTag,
        Camera3d::default(),
        Transform::from_xyz(-0.4, 0.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        FlyCam,
    ));
}

#[derive(Clone, Copy, Debug, PartialEq, ShaderType, Component, Reflect)]
pub struct FogSettings {
    pub color: Vec3,
    pub distance_start: f32,
    pub distance_end: f32,
}

impl Default for FogSettings {
    fn default() -> Self {
        Self {
            color: vec3(1.0, 1.0, 1.0),
            distance_start: 40.0,
            distance_end: 80.0,
        }
    }
}
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct FogMaterial {
    #[uniform(0)]
    pub settings: FogSettings,
    #[texture(1, dimension = "2d")]
    #[sampler(2)]
    pub sky_texture: Handle<Image>,
}

impl Material for FogMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fog.wgsl".into()
    }
}

// egui by default renders into the first camera it finds.
// which happends to be our AuroraCamera lmao.
// this ensures egui doesn't render onto our aurora. disable for some fun :)
fn setup_egui_render_layer(
    mut commands: Commands,
    mut egui_global_settings: ResMut<EguiGlobalSettings>,
) {
    egui_global_settings.auto_create_primary_context = false;
    commands.spawn((
        Name::new("camera_egui_fix"),
        PrimaryEguiContext,
        Camera3d::default(),
        Camera {
            order: 3,
            ..default()
        },
        RenderLayers::none(),
    ));
}

// HACK: BEVY acts weird
// discussion: https://github.com/bevyengine/bevy/issues/16159
fn force_material_update(
    mut materials: ResMut<Assets<FogMaterial>>,
    query: Query<&MeshMaterial3d<FogMaterial>>,
) {
    // If the sky state changed or the image was resized this frame:
    for handle in query.iter() {
        if let Some(_material) = materials.get_mut(handle) {
            // This operation *should* force Bevy to re-prepare the material's bind group
            // and re-evaluate its texture view dependency.
            // The actual bug is on the Camera's side, but this is the user workaround.
            materials.get_mut(handle).map(|_| ()); // Get mutable reference to flag change
        }
    }
}
