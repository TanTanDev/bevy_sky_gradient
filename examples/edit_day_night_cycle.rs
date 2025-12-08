use bevy::{prelude::*, render::view::RenderLayers};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_sky_gradient::{
    aurora::AuroraSettings,
    aurora_material::AuroraMaterial,
    cycle::{SkyCyclePlugin, SkyTime, SkyTimeSettings},
    gradient::SkyColors,
    noise::NoiseSettings,
    plugin::{SkyPlugin, SkyboxMagnetTag},
    sky_material::FullSkyMaterial,
    sun::{SunDriverTag, SunSettings},
};

use bevy_inspector_egui::{
    bevy_egui::{
        self, EguiContext, EguiGlobalSettings, EguiPlugin, EguiPrimaryContextPass,
        PrimaryEguiContext,
    },
    bevy_inspector::ui_for_resource,
    egui,
    quick::{AssetInspectorPlugin, ResourceInspectorPlugin},
};
use egui_colorgradient::gradient_editor;

// this example showcase live tweaking via egui ui.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // egui
        .add_plugins(EguiPlugin::default())
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(AssetInspectorPlugin::<FullSkyMaterial>::default())
        .add_plugins(AssetInspectorPlugin::<AuroraMaterial>::default())
        .add_plugins(ResourceInspectorPlugin::<AuroraSettings>::default())
        .add_plugins(ResourceInspectorPlugin::<NoiseSettings>::default())
        .add_plugins(ResourceInspectorPlugin::<SkyTimeSettings>::default())
        // camera
        .add_plugins(NoCameraPlayerPlugin)
        // SKY plugin
        .add_plugins(
            SkyPlugin::builder_all_features()
                .with_noise_settings(NoiseSettings {
                    noise_texture_size: 128,
                    voronoi_texture_size: 128,
                })
                .set_cycle(SkyCyclePlugin {
                    sky_time_settings: SkyTimeSettings::default(),
                    sky_time: SkyTime {
                        // start by night because it looks lovely
                        time: 14.0,
                        auto_tick: true,
                    },
                })
                .build(),
        )
        .add_systems(EguiPrimaryContextPass, edit_ui)
        .add_systems(Startup, (setup, setup_egui_render_layer))
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
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
            .with_translation(Vec3::new(0.0, -2.0, 0.0)),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        // tell SkyPlugin we want the skybox centered on this camera
        SkyboxMagnetTag,
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

// egui by default renders into the first camera it finds.
// which happends to be our AuroraCamera lmao.
// this ensures egui doesn't render onto our aurora. disable for some fun :)
fn setup_egui_render_layer(
    mut commands: Commands,
    mut egui_global_settings: ResMut<EguiGlobalSettings>,
) {
    egui_global_settings.auto_create_primary_context = false;
    commands.spawn((
        PrimaryEguiContext,
        Camera3d::default(),
        Camera {
            order: 1,
            ..default()
        },
        RenderLayers::none(),
    ));
}
