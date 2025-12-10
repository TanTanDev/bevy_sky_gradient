use bevy::{
    asset::{embedded_asset, weak_handle},
    prelude::*,
};

pub const SKY_SHADER_PATH: &str = "shaders/full_sky.wgsl";
pub const SKY_SHADER_HANDLE: Handle<Shader> = weak_handle!("0aed3aa7-55d3-43be-9e04-5637b0e9ceef");
pub const GRADIENT_SHADER_PATH: &str = "shaders/sky_gradient.wgsl";
pub const GRADIENT_SHADER_HANDLE: Handle<Shader> =
    weak_handle!("0aed3aa1-15d3-42be-9e04-5637b0e9cefc");
pub const AURORA_SHADER_PATH: &str = "shaders/aurora.wgsl";
pub const AURORA_SHADER_HANDLE: Handle<Shader> =
    weak_handle!("0aed3aa1-15d3-42be-9e03-2137b0eecbfc");
pub const FULL_AURORA_SHADER_PATH: &str = "shaders/full_aurora.wgsl";
pub const FULL_AURORA_SHADER_HANDLE: Handle<Shader> =
    weak_handle!("0aed3aa1-15d3-42be-9e03-2731b4eecbfb");
pub const SUN_SHADER_PATH: &str = "shaders/sun.wgsl";
pub const SUN_SHADER_HANDLE: Handle<Shader> = weak_handle!("0aed3aa1-15d3-42be-9e03-2137b0e2c3fb");
pub const STARS_SHADER_PATH: &str = "shaders/stars.wgsl";
pub const STARS_SHADER_HANDLE: Handle<Shader> =
    weak_handle!("1a3d3ae1-15d3-42be-9e03-2137b0e2c3fb");
pub const NOISE_SHADER_PATH: &str = "shaders/noise.wgsl";
pub const NOISE_SHADER_HANDLE: Handle<Shader> =
    weak_handle!("1a3d3ae1-15d3-42be-9e03-2131b0e5c2fe");
pub const FULL_GRADIENT_SHADER_PATH: &str = "shaders/full_gradient.wgsl";
pub const FULL_GRADIENT_SHADER_HANDLE: Handle<Shader> =
    weak_handle!("1a3d3ae1-15d3-42be-9e03-2131b0e3c1ef");

pub struct SkyAssetsPlugin;

impl Plugin for SkyAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_shaders);
    }
}

pub fn initialize_shaders(mut shaders: ResMut<Assets<Shader>>) {
    shaders.insert(
        &SKY_SHADER_HANDLE,
        Shader::from_wgsl(
            String::from_utf8(include_bytes!("../assets/shaders/full_sky.wgsl").into())
                .expect(format!("'{}' shader is not valid UTF-8", SKY_SHADER_PATH).as_str()),
            "bevy_sky_gradient/shaders/full_sky.wgsl",
        ),
    );
    shaders.insert(
        &GRADIENT_SHADER_HANDLE,
        Shader::from_wgsl(
            String::from_utf8(include_bytes!("../assets/shaders/sky_gradient.wgsl").into())
                .expect(format!("'{}' shader is not valid UTF-8", GRADIENT_SHADER_PATH).as_str()),
            "bevy_sky_gradient/shaders/sky_gradient.wgsl",
        ),
    );
    shaders.insert(
        &FULL_GRADIENT_SHADER_HANDLE,
        Shader::from_wgsl(
            String::from_utf8(include_bytes!("../assets/shaders/full_gradient.wgsl").into())
                .expect(
                    format!("'{}' shader is not valid UTF-8", FULL_GRADIENT_SHADER_PATH).as_str(),
                ),
            "bevy_sky_gradient/shaders/full_gradient.wgsl",
        ),
    );
    shaders.insert(
        &AURORA_SHADER_HANDLE,
        Shader::from_wgsl(
            String::from_utf8(include_bytes!("../assets/shaders/aurora.wgsl").into())
                .expect(format!("'{}' shader is not valid UTF-8", AURORA_SHADER_PATH).as_str()),
            "bevy_sky_gradient/shaders/aurora.wgsl",
        ),
    );
    shaders.insert(
        &FULL_AURORA_SHADER_HANDLE,
        Shader::from_wgsl(
            String::from_utf8(include_bytes!("../assets/shaders/full_aurora.wgsl").into()).expect(
                format!("'{}' shader is not valid UTF-8", FULL_AURORA_SHADER_PATH).as_str(),
            ),
            "bevy_sky_gradient/shaders/full_aurora.wgsl",
        ),
    );
    shaders.insert(
        &STARS_SHADER_HANDLE,
        Shader::from_wgsl(
            String::from_utf8(include_bytes!("../assets/shaders/stars.wgsl").into())
                .expect(format!("'{}' shader is not valid UTF-8", STARS_SHADER_PATH).as_str()),
            "bevy_sky_gradient/shaders/stars.wgsl",
        ),
    );
    shaders.insert(
        &SUN_SHADER_HANDLE,
        Shader::from_wgsl(
            String::from_utf8(include_bytes!("../assets/shaders/sun.wgsl").into())
                .expect(format!("'{}' shader is not valid UTF-8", SUN_SHADER_PATH).as_str()),
            "bevy_sky_gradient/shaders/sun.wgsl",
        ),
    );
    shaders.insert(
        &NOISE_SHADER_HANDLE,
        Shader::from_wgsl(
            String::from_utf8(include_bytes!("../assets/shaders/noise.wgsl").into())
                .expect(format!("'{}' shader is not valid UTF-8", NOISE_SHADER_PATH).as_str()),
            "bevy_sky_gradient/shaders/noise.wgsl",
        ),
    );
}
