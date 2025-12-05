use bevy::{
    asset::RenderAssetUsages,
    image::ImageSampler,
    prelude::*,
    render::{
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
        view::RenderLayers,
    },
    window::WindowResized,
};

use crate::{
    aurora_material::AuroraMaterial,
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
        app.add_plugins(NoisePlugin);

        app.add_systems(Startup, crate::assets::initialize_shaders);
        app.add_plugins(MaterialPlugin::<FullSkyMaterial>::default());
        app.add_plugins(MaterialPlugin::<AuroraMaterial>::default());
        app.add_systems(PreStartup, spawn_aurora_skybox);
        if self.spawn_default_skybox {
            app.add_systems(Startup, spawn_default_skybox);
        }
        app.add_systems(
            PostUpdate,
            (
                sky_follow_camera,
                aurora_follow_camera,
                resize_aurora_on_window_change,
            )
                .before(TransformSystem::TransformPropagate),
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
    aurora_handles: Res<AuroraHandles>,
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

#[derive(Component)]
pub struct AuroraCameraTag;

#[derive(Resource)]
pub struct AuroraHandles {
    render_target: Handle<Image>,
}

fn spawn_aurora_skybox(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut sky_materials: ResMut<Assets<AuroraMaterial>>,
    noise_handles: Res<NoiseHandles>,
) {
    let first_pass_layer = RenderLayers::layer(7);
    commands.spawn((
        Name::new("sky_aurora_skybox"),
        Mesh3d(meshes.add(utils::default_sky_mesh())),
        Transform::from_xyz(0.0, 0.0, 0.0),
        MeshMaterial3d(sky_materials.add(AuroraMaterial {
            noise3_image: noise_handles.noise3.clone(),
            voronoi3_image: noise_handles.voronoi3.clone(),
            ..default()
        })),
        first_pass_layer.clone(),
    ));

    let size = Extent3d {
        width: 256,
        height: 256,
        // width: 512,
        // height: 512,
        // width: 1920,
        // height: 1080,
        ..default()
    };

    // This is the texture that will be rendered to.
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
    // AURORA CAMERA
    commands.spawn((
        Name::new("camera_aurora"),
        Camera3d::default(),
        AuroraCameraTag,
        Camera {
            // render aurora before the "main pass" camera
            order: -1,
            target: aurora_image_handle.clone().into(),
            clear_color: ClearColorConfig::Custom(Color::NONE),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)).looking_at(Vec3::ZERO, Vec3::Y),
        first_pass_layer,
    ));
    commands.insert_resource(AuroraHandles {
        render_target: aurora_image_handle.clone(),
    });
    // commands.spawn((ImageNode::new(aurora_image_handle.clone())));
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

fn aurora_follow_camera(
    primary_cameras: Query<(&Transform, &Camera), Without<AuroraCameraTag>>,
    mut aurora_cameras: Query<(&mut Transform, &Camera), With<AuroraCameraTag>>,
    mut aurora_mesh: Query<&mut Transform, (Without<Camera>, With<MeshMaterial3d<AuroraMaterial>>)>,
) {
    // find active camera TODO: IDENTIFY THE CORRECT CAMERA BETTER
    for (cam_tf, _camera) in primary_cameras
        .iter()
        .filter(|cam| cam.1.is_active && cam.1.order == 0)
    {
        for (mut aurora_tf, _came) in aurora_cameras.iter_mut() {
            // aurora_tf.translation = cam_tf.translation;
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
    aurora_handles: Res<AuroraHandles>,
) {
    for event in resize_events.read() {
        let aspect = event.width / event.height;
        // let width = 256;
        let width = 514;
        let height = (width as f32 / aspect) as u32;

        if let Some(image) = images.get_mut(&aurora_handles.render_target) {
            image.resize(Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            });
        }
    }
}
