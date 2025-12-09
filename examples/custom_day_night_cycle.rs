use bevy::{color::palettes::css::WHITE, pbr::light_consts::lux::AMBIENT_DAYLIGHT, prelude::*};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_inspector_egui::egui::Color32;
use bevy_sky_gradient::{
    cycle::{SkyCyclePlugin, SkyTime, SkyTimeSettings},
    gradient::{GradientDriverPlugin, SkyColorsBuilder, StopsColors},
    noise::NoiseHandles,
    plugin::{AuroraTextureHandle, SkyPlugin, SkyboxMagnetTag},
    sky_material::FullSkyMaterial,
    sun::{SunDriverPlugin, SunDriverTag, SunSettings},
};

// This example show how you can customize in depth
// here we manually spawn: skybox, and our sun light
// we also configure the cycle timings (long night)
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NoCameraPlayerPlugin)
        // SKY
        .add_plugins(
            SkyPlugin::builder_all_features()
                .set_spawn_default_skybox(false)
                .set_cycle(SkyCyclePlugin {
                    sky_time_settings: SkyTimeSettings {
                        day_time_sec: 3.0,
                        night_time_sec: 3.0,
                        sunrise_time_sec: 0.2,
                        sunset_time_sec: 0.2,
                    },
                    sky_time: SkyTime::default(),
                })
                .set_sun_driver(SunDriverPlugin {
                    spawn_default_sun_light: false,
                    sun_settings: SunSettings {
                        illuminance: 10000.0,
                        sun_color: vec4(1.0, 1.0, 0.0, 1.0),
                        sun_strength: default(),
                        sun_sharpness: default(),
                    },
                })
                // THIS WILL CREATE A SOLID COLOR, because the stops are the same
                // except for the day high, which we make very wonky
                .set_gradient_driver(GradientDriverPlugin {
                    sky_colors_builder: SkyColorsBuilder {
                        sunset_color: StopsColors::new_splat(Color32::RED),
                        sunrise_color: StopsColors::new_splat(Color32::RED),
                        day_low_color: StopsColors::new_splat(Color32::LIGHT_BLUE),
                        day_high_color: StopsColors {
                            stop0: [0, 255, 0, 255],
                            stop1: [255, 0, 0, 255],
                            stop2: [0, 0, 255, 255],
                            stop3: [0, 255, 255, 255],
                        },
                        night_low_color: StopsColors::new_splat(Color32::DARK_BLUE),
                        night_high_color: StopsColors::new_splat(Color32::BLACK),
                    },
                })
                .build(),
        )
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut sky_materials: ResMut<Assets<FullSkyMaterial>>,
    noise_handles: Res<NoiseHandles>,
    aurora_handle: Res<AuroraTextureHandle>,
) {
    // MANUAL SKYBOX CREATION, using a cuboid mesh instead of Sphere, because we can.
    let mut mesh = Cuboid::from_length(1.0).mesh().build();
    bevy_sky_gradient::utils::flip_mesh_normals(&mut mesh);
    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        // if you manually create the sky mesh...
        // you still need valid texture handles, so we can fetch that.
        MeshMaterial3d(sky_materials.add(FullSkyMaterial {
            noise3_image: noise_handles.noise3.clone(),
            voronoi3_image: noise_handles.voronoi3.clone(),
            aurora_image: aurora_handle.render_target.clone(),
            ..default()
        })),
    ));

    // MANUAL SUN LIGHT SOURCE creation
    commands.spawn((
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
        // tell SkyPlugin we want the skybox centered on this camera
        SkyboxMagnetTag,
        Camera3d::default(),
        Transform::from_xyz(-0.4, 0.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        FlyCam,
    ));
}
