use bevy::{
    asset::RenderAssetUsages,
    image::ImageSampler,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
};

use crate::{
    aurora::{AuroraPlugin, AuroraSettings},
    noise::{NoiseHandles, NoisePlugin},
    sky_material::FullSkyMaterial,
    utils,
};

///! sets up all you need to show a gradient skybox
pub struct SkyGradientPlugin {
    ///! if true, an entity skybox will spawn
    pub spawn_default_skybox: bool,
}

impl Plugin for SkyGradientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NoisePlugin::default());
        app.add_plugins(AuroraPlugin {
            aurora_settings: AuroraSettings::default(),
        });

        app.insert_resource(AuroraTextureHandle {
            render_target: Handle::default(),
        });
        app.add_systems(PreStartup, spawn_aurora_texture);

        app.add_systems(Startup, crate::assets::initialize_shaders);
        app.add_plugins(MaterialPlugin::<FullSkyMaterial>::default());
        if self.spawn_default_skybox {
            app.add_systems(Startup, spawn_default_skybox);
        }
        app.add_systems(
            PostUpdate,
            (sky_follow_camera,).before(TransformSystem::TransformPropagate),
        );
    }
}

impl Default for SkyGradientPlugin {
    fn default() -> Self {
        Self {
            spawn_default_skybox: true,
        }
    }
}

fn spawn_default_skybox(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut sky_materials: ResMut<Assets<FullSkyMaterial>>,
    noise_handles: Res<NoiseHandles>,
    aurora_handles: Res<AuroraTextureHandle>,
) {
    commands.spawn((
        Name::new("sky_gradient_skybox"),
        Mesh3d(meshes.add(utils::default_sky_mesh())),
        MeshMaterial3d(sky_materials.add(FullSkyMaterial {
            noise3_image: noise_handles.noise3.clone(),
            voronoi3_image: noise_handles.voronoi3.clone(),
            aurora_image: aurora_handles.render_target.clone(),
            ..default()
        })),
    ));
}

// aurora texture is defined by sky, and the aurora render into it. it needs to be defined by the sky plugin
#[derive(Resource)]
pub struct AuroraTextureHandle {
    pub render_target: Handle<Image>,
}

// spawn the aurora target texture, if not used, it's just a blank 2x2 texture
pub fn spawn_aurora_texture(
    mut images: ResMut<Assets<Image>>,
    mut aurora_texture_handle: ResMut<AuroraTextureHandle>,
) {
    let size = Extent3d {
        width: 2,
        height: 2,
        ..default()
    };

    let mut aurora_image = Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::default(),
    );
    aurora_image.sampler = ImageSampler::linear();
    aurora_image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;

    let aurora_image_handle = images.add(aurora_image);
    aurora_texture_handle.render_target = aurora_image_handle;
}

fn sky_follow_camera(
    camera_query: Query<(&Transform, &Camera)>,
    mut sky_query: Query<&mut Transform, (Without<Camera>, With<MeshMaterial3d<FullSkyMaterial>>)>,
) {
    // find active camera TODO: IDENTIFY THE CORRECT CAMERA BETTER
    for (cam_tf, _camera) in camera_query
        .iter()
        .filter(|cam| cam.1.is_active && cam.1.order == 0)
    {
        for mut tf in &mut sky_query {
            tf.translation = cam_tf.translation;
        }
    }
}
