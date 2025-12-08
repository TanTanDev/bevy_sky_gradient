use bevy::{
    asset::{AssetLoader, RenderAssetUsages, saver::AssetSaver},
    image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
    render::render_resource::{Extent3d, TextureDataOrder, TextureDimension},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// we will bake noise funcitons into textures for performance reasons
// 3D textures for noise3d, and voronoi3d
#[derive(Resource)]
pub struct NoiseHandles {
    pub noise3: Handle<Image>,
    pub voronoi3: Handle<Image>,
}

#[derive(Default, Clone)]
pub struct NoisePlugin {
    pub noise_settings: NoiseSettings,
}

impl Plugin for NoisePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.noise_settings.clone());
        app.add_systems(PreStartup, setup_noise_texture);
        app.add_systems(PostUpdate, update_noise_textures);

        #[cfg(feature = "serde")]
        {
            app.init_asset::<NoiseTextureAsset>();
            app.register_asset_loader(NoiseTextureAssetLoader);
            app.add_systems(Update, on_asset_load_texture_update);
        }
    }
}

///! texture_size data chart:
///! 64x64x64 = 0.25 mb
///! 128x128x128 = 2 mb
///! 256x256x256 = 16 mb
///! 514x514x514 = 129 mb
#[derive(Resource, Reflect, Clone)]
pub struct NoiseSettings {
    ///! size of 3d noise texture
    pub noise_texture_size: u32,
    ///! size of 3d noise texture
    pub voronoi_texture_size: u32,
    ///! if set, USERS may not exceed this size
    pub noise_size_limit: Option<u32>,

    #[cfg(feature = "serde")]
    pub cache_textures_locally: bool,
}

impl Default for NoiseSettings {
    fn default() -> Self {
        Self {
            noise_texture_size: 64,
            // stars looks much rounder 128, than 64
            voronoi_texture_size: 128,
            // prevent larger than 16 mb noise textures
            noise_size_limit: Some(256),
            #[cfg(feature = "serde")]
            cache_textures_locally: true,
        }
    }
}

#[cfg(feature = "serde")]
fn texture_file_name(noise_size: u32, voronoi_size: u32) -> String {
    format!("noise_textures_{}_{}.ron", noise_size, voronoi_size)
}

#[cfg(feature = "serde")]
pub fn on_asset_load_texture_update(
    mut commands: Commands,
    assets: Res<Assets<NoiseTextureAsset>>,
    mut images: ResMut<Assets<Image>>,
    noise_handles: Res<NoiseHandles>,
    pending: Option<Res<PendingNoiseTextureAsset>>,
) {
    let Some(pending) = pending else {
        return;
    };
    let Some(noise_asset) = assets.get(&pending.0) else {
        return;
    };
    commands.remove_resource::<PendingNoiseTextureAsset>();
    match images.get_mut(&noise_handles.noise3) {
        Some(noise3_img) => {
            noise3_img.resize(Extent3d {
                width: noise_asset.noise3_size,
                height: noise_asset.noise3_size,
                depth_or_array_layers: noise_asset.noise3_size,
            });

            noise3_img.data = Some(noise_asset.noise3_raw.clone());
        }
        None => warn!("SkyPlugin: Failed to set noise3 texture, no asset"),
    }

    match images.get_mut(&noise_handles.voronoi3) {
        Some(voronoi3_img) => {
            voronoi3_img.resize(Extent3d {
                width: noise_asset.voronoi3_size,
                height: noise_asset.voronoi3_size,
                depth_or_array_layers: noise_asset.voronoi3_size,
            });
            voronoi3_img.data = Some(noise_asset.voronoi_raw.clone());
        }
        None => warn!("SkyPlugin: Failed to set voronoi3 texture, no asset"),
    }
}

