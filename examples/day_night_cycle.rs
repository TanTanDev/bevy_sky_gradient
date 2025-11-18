use std::time::Duration;

use bevy::{
    color::palettes::css::RED, core_pipeline::bloom::Bloom,
    pbr::light_consts::lux::AMBIENT_DAYLIGHT, prelude::*,
};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use egui_colorgradient::{Gradient, InterpolationMethod, gradient_editor};
use gradient_sky::{gradient_material::SkyGradientMaterial, plugin::SkyGradientPlugin};

use bevy_inspector_egui::{
    bevy_egui::{self, EguiContext, EguiPlugin, EguiPrimaryContextPass},
    bevy_inspector::ui_for_resource,
    egui::{self, Color32},
    quick::{AssetInspectorPlugin, WorldInspectorPlugin},
};

// pub const DAY_TIME_SEC: f32 = 60.0;
pub const DAY_TIME_SEC: f32 = 12.0;
pub const NIGHT_TIME_SEC: f32 = 12.0;
pub const CYCLE_TIME: f32 = DAY_TIME_SEC + NIGHT_TIME_SEC;

pub const SUNRISE_TIME_SEC: f32 = 3.0;
pub const SUNSET_TIME_SEC: f32 = 3.0;

pub const SUNRISE_TIME_PERCENT_DAY: f32 = SUNRISE_TIME_SEC / DAY_TIME_SEC;
pub const SUNRISE_TIME_PERCENT_NIGHT: f32 = SUNRISE_TIME_SEC / NIGHT_TIME_SEC;
pub const SUNSET_TIME_PERCENT_DAY: f32 = SUNSET_TIME_SEC / DAY_TIME_SEC;
pub const SUNSET_TIME_PERCENT_NIGHT: f32 = SUNSET_TIME_SEC / NIGHT_TIME_SEC;

pub const SUNSET_COLOR_0: Color32 = Color32::from_rgb(255, 70, 70);
pub const SUNSET_COLOR_1: Color32 = Color32::from_rgb(243, 84, 47);
pub const SUNSET_COLOR_2: Color32 = Color32::from_rgb(255, 242, 72);
pub const SUNSET_COLOR_3: Color32 = Color32::from_rgb(73, 177, 250);

pub const DAY_LOW_COLOR_0: Color32 = Color32::from_rgb(157, 157, 248);
pub const DAY_LOW_COLOR_1: Color32 = Color32::from_rgb(205, 242, 255);
pub const DAY_LOW_COLOR_2: Color32 = Color32::from_rgb(182, 200, 254);
pub const DAY_LOW_COLOR_3: Color32 = Color32::from_rgb(224, 224, 255);

pub const DAY_HIGH_COLOR_0: Color32 = Color32::from_rgb(48, 48, 255);
pub const DAY_HIGH_COLOR_1: Color32 = Color32::from_rgb(0, 226, 255);
pub const DAY_HIGH_COLOR_2: Color32 = Color32::from_rgb(0, 170, 255);
pub const DAY_HIGH_COLOR_3: Color32 = Color32::from_rgb(66, 195, 255);

pub const NIGHT_LOW_COLOR_0: Color32 = Color32::from_rgb(0, 3, 40);
pub const NIGHT_LOW_COLOR_1: Color32 = Color32::from_rgb(47, 0, 93);
pub const NIGHT_LOW_COLOR_2: Color32 = Color32::from_rgb(0, 38, 97);
pub const NIGHT_LOW_COLOR_3: Color32 = Color32::from_rgb(74, 0, 89);

pub const NIGHT_HIGH_COLOR_0: Color32 = Color32::from_rgb(0, 0, 45);
pub const NIGHT_HIGH_COLOR_1: Color32 = Color32::from_rgb(0, 32, 93);
pub const NIGHT_HIGH_COLOR_2: Color32 = Color32::from_rgb(0, 0, 112);
pub const NIGHT_HIGH_COLOR_3: Color32 = Color32::from_rgb(0, 0, 43);

pub const SUNRISE_COLOR_0: Color32 = Color32::from_rgb(255, 70, 70);
pub const SUNRISE_COLOR_1: Color32 = Color32::from_rgb(243, 84, 47);
pub const SUNRISE_COLOR_2: Color32 = Color32::from_rgb(255, 242, 72);
pub const SUNRISE_COLOR_3: Color32 = Color32::from_rgb(73, 177, 250);

