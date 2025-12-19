use bevy::{camera::visibility::RenderLayers, prelude::*};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_sky_gradient::{
    ambient_driver::AmbientColorsBuilder, aurora_material::AuroraMaterial,
    gradient_material::FullGradientMaterial, prelude::*, sky_material::FullSkyMaterial,
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

#[cfg(feature = "serde")]
use ron::ser::PrettyConfig;

// this example showcase live tweaking via egui ui.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // egui
        .add_plugins(EguiPlugin::default())
        .add_plugins(AssetInspectorPlugin::<FullSkyMaterial>::default())
        .add_plugins(AssetInspectorPlugin::<FullGradientMaterial>::default())
        .add_plugins(AssetInspectorPlugin::<AuroraMaterial>::default())
        .add_plugins(ResourceInspectorPlugin::<AuroraSettings>::default())
        .add_plugins(ResourceInspectorPlugin::<NoiseSettings>::default())
        .add_plugins(ResourceInspectorPlugin::<SkyTimeSettings>::default())
        .add_plugins(ResourceInspectorPlugin::<AmbientSettings>::default())
        .add_plugins(ResourceInspectorPlugin::<AmbientColorsBuilder>::default())
        // camera
        .add_plugins(NoCameraPlayerPlugin)
        // SKY plugin
        .add_plugins(
            SkyPlugin::builder_all_features()
                .with_noise_settings(NoiseSettings {
                    noise_texture_size: 128,
                    voronoi_texture_size: 128,
                    noise_size_limit: Some(256),
                    // because this example showcase live tweaking...
                    // I disable noise texture file cashing, because tweaking the noise size
                    // would generate hundreds of files for various texture settings like:
                    // noise_textures_{1,2,3,...}_{1,2,3...}.ron
                    // realistically, in a complete project, this would be enabled and you would force specific resolutions.
                    // like: 32, 64, 128, 256. probably hidden under a NoiseQuality enum
                    #[cfg(feature = "serde")]
                    cache_textures_locally: false,
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

// helper ui things to save our sky look to SkyPreset files
#[cfg(feature = "serde")]
#[derive(Resource)]
struct SkyPresetFileName(String);
#[cfg(feature = "serde")]
#[derive(Resource)]
struct SkyPresetResult(String);

fn edit_ui(mut world: &mut World) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<bevy_egui::PrimaryEguiContext>>()
        .single(world)
        .expect("EguiContext not found")
        .clone();

    #[cfg(feature = "serde")]
    show_save_load_preset_uis(world, &mut egui_context);

    egui::Window::new("sky settings").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
            ui.label("sky time");
            ui.push_id("skytime", |ui| {
                ui_for_resource::<SkyTime>(world, ui);
            });
            ui.push_id("sunsettings", |ui| {
                ui_for_resource::<SunSettings>(world, ui);
            });
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

#[cfg(feature = "serde")]
fn show_save_load_preset_uis(world: &mut World, egui_context: &mut EguiContext) {
    use bevy_sky_gradient::gradient::SkyGradientBuilder;
    use bevy_sky_gradient::presets::{ApplyPresetEvent, SkyPreset};
    use bevy_sky_gradient::utils::path_relative_to_bevy_exe;

    egui::Window::new("SAVE PRESET").show(egui_context.get_mut(), |ui| {
        let _ = world.get_resource_or_insert_with(|| SkyPresetResult(String::default()));
        let mut file_name =
            world.get_resource_or_insert_with(|| SkyPresetFileName(String::default()));
        ui.label("file name:");
        ui.text_edit_singleline(&mut file_name.0);
        let sky_preset_folder = "assets/sky_presets/";
        let sky_preset_folder = path_relative_to_bevy_exe(sky_preset_folder);
        if ui.button("save").clicked() {
            if file_name.0.is_empty() {
                return;
            }

            let file_name = file_name.0.clone();
            // fetch the bind group from the aurora
            let all_aurora_material = world.get_resource::<Assets<AuroraMaterial>>().unwrap();
            let current_aurora_material = all_aurora_material.iter().next().unwrap().1;
            let all_gradient_material = world
                .get_resource::<Assets<FullGradientMaterial>>()
                .unwrap();
            let current_gradient_material = all_gradient_material.iter().next().unwrap().1;

            let all_sky_materials = world.get_resource::<Assets<FullSkyMaterial>>().unwrap();
            let current_sky_material = all_sky_materials.iter().next().unwrap().1;

            let sun_settings = world.get_resource::<SunSettings>().unwrap();
            let sky_colors_builder = world.get_resource::<SkyGradientBuilder>().unwrap();
            // fetch the sky information
            let sky_preset = SkyPreset {
                aurora_settings: Some(current_aurora_material.aurora_settings.clone()),
                sun_settings: Some(sun_settings.clone()),
                sky_colors_builder: Some(sky_colors_builder.clone()),
                stars: Some(current_sky_material.stars.clone()),
                gradient_bind_group: Some(current_gradient_material.gradient_bind_group.clone()),
            };
            let sky_preset = ron::ser::to_string_pretty(&sky_preset, PrettyConfig::default());
            let sky_preset = sky_preset.unwrap();

            let mut result = world.get_resource_mut::<SkyPresetResult>().unwrap();
            if let Err(err) = std::fs::create_dir_all(&sky_preset_folder) {
                result.0 = format!(
                    "err, failed to make folder: {:?}, error: {:?}",
                    sky_preset_folder, err
                );
                return;
            }
            let path = format!("{}{}.ron", sky_preset_folder.to_string_lossy(), file_name);
            let save_result = std::fs::write(path, sky_preset);
            result.0 = format!("save result: {:?}", save_result);
        }

        if let Ok(read_dir) = std::fs::read_dir(sky_preset_folder) {
            ui.label("-- LOAD PRESET FILES --");
            for entry in read_dir.into_iter().flatten() {
                if ui.button(entry.file_name().to_string_lossy()).clicked() {
                    match std::fs::read(entry.path()) {
                        Ok(bytes) => match ron::de::from_bytes::<SkyPreset>(&bytes) {
                            Ok(preset) => {
                                world.write_message(ApplyPresetEvent { sky_preset: preset });
                            }
                            Err(err) => {
                                let mut result =
                                    world.get_resource_mut::<SkyPresetResult>().unwrap();
                                result.0 = format!("faield to deserialize file: {:?}", err);
                            }
                        },
                        Err(err) => {
                            let mut result = world.get_resource_mut::<SkyPresetResult>().unwrap();
                            result.0 = format!("failed to read file err: {:?}", err);
                        }
                    }
                }
            }
        } else {
            ui.label("no files inside assets/presets");
        }

        let result = world.get_resource_or_insert_with(|| SkyPresetResult(String::default()));
        ui.label(&result.0);
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