pub fn update_noise_textures(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    noise_settings: Res<NoiseSettings>,
    noise_handles: Res<NoiseHandles>,
    mut repeated_calls: Local<i32>,
    #[cfg(feature = "serde")] asset_server: Res<AssetServer>,
) {
    if !noise_settings.is_changed() {
        *repeated_calls = 0;
        return;
    }
    *repeated_calls += 1;
    if *repeated_calls > 10 {
        warn!(
            "SkyPlugin: noise textures, was resized every last: {} frames!",
            *repeated_calls
        );
        warn!("SkyPlugin: make sure NoiseSettings doesn't mutate every frame");
    }

    // limit texture size
    let max_size = noise_settings.noise_size_limit.unwrap_or(u32::MAX);
    let noise_size = noise_settings.noise_texture_size.clamp(1, max_size);
    let voronoi_size = noise_settings.voronoi_texture_size.clamp(1, max_size);

    // load cached noise texture files if possible
    #[cfg(feature = "serde")]
    {
        if noise_settings.cache_textures_locally {
            let file_name = texture_file_name(noise_size, voronoi_size);

            let fs_file_path = format!("assets/noise/{}", file_name);
            // does file exists, then load otherwise create file and load
            if std::fs::exists(path_relative_to_bevy_exe(&fs_file_path)).unwrap_or(false) {
                // LOAD FILE
                let bevy_file_path = format!("noise/{}", file_name);
                let handle = asset_server.load::<NoiseTextureAsset>(bevy_file_path);
                commands.insert_resource(PendingNoiseTextureAsset(handle));
                return;
            }
            // else continue below, to generate the textures
        }
    }

    // generate and set noise texture
    if let Some(noise3_image) = images.get_mut(&noise_handles.noise3) {
        let same_size = noise3_image.texture_descriptor.size.width == noise_size;
        if !same_size {
            noise3_image.resize(Extent3d {
                width: noise_size,
                height: noise_size,
                depth_or_array_layers: noise_size,
            });
            let noise3_data = generate_noise3(noise_size as usize);
            noise3_image.data = Some(noise3_data);
        }
    }

    if let Some(voronoi3_image) = images.get_mut(&noise_handles.voronoi3) {
        let same_size = voronoi3_image.texture_descriptor.size.width == voronoi_size;
        if !same_size {
            voronoi3_image.resize(Extent3d {
                width: voronoi_size,
                height: voronoi_size,
                depth_or_array_layers: voronoi_size,
            });
            let voronoi3_data = generate_voronoi3(voronoi_size as usize);
            voronoi3_image.data = Some(voronoi3_data);
        }
    }

    // save the generated noise texture to file
    #[cfg(feature = "serde")]
    {
        if noise_settings.cache_textures_locally {
            // if both images exists we can save lol
            if let (Some(n3_img), Some(v3_img)) = (
                images.get(&noise_handles.noise3),
                images.get(&noise_handles.voronoi3),
            ) {
                // clone the image data, to save to file
                let asset = NoiseTextureAsset {
                    noise3_raw: n3_img.data.clone().unwrap_or_default(),
                    voronoi_raw: v3_img.data.clone().unwrap_or_default(),
                    noise3_size: noise_size,
                    voronoi3_size: voronoi_size,
                };
                let dir = "assets/noise/";
                let file_name = texture_file_name(noise_size, voronoi_size);
                save_noise(dir, file_name.as_ref(), &asset);
            }
        }
    }
}

pub fn setup_noise_texture(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    noise_settings: Res<NoiseSettings>,
) {
    let noise3_data = generate_noise3(noise_settings.noise_texture_size as usize);
    let mut noise3_image = Image::new(
        Extent3d {
            width: noise_settings.noise_texture_size,
            height: noise_settings.noise_texture_size,
            depth_or_array_layers: noise_settings.noise_texture_size,
        },
        TextureDimension::D3,
        noise3_data,
        bevy::render::render_resource::TextureFormat::R8Unorm,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    noise3_image.sampler = make_noise_sampler();
    let noise3_handle = images.add(noise3_image);

    let voronoi3_data = generate_voronoi3(noise_settings.voronoi_texture_size as usize);
    let mut voronoi3_image = Image::new(
        Extent3d {
            width: noise_settings.voronoi_texture_size,
            height: noise_settings.voronoi_texture_size,
            depth_or_array_layers: noise_settings.voronoi_texture_size,
        },
        TextureDimension::D3,
        voronoi3_data,
        bevy::render::render_resource::TextureFormat::R8Unorm,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    voronoi3_image.sampler = make_noise_sampler();
    let voronoi3_handle = images.add(voronoi3_image);

    commands.insert_resource(NoiseHandles {
        noise3: noise3_handle,
        voronoi3: voronoi3_handle,
    });
}

fn make_noise_sampler() -> ImageSampler {
    ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        address_mode_w: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        ..default()
    })
}

///! will wait until a noisetextureasset is loaded, then override hte texture data
#[cfg(feature = "serde")]
#[derive(Resource)]
pub struct PendingNoiseTextureAsset(Handle<NoiseTextureAsset>);