fn sunset_color(pos: i32) -> Color32 {
    match pos {
        0 => SUNSET_COLOR_0,
        1 => SUNSET_COLOR_1,
        2 => SUNSET_COLOR_2,
        _ => SUNSET_COLOR_3,
    }
}
fn sunrise_color(pos: i32) -> Color32 {
    match pos {
        0 => SUNRISE_COLOR_0,
        1 => SUNRISE_COLOR_1,
        2 => SUNRISE_COLOR_2,
        _ => SUNRISE_COLOR_3,
    }
}
fn day_low_color(pos: i32) -> Color32 {
    match pos {
        0 => DAY_LOW_COLOR_0,
        1 => DAY_LOW_COLOR_1,
        2 => DAY_LOW_COLOR_2,
        _ => DAY_LOW_COLOR_3,
    }
}
fn day_high_color(pos: i32) -> Color32 {
    match pos {
        0 => DAY_HIGH_COLOR_0,
        1 => DAY_HIGH_COLOR_1,
        2 => DAY_HIGH_COLOR_2,
        _ => DAY_HIGH_COLOR_3,
    }
}
fn night_low_color(pos: i32) -> Color32 {
    match pos {
        0 => NIGHT_LOW_COLOR_0,
        1 => NIGHT_LOW_COLOR_1,
        2 => NIGHT_LOW_COLOR_2,
        _ => NIGHT_LOW_COLOR_3,
    }
}
fn night_high_color(pos: i32) -> Color32 {
    match pos {
        0 => NIGHT_HIGH_COLOR_0,
        1 => NIGHT_HIGH_COLOR_1,
        2 => NIGHT_HIGH_COLOR_2,
        _ => NIGHT_HIGH_COLOR_3,
    }
}

// 0.0   NIGHT_COLOR
// SUNRISE_TIME/2 SUNRISE_COLOR
// SUNRISE_TIME/2..SUNRISE_TIME   DAY COLOR
// 0.5-SUNSET_TIME DAY_COLOR
// 0.5-SUNSET_TIME/2 SUNSET_COLOR
// 0.5 NIGHT_COLOR
// 1.0: NIGHT_COLOR

// sunrise color, ORANGE, YELLOW, BLUE
// day color, light blue
// night color, dark blue
// sunset color, RED into ORANGE into PINK into blue

fn default_gradient(pos: i32) -> Gradient {
    Gradient::new(
        InterpolationMethod::Linear,
        vec![
            (0.0, sunrise_color(pos)),
            (SUNRISE_TIME_PERCENT_DAY * 0.5, day_low_color(pos)),
            (
                (SUNRISE_TIME_PERCENT_DAY * 0.5 + 0.5 - SUNSET_TIME_PERCENT_DAY * 0.5) * 0.5,
                day_high_color(pos),
            ),
            (0.5 - SUNSET_TIME_PERCENT_DAY * 0.5, day_low_color(pos)),
            (0.5, sunset_color(pos)),
            (0.5 + SUNSET_TIME_PERCENT_NIGHT * 0.5, night_low_color(pos)),
            (
                ((0.5 + SUNSET_TIME_PERCENT_NIGHT * 0.5)
                    + (1.0 - SUNRISE_TIME_PERCENT_NIGHT * 0.5))
                    * 0.5,
                night_high_color(pos),
            ),
            (1.0 - SUNRISE_TIME_PERCENT_NIGHT * 0.5, night_low_color(pos)),
            (1.0, sunrise_color(pos)),
        ],
    )
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(SunPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(EguiPrimaryContextPass, time_ui)
        .add_plugins(AssetInspectorPlugin::<SkyGradientMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, sky_follow_camera)
        .add_plugins(NoCameraPlayerPlugin)
        .add_plugins(SkyGradientPlugin::default())
        .insert_resource(SkyColors {
            color1: default_gradient(0),
            color2: default_gradient(1),
            color3: default_gradient(2),
            color4: default_gradient(3),
        })
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut sky_materials: ResMut<Assets<SkyGradientMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let mut mesh = Sphere::new(1.0).mesh().ico(8).unwrap();
    gradient_sky::utils::flip_mesh_normals(&mut mesh);

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
        DirectionalLight {
            color: RED.into(),
            illuminance: AMBIENT_DAYLIGHT,
            shadows_enabled: true,
            // affects_lightmapped_mesh_diffuse: todo!(),
            // shadow_depth_bias: todo!(),
            // shadow_normal_bias: todo!(),
            ..default()
        },
        Sun,
        Transform::from_xyz(0.0, 0.0, 0.0),
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
            positions: Vec4::new(0.38, 0.47, 0.61, 1.0),
            // positions: Vec4::new(0.0, 0.3, 0.6, 1.0),
            num_stops: 4,
            // east
            // sun_dir: Vec3::new(0.0, 0.1, 1.0),
            sun_dir: Vec3::new(0.0, 0.1, -1.0),
            sun_color: Vec4::new(1.0, 1.0, 0.5, 1.0),
            sun_strength: 1.5,
            sun_sharpness: 164.0,
            time_percent: 0.0,
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

///! current time of day
#[derive(Resource, Reflect)]
struct SkyTime(pub f32);

#[derive(Resource, Reflect)]
pub struct SunSettings {
    illuminance: f32,
    tick_enabled: bool,
}

// Marker for updating the position of the light, not needed unless we have multiple lights
#[derive(Component)]
pub struct Sun;

#[derive(Default)]
pub struct SunPlugin;

impl Plugin for SunPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SunSettings>();
        app.insert_resource(SkyTime(18f32));
        app.insert_resource(SunSettings {
            illuminance: AMBIENT_DAYLIGHT,
            tick_enabled: false,
        });
        app.add_systems(Update, (daylight_cycle, drive_sun).chain());
    }
}

