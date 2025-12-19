use bevy::{
    asset::RenderAssetUsages,
    camera::visibility::RenderLayers,
    image::ImageSampler,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
    window::WindowResized,
};

use crate::plugin::SkyboxMagnetTag;

#[derive(Resource)]
pub struct FullSkyTextureHandle {
    pub render_target: Handle<Image>,
}

/// tag to find camera that render the sky
#[derive(Component)]
pub struct FullSkyCameraTag;

// Use a high-numbered, distinct render layer.
pub const FULL_SKY_RENDER_LAYER: RenderLayers = RenderLayers::layer(8);

#[derive(Resource, Clone)]
pub struct SkyTexturePluginSettings {
    pub sky_render_layer: RenderLayers,
    // order for camera that renders the entire sky, into a render texture
    pub full_sky_camera_order: isize,
    // order for the final camera that render the resulting sky texture
    // onto the screen
    pub final_camera_order: isize,
}

impl Default for SkyTexturePluginSettings {
    fn default() -> Self {
        Self {
            sky_render_layer: FULL_SKY_RENDER_LAYER,
            full_sky_camera_order: -2,
            final_camera_order: -1,
        }
    }
}

#[derive(Clone)]
pub struct SkyTexturePlugin {
    pub settings: SkyTexturePluginSettings,
}

impl Default for SkyTexturePlugin {
    fn default() -> Self {
        Self {
            settings: SkyTexturePluginSettings::default(),
        }
    }
}

/// holds the quad where we render the sky
#[derive(Component)]
pub struct FullSkySpriteTag;

impl Plugin for SkyTexturePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.settings.clone());
        app.insert_resource(FullSkyTextureHandle {
            render_target: Handle::default(),
        });
        app.add_systems(PreStartup, spawn_full_sky_texture);
        app.add_systems(Startup, spawn_full_sky_camera.after(spawn_full_sky_texture));
        app.add_systems(
            Startup,
            spawn_full_sky_screen_quad.after(spawn_full_sky_texture),
        );
        app.add_systems(
            PostUpdate,
            (
                full_sky_camera_follow_primary.before(TransformSystems::Propagate),
                resize_full_sky_on_window_change,
            ),
        );
    }
}

// System to spawn the full sky target texture at the start
pub fn spawn_full_sky_texture(
    mut images: ResMut<Assets<Image>>,
    mut texture_handle: ResMut<FullSkyTextureHandle>,
    // Use the window to get an initial size, full-resolution is best for the sky.
    primary_windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    let (width, height) = primary_windows
        .single()
        .map_or((1.0, 1.0), |v| (v.width(), v.height()));
    let size = Extent3d {
        width: width as u32,
        height: height as u32,
        ..default()
    };

    let mut sky_image = Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 0],                 // Start with a black/clear texture
        TextureFormat::Rgba8UnormSrgb, // Use RGBA for the final sky color
        RenderAssetUsages::default(),
    );
    sky_image.sampler = ImageSampler::linear();
    sky_image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;

    texture_handle.render_target = images.add(sky_image);
}

// System to resize the full sky texture when the window changes
fn resize_full_sky_on_window_change(
    mut resize_events: MessageReader<WindowResized>,
    mut images: ResMut<Assets<Image>>,
    sky_handles: Res<FullSkyTextureHandle>,
    primary_windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    let mut update_texture = false;
    for event in resize_events.read() {
        if primary_windows.get(event.window).is_ok() {
            update_texture = true;
            break;
        }
    }
    if !update_texture {
        return;
    }

    let Ok(window) = primary_windows.single() else {
        return;
    };
    let width = window.width() as u32;
    let height = window.height() as u32;

    if let Some(image) = images.get_mut(&sky_handles.render_target) {
        image.resize(Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        });
    }
}

fn spawn_full_sky_camera(
    mut commands: Commands,
    full_sky_handle: Res<FullSkyTextureHandle>,
    settings: Res<SkyTexturePluginSettings>,
) {
    // draw the full sky to a texture, instead of drawing directly to screen
    commands.spawn((
        Name::new("camera_full_sky"),
        Camera3d::default(),
        FullSkyCameraTag,
        Camera {
            order: settings.full_sky_camera_order,
            target: full_sky_handle.render_target.clone().into(),
            clear_color: ClearColorConfig::Custom(Color::NONE),
            ..default()
        },
        Transform::default(),
        settings.sky_render_layer.clone(), // The camera also needs the render layer
    ));
}

fn full_sky_camera_follow_primary(
    primary_cameras: Query<&Transform, (With<SkyboxMagnetTag>, Without<FullSkyCameraTag>)>,
    mut sky_cameras: Query<&mut Transform, (With<FullSkyCameraTag>, Without<SkyboxMagnetTag>)>,
) {
    if let Some(cam_tf) = primary_cameras.iter().next() {
        for mut sky_tf in sky_cameras.iter_mut() {
            *sky_tf = *cam_tf;
        }
    }
}

fn spawn_full_sky_screen_quad(
    mut commands: Commands,
    full_sky_handle: Res<FullSkyTextureHandle>,
    settings: Res<SkyTexturePluginSettings>,
) {
    commands.spawn((
        Name::new("sky_screen_sprite"),
        FullSkySpriteTag,
        Sprite {
            image: full_sky_handle.render_target.clone(),
            custom_size: Some(vec2(1.0, 1.0)),
            ..default()
        },
        Transform::default(),
    ));

    // this camera renders the sky that we put into the full_sky_handle.render_target
    // should render before you draw your game scene
    commands.spawn((
        Name::new("Camera_fullsky_screenquad"),
        Camera2d,
        Camera {
            order: settings.final_camera_order,
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::Fixed {
                width: 1.0,
                height: 1.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}