pub fn generate_noise3(size: usize) -> Vec<u8> {
    let mut voxels = vec![0u8; size * size * size];

    for z in 0..size {
        for y in 0..size {
            for x in 0..size {
                let fx = x as f32 / size as f32 * std::f32::consts::TAU;
                let fy = y as f32 / size as f32 * std::f32::consts::TAU;
                let fz = z as f32 / size as f32 * std::f32::consts::TAU;

                let noise_units_per_volume = 10.0;
                // Tileable coordinate warp
                let p = vec3(fx.sin(), fy.sin(), fz.sin()) * noise_units_per_volume
                    + vec3(fx.cos(), fy.cos(), fz.cos()) * noise_units_per_volume;

                let n = noise3(p);
                let n8 = (n.clamp(0.0, 1.0) * 255.0) as u8;

                voxels[z * size * size + y * size + x] = n8;
            }
        }
    }
    voxels
}
fn noise3(p: Vec3) -> f32 {
    fn mod289(x: Vec4) -> Vec4 {
        x - (x * (1.0 / 289.0)).floor() * 289.0
    }
    fn perm4(x: Vec4) -> Vec4 {
        let x = ((x * 34.0) + 1.0) * x;
        mod289(x)
    }

    let a = p.floor();
    let mut d = p - a;
    d = d * d * (3.0 - 2.0 * d);

    let b = vec4(a.x, a.x, a.x + 1.0, a.x + 1.0);
    let c = vec4(a.y, a.y + 1.0, a.y, a.y + 1.0);

    let k1 = perm4(vec4(b.x, b.y, b.x, b.y));
    let k2 = perm4(vec4(k1.x, k1.y, k1.x, k1.y) + vec4(c.x, c.x, c.w, c.w));

    let c2 = k2 + vec4(a.z, a.z, a.z + 1.0, a.z + 1.0);
    let k3 = perm4(c2);
    let k4 = perm4(c2 + vec4(1.0, 1.0, 1.0, 1.0));

    let o1 = (k3 * (1. / 41.)).fract();
    let o2 = (k4 * (1. / 41.)).fract();

    let o3 = o2 * d.z + o1 * (1.0 - d.z);
    let o4 = vec4(o3.y, o3.w, o3.x, o3.z) * d.x + vec4(o3.x, o3.z, o3.y, o3.w) * (1.0 - d.x);

    o4.y * d.y + o4.x * (1.0 - d.y)
}

pub fn generate_voronoi3(size: usize) -> Vec<u8> {
    let mut voxels = vec![0u8; size * size * size];

    for z in 0..size {
        for y in 0..size {
            for x in 0..size {
                let scale = 7.0;
                let p = vec3(
                    x as f32 / size as f32 * scale,
                    y as f32 / size as f32 * scale,
                    z as f32 / size as f32 * scale,
                );
                let v = voronoi3(p);
                let v = (v.clamp(0.0, 1.0) * 255.0) as u8;

                let idx = z * size * size + y * size + x;
                voxels[idx] = v;
            }
        }
    }

    voxels
}

fn voronoi3(p: Vec3) -> f32 {
    let n = p.floor();
    let f = p - n;

    let mut min_dist = 1.0;

    for k in -1..=1 {
        for j in -1..=1 {
            for i in -1..=1 {
                let g = vec3(i as f32, j as f32, k as f32);
                let o = hash33(n + g);
                let r = g + o - f;
                let d = r.length();

                if d < min_dist {
                    min_dist = d;
                }
            }
        }
    }

    min_dist
}

fn hash33(p: Vec3) -> Vec3 {
    let mut p3 = (p * vec3(0.1031, 0.1030, 0.0973)).fract();
    p3 += Vec3::dot(p3, p3.yxz() + 33.33);
    return ((p3.xxy() + p3.yxx()) * p3.zyx()).fract();
}

///! file data of noise textures to avoid slow noise generation
#[cfg(feature = "serde")]
#[derive(Asset, TypePath, Clone, Serialize, Deserialize)]
pub struct NoiseTextureAsset {
    noise3_raw: Vec<u8>,
    noise3_size: u32,
    voronoi_raw: Vec<u8>,
    voronoi3_size: u32,
}

#[cfg(feature = "serde")]
#[derive(Debug, thiserror::Error)]
pub enum NoiseTextureLoaderError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    DecodeError(#[from] bincode::error::DecodeError),
    #[error(transparent)]
    LoadDirectError(#[from] bevy::asset::LoadDirectError),
}

#[cfg(feature = "serde")]
pub struct NoiseTextureAssetLoader;

#[cfg(feature = "serde")]
impl AssetLoader for NoiseTextureAssetLoader {
    type Asset = NoiseTextureAsset;
    type Settings = ();
    type Error = NoiseTextureLoaderError;

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        _load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<NoiseTextureAsset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let (noise_texture_asset, _s): (NoiseTextureAsset, usize) =
            bincode::serde::decode_from_slice(&bytes, bincode::config::standard())?;
        Ok(noise_texture_asset)
    }
}

#[cfg(feature = "serde")]
fn path_relative_to_bevy_exe(path: &str) -> std::path::PathBuf {
    let current_dir = bevy::asset::io::file::FileAssetReader::get_base_path();
    let new_path = current_dir.join(path);
    new_path
}

#[cfg(feature = "serde")]
pub fn save_noise(dir: &str, file_name: &str, asset: &NoiseTextureAsset) {
    let path = path_relative_to_bevy_exe(dir);
    if let Err(err) = std::fs::create_dir_all(&path) {
        error!(
            "SkyPlugin: failed to create_dir_all path: {:?}: error: {:?}",
            path, err
        );
    }

    match bincode::serde::encode_to_vec(&asset, bincode::config::standard()) {
        Ok(bytes) => {
            let file_path = path_relative_to_bevy_exe(format!("{}{}", dir, file_name).as_str());
            if let Err(err) = std::fs::write(&file_path, bytes) {
                info!(
                    "SkyPlugin: failed to fs::write path: {:?}, result: {:?}",
                    file_path, err
                );
            }
        }
        Err(err) => {
            error!("SkyPlugin: failed to encode asset err: {:?}", err);
        }
    }
}
