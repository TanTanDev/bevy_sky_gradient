use bevy::{color::palettes::css::WHITE, pbr::light_consts::lux::AMBIENT_DAYLIGHT, prelude::*};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_sky_gradient::{
    cycle::{SkyCyclePlugin, SkyTime, SkyTimeSettings},
    gradient::{GradientBuilder, SkyGradientBuilder},
    gradient_driver::GradientDriverPlugin,
    noise::NoiseHandles,
    plugin::{AuroraTextureHandle, GradientTextureHandle, SkyPlugin, SkyboxMagnetTag},
    sky_material::FullSkyMaterial,
    sun::{SunDriverPlugin, SunDriverTag, SunSettings},
};

// This example show how you can customize any aspect of the sky
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
                .set_gradient_driver(GradientDriverPlugin {
                    sky_colors_builder: CUSTOM_SKY_COLORS_BUILDER,
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
    gradient_handle: Res<GradientTextureHandle>,
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
            gradient_image: gradient_handle.render_target.clone(),
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

pub const CUSTOM_SKY_COLORS_BUILDER: SkyGradientBuilder = SkyGradientBuilder {
    gradient_builder_stop0: GradientBuilder {
        sunrise_color: [255, 0, 0, 255],
        day_low_color: [0, 0, 248, 255],
        day_high_color: [0, 48, 255, 255],
        sunset_color: [255, 70, 70, 255],
        night_low_color: [0, 0, 0, 245],
        night_high_color: [0, 0, 0, 245],
    },
    gradient_builder_stop1: GradientBuilder {
        sunrise_color: [255, 0, 0, 255],
        day_low_color: [0, 0, 255, 255],
        day_high_color: [0, 226, 255, 255],
        sunset_color: [243, 84, 47, 255],
        night_low_color: [0, 0, 0, 245],
        night_high_color: [0, 0, 0, 245],
    },
    gradient_builder_stop2: GradientBuilder {
        sunrise_color: [255, 0, 0, 255],
        day_low_color: [0, 0, 254, 255],
        day_high_color: [0, 170, 255, 255],
        sunset_color: [255, 242, 72, 255],
        night_low_color: [0, 0, 0, 245],
        night_high_color: [0, 0, 0, 245],
    },
    gradient_builder_stop3: GradientBuilder {
        sunrise_color: [255, 0, 0, 255],
        day_low_color: [0, 0, 255, 255],
        day_high_color: [0, 195, 255, 255],
        sunset_color: [73, 177, 250, 255],
        night_low_color: [0, 0, 0, 245],
        night_high_color: [0, 0, 0, 245],
    },
};