#[derive(Resource)]
pub struct SkyColors {
    color1: Gradient,
    color2: Gradient,
    color3: Gradient,
    color4: Gradient,
}

fn drive_sun(
    query: Query<&mut Transform, With<Sun>>,
    skyboxes: Query<&mut MeshMaterial3d<SkyGradientMaterial>>,
    mut sky_materials: ResMut<Assets<SkyGradientMaterial>>,
    sky_colors: Res<SkyColors>,
    mut timer: ResMut<SkyTime>,
) {
    let Ok(skybox_material_handle) = skyboxes.single() else {
        panic!("0 or '>1' skyboxes");
    };
    let Some(skybox_material) = sky_materials.get_mut(skybox_material_handle) else {
        return;
    };
    let day = (timer.0 / DAY_TIME_SEC).min(1.0);
    let night = ((timer.0 - DAY_TIME_SEC) / NIGHT_TIME_SEC).max(0.0);
    let percent = (day + night) * 0.5;

    // 0.0..1..0.0
    skybox_material.time_percent = 1.0 - (night - 0.5).abs() * 2.0;

    skybox_material.color_stops[0] = sky_colors
        .color1
        .interpolator()
        .sample_at(percent)
        .unwrap()
        .to_array()
        .into();
    skybox_material.color_stops[1] = sky_colors
        .color2
        .interpolator()
        .sample_at(percent)
        .unwrap()
        .to_array()
        .into();
    skybox_material.color_stops[2] = sky_colors
        .color3
        .interpolator()
        .sample_at(percent)
        .unwrap()
        .to_array()
        .into();
    skybox_material.color_stops[3] = sky_colors
        .color4
        .interpolator()
        .sample_at(percent)
        .unwrap()
        .to_array()
        .into();
    for sun_transform in query.iter() {
        skybox_material.sun_dir = sun_transform.forward().as_vec3();
    }
}

fn daylight_cycle(
    mut suns: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
    mut timer: ResMut<SkyTime>,
    time: Res<Time>,
    sun_settings: Res<SunSettings>,
) {
    if sun_settings.tick_enabled {
        timer.0 += time.delta_secs();
        if timer.0 > CYCLE_TIME {
            timer.0 -= CYCLE_TIME;
        }
    }

    let day = (timer.0 / DAY_TIME_SEC).min(1.0);
    let night = ((timer.0 - DAY_TIME_SEC) / NIGHT_TIME_SEC).max(0.0);
    // let percent = (day + night) * 0.5;
    let time_rotation = day * std::f32::consts::PI + night * std::f32::consts::PI;

    for (mut light_trans, mut directional) in suns.iter_mut() {
        light_trans.rotation =
            Quat::from_rotation_x(time_rotation.sin().atan2(time_rotation.cos()));
        directional.illuminance = time_rotation.sin().max(0.0).powf(2.0) * sun_settings.illuminance;
    }
}

fn time_ui(mut world: &mut World) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<bevy_egui::PrimaryEguiContext>>()
        .single(world)
        .expect("EguiContext not found")
        .clone();

    egui::Window::new("colors").show(egui_context.get_mut(), |ui| {
        let mut sky_colors = world.get_resource_mut::<SkyColors>().unwrap();
        ui.push_id("c1", |ui| {
            gradient_editor(ui, &mut sky_colors.color1);
        });
        ui.push_id("c2", |ui| {
            gradient_editor(ui, &mut sky_colors.color2);
        });
        ui.push_id("c3", |ui| {
            gradient_editor(ui, &mut sky_colors.color3);
        });
        ui.push_id("c4", |ui| {
            gradient_editor(ui, &mut sky_colors.color4);
        });
    });

    egui::Window::new("sky settings").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
            ui.label("sky time");
            ui_for_resource::<SkyTime>(world, ui);
            ui_for_resource::<SunSettings>(world, ui);
            let mut dirlight = world
                .query_filtered::<&mut DirectionalLight, With<Sun>>()
                .single_mut(&mut world)
                .unwrap();
            let lin = dirlight.color.to_srgba();
            let mut lin = lin.to_f32_array_no_alpha();
            ui.label("sun color");
            if egui::widgets::color_picker::color_edit_button_rgb(ui, &mut lin).changed() {
                dirlight.color = Color::srgb_from_array(lin);
            }
            ui.label("ambient color");
            ui_for_resource::<AmbientLight>(world, ui);
        });
    });
}
