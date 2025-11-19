use bevy::{color::palettes::css::WHITE, pbr::light_consts::lux::AMBIENT_DAYLIGHT, prelude::*};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_sky_gradient::{
    cycle::{
        SkyColorsBuilder, SkyCyclePlugin, SkyTimeSettings, StopsColors, SunDriverPlugin,
        SunDriverTag,
    },
    gradient_material::SkyGradientMaterial,
    plugin::SkyGradientPlugin,
};

use bevy_inspector_egui::{bevy_egui::EguiPlugin, egui::Color32, quick::AssetInspectorPlugin};

// This example show the customization of the Cycle
// here we manually spawn: SKYBOX, and our sun light
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // egui
        .add_plugins(EguiPlugin::default())
        .add_plugins(AssetInspectorPlugin::<SkyGradientMaterial>::default())
        // camera
        .add_plugins(NoCameraPlayerPlugin)
        // skygradient plugin
        .add_plugins(SkyGradientPlugin {
            // WE WILL spawn the skybox manually
            spawn_default_skybox: false,
        })
        // cycle plugins
        .add_plugins(SkyCyclePlugin {
            sky_time_settings: SkyTimeSettings {
                day_time_sec: 10.0,
                night_time_sec: 20.0,
                sunrise_time_sec: 2.0,
                sunset_time_sec: 2.0,
            },
            // THIS WILL CREATE A SOLID COLOR, because the stops are the same
            // except for the day high, which we make very wonky
            sky_colors_builder: SkyColorsBuilder {
                sunset_color: StopsColors::new_splat(Color32::RED),
                sunrise_color: StopsColors::new_splat(Color32::RED),
                day_low_color: StopsColors::new_splat(Color32::LIGHT_BLUE),
                day_high_color: StopsColors {
                    stop0: Color32::from_rgb(0, 255, 0),
                    stop1: Color32::from_rgb(255, 0, 0),
                    stop2: Color32::from_rgb(0, 0, 255),
                    stop3: Color32::from_rgb(0, 255, 255),
                },
                night_low_color: StopsColors::new_splat(Color32::DARK_BLUE),
                night_high_color: StopsColors::new_splat(Color32::BLACK),
            },
        })
        .add_plugins(SunDriverPlugin {
            // WE WILL spawn the suns light directional light manually
            spawn_default_sun: false,
        })
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut sky_materials: ResMut<Assets<SkyGradientMaterial>>,
) {
    // MANUAL SKYBOX CREATION, using a cuboid mesh instead of Sphere
    let mut mesh = Cuboid::from_length(1.0).mesh().build();
    bevy_sky_gradient::utils::flip_mesh_normals(&mut mesh);
    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(sky_materials.add(SkyGradientMaterial::default())),
    ));

    // MANUAL SUN LIGHT SOURCE creation
    commands.spawn((
        // The driver tag is required
        SunDriverTag,
        DirectionalLight {
            color: WHITE.into(),
            illuminance: AMBIENT_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        Transform::default(),
    ));

    // spawn a flat circular base.
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(3.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-0.4, 0.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        FlyCam,
    ));
}
