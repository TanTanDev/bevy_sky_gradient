use bevy::{
    asset::RenderAssetUsages,
    image::ImageSampler,
    prelude::*,
    render::{
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
        view::RenderLayers,
    },
    window::{PrimaryWindow, WindowResized},
};

use crate::{
    aurora_material::AuroraMaterial,
    noise::{NoiseHandles, setup_noise_texture},
    plugin::{AuroraTextureHandle, spawn_aurora_texture},
    utils,
};

#[derive(Component)]
pub struct AuroraCameraTag;

#[derive(Resource, Reflect, Clone)]
pub struct AuroraSettings {
    ///! controlls size of the render target of the aurora material
    ///! a value of 1.0: use 100% of the windows screen size. aka full quality.
    ///! a value of 0.5: will render the aurora 50% of the screen and be upscaled 200%
    pub render_texture_percent: f32,
}

impl Default for AuroraSettings {
    fn default() -> Self {
        Self {
            render_texture_percent: 0.33,
        }
    }
}

pub struct AuroraPlugin {
    pub aurora_settings: AuroraSettings,
}

impl Plugin for AuroraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.aurora_settings.clone());
        app.add_plugins(MaterialPlugin::<AuroraMaterial>::default());
        app.add_systems(
            PreStartup,
            spawn_aurora_skybox
                .after(spawn_aurora_texture)
                .after(setup_noise_texture),
        );
        app.add_systems(
            PostUpdate,
            (aurora_follow_camera, resize_aurora_on_window_change)
                .before(TransformSystem::TransformPropagate),
        );
    }
}

fn aurora_follow_camera(
    primary_cameras: Query<(&Transform, &Camera, &Projection), Without<AuroraCameraTag>>,
    mut aurora_cameras: Query<(&mut Transform, &Camera, &mut Projection), With<AuroraCameraTag>>,
    mut aurora_mesh: Query<&mut Transform, (Without<Camera>, With<MeshMaterial3d<AuroraMaterial>>)>,
) {
    // find active camera TODO: IDENTIFY THE CORRECT CAMERA BETTER
    for (cam_tf, _camera, cam_proj) in primary_cameras
        .iter()
        .filter(|cam| cam.1.is_active && cam.1.order == 0)
    {
        for (mut aurora_tf, _cam, mut aurora_projection) in aurora_cameras.iter_mut() {
            // ensure same projection
            *aurora_projection = cam_proj.clone();
            *aurora_tf = *cam_tf;
            for mut aurora_tf in aurora_mesh.iter_mut() {
                aurora_tf.translation = cam_tf.translation;
            }
        }
    }
}

fn resize_aurora_on_window_change(
    mut resize_events: EventReader<WindowResized>,
    mut images: ResMut<Assets<Image>>,
    aurora_handles: Res<AuroraTextureHandle>,
    aurora_settings: Res<AuroraSettings>,
    primary_windows: Query<&Window, With<PrimaryWindow>>,
    mut repeated_calls: Local<i32>,
) {
    let mut update_aurora = aurora_settings.is_changed();

    for event in resize_events.read() {
        let is_primary = primary_windows.get(event.window).is_ok();
        update_aurora |= is_primary;
    }
    if !update_aurora {
        *repeated_calls = 0;
        return;
    }

    *repeated_calls += 1;
    if *repeated_calls > 10 {
        warn!(
            "aurora texture, was resized every last:{} frames!",
            *repeated_calls
        );
        warn!("make sure AuroraSettings doesn't mutate every frame");
    }

    let Ok(window) = primary_windows.single() else {
        return;
    };
    let aspect = window.width() / window.height();

    let width = (window.width() * aurora_settings.render_texture_percent.clamp(0.0, 1.0)) as u32;
    let height = (width as f32 / aspect.max(0.0001)) as u32;
    let width = width.max(2);
    let height = height.max(2);

    if let Some(image) = images.get_mut(&aurora_handles.render_target) {
        image.resize(Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        });
    }
}

fn spawn_aurora_skybox(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut sky_materials: ResMut<Assets<AuroraMaterial>>,
    noise_handles: Res<NoiseHandles>,
    aurora_texture_handle: Res<AuroraTextureHandle>,
) {
    let first_pass_layer = RenderLayers::layer(7);
    commands.spawn((
        Name::new("sky_aurora_skybox"),
        Mesh3d(meshes.add(utils::default_sky_mesh())),
        Transform::from_xyz(0.0, 0.0, 0.0),
        MeshMaterial3d(sky_materials.add(AuroraMaterial {
            noise3_image: noise_handles.noise3.clone(),
            ..default()
        })),
        first_pass_layer.clone(),
    ));

    // AURORA CAMERA
    commands.spawn((
        Name::new("camera_aurora"),
        Camera3d::default(),
        AuroraCameraTag,
        Camera {
            // render aurora before the "main pass" camera
            order: -1,
            target: aurora_texture_handle.render_target.clone().into(),
            clear_color: ClearColorConfig::Custom(Color::NONE),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)).looking_at(Vec3::ZERO, Vec3::Y),
        first_pass_layer,
    ));
}
