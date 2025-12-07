use bevy::{
    asset::RenderAssetUsages,
    image::ImageSampler,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
};

use crate::{
    aurora::{AuroraPlugin, AuroraSettings},
    cycle::SkyCyclePlugin,
    gradient::GradientDriverPlugin,
    noise::{NoiseHandles, NoisePlugin, NoiseSettings},
    sky_material::FullSkyMaterial,
    sun::SunDriverPlugin,
    utils,
};

///! controlls what features you want.  
///! you might not want to use the default Cycle/SunDriver/GradientDriver/Aurora for example
///! then you can skip that plugin and implement your own.
pub struct SkyPluginBuilder {
    pub spawn_default_skybox: bool,
    pub noise: NoisePlugin,
    pub aurora: Option<AuroraPlugin>,
    pub cycle: Option<SkyCyclePlugin>,
    pub sun_driver: Option<SunDriverPlugin>,
    pub gradient_driver: Option<GradientDriverPlugin>,
}

impl Default for SkyPluginBuilder {
    fn default() -> Self {
        Self::all_features()
    }
}

impl SkyPluginBuilder {
    pub fn no_features() -> Self {
        Self {
            spawn_default_skybox: true,
            noise: NoisePlugin::default(),
            aurora: None,
            cycle: None,
            sun_driver: None,
            gradient_driver: None,
        }
    }

    pub fn all_features() -> Self {
        Self {
            spawn_default_skybox: true,
            noise: NoisePlugin::default(),
            aurora: Some(AuroraPlugin::default()),
            cycle: Some(SkyCyclePlugin::default()),
            sun_driver: Some(SunDriverPlugin::default()),
            gradient_driver: Some(GradientDriverPlugin::default()),
        }
    }

    pub fn set_spawn_default_skybox(mut self, spawn_default_skybox: bool) -> Self {
        self.spawn_default_skybox = spawn_default_skybox;
        self
    }

    pub fn build(self) -> SkyPlugin {
        SkyPlugin {
            spawn_default_skybox: self.spawn_default_skybox,
            sky_builder: self,
        }
    }

    pub fn with_aurora(mut self) -> Self {
        self.aurora = Some(AuroraPlugin::default());
        self
    }

    pub fn with_cycle(mut self) -> Self {
        self.cycle = Some(SkyCyclePlugin::default());
        self
    }

    pub fn with_sun_driver(mut self) -> Self {
        self.sun_driver = Some(SunDriverPlugin::default());
        self
    }

    pub fn with_gradient_driver(mut self) -> Self {
        self.gradient_driver = Some(GradientDriverPlugin::default());
        self
    }

    pub fn with_noise_settings(mut self, noise_settings: NoiseSettings) -> Self {
        self.noise.noise_settings = noise_settings;
        self
    }

    pub fn set_sun_driver(mut self, sun_driver: SunDriverPlugin) -> Self {
        self.sun_driver = Some(sun_driver);
        self
    }

    pub fn set_cycle(mut self, cycle: SkyCyclePlugin) -> Self {
        self.cycle = Some(cycle);
        self
    }

    pub fn set_gradient_driver(mut self, gradient_driver: GradientDriverPlugin) -> Self {
        self.gradient_driver = Some(gradient_driver);
        self
    }

    pub fn set_aurora(mut self, aurora_plugin: AuroraPlugin) -> Self {
        self.aurora = Some(aurora_plugin);
        self
    }
}

///! sets up all you need to show a gradient skybox
pub struct SkyPlugin {
    ///! if true, an entity skybox will spawn
    pub spawn_default_skybox: bool,
    pub sky_builder: SkyPluginBuilder,
}

impl SkyPlugin {
    pub fn builder() -> SkyPluginBuilder {
        SkyPluginBuilder::no_features()
    }
    pub fn builder_all_features() -> SkyPluginBuilder {
        SkyPluginBuilder::all_features()
    }
}

impl Plugin for SkyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(self.sky_builder.noise.clone());
        if let Some(aurora_plugin) = &self.sky_builder.aurora {
            app.add_plugins(aurora_plugin.clone());
        }
        if let Some(cycle_plugin) = &self.sky_builder.cycle {
            app.add_plugins(cycle_plugin.clone());
        }
        if let Some(sun_driver) = &self.sky_builder.sun_driver {
            if self.sky_builder.cycle.is_none() {
                error!("sun driver requires cycle plugin. prepare for crash");
            }
            app.add_plugins(sun_driver.clone());
        }
        if let Some(gradient_driver) = &self.sky_builder.gradient_driver {
            if self.sky_builder.gradient_driver.is_none() {
                error!("gradient driver requires cycle plugin. prepare for crash");
            }
            app.add_plugins(gradient_driver.clone());
        }

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

impl Default for SkyPlugin {
    fn default() -> Self {
        Self {
            spawn_default_skybox: true,
            sky_builder: SkyPluginBuilder::default(),
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
