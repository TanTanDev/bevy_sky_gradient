use bevy::prelude::*;
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_gradient_sky::{
    cycle::{SkyColors, SkyCyclePlugin, SkyTime, SunDriverPlugin, SunDriverTag, SunSettings},
    gradient_material::SkyGradientMaterial,
    plugin::SkyGradientPlugin,
};

use bevy_inspector_egui::{
    bevy_egui::{self, EguiContext, EguiPlugin, EguiPrimaryContextPass},
    bevy_inspector::ui_for_resource,
    egui,
    quick::AssetInspectorPlugin,
};
use egui_colorgradient::gradient_editor;

// this example showcase live tweaking via egui ui.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // egui
        .add_plugins(EguiPlugin::default())
        .add_plugins(AssetInspectorPlugin::<SkyGradientMaterial>::default())
        // camera
        .add_plugins(NoCameraPlayerPlugin)
        // SKY PLUGINS
        .add_plugins(SkyGradientPlugin::default()) // spawns skybox
        .add_plugins(SkyCyclePlugin::default()) // drives skybox colors in cycles
        .add_plugins(SunDriverPlugin::default()) // spawns sun
        .add_systems(EguiPrimaryContextPass, edit_ui)
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

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-0.4, 0.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        FlyCam,
    ));
}

fn edit_ui(mut world: &mut World) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<bevy_egui::PrimaryEguiContext>>()
        .single(world)
        .expect("EguiContext not found")
        .clone();

    egui::Window::new("colors").show(egui_context.get_mut(), |ui| {
        let mut sky_colors = world.get_resource_mut::<SkyColors>().unwrap();
        ui.push_id("c1", |ui| {
            gradient_editor(ui, &mut sky_colors.sky_color0);
        });
        ui.push_id("c2", |ui| {
            gradient_editor(ui, &mut sky_colors.sky_color1);
        });
        ui.push_id("c3", |ui| {
            gradient_editor(ui, &mut sky_colors.sky_color2);
        });
        ui.push_id("c4", |ui| {
            gradient_editor(ui, &mut sky_colors.sky_color3);
        });
    });

    egui::Window::new("sky settings").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
            ui.label("sky time");
            ui_for_resource::<SkyTime>(world, ui);
            ui_for_resource::<SunSettings>(world, ui);
            let mut dirlight = world
                .query_filtered::<&mut DirectionalLight, With<SunDriverTag>>()
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
