use bevy::{asset::weak_handle, prelude::*};

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

use rust_embed::RustEmbed;
#[derive(RustEmbed)]
#[folder = "assets"]
pub struct GradientSkyAssets;

fn add_shader(shaders: &mut Assets<Shader>, handle: Handle<Shader>, path: &str) {
    shaders.insert(
        &handle,
        Shader::from_wgsl(
            String::from_utf8(
                GradientSkyAssets::get(path)
                    .expect(format!("'{}' shader wgsl is not embedded", path).as_str())
                    .data
                    .into(),
            )
            .expect(format!("'{}' shader is not valid UTF-8", path).as_str()),
            path,
        ),
    );
}

pub fn initialize_shaders(mut shaders: ResMut<Assets<Shader>>) {
    add_shader(&mut shaders, SKY_SHADER_HANDLE, SKY_SHADER_PATH);
    add_shader(
        &mut shaders,
        FULL_AURORA_SHADER_HANDLE,
        FULL_AURORA_SHADER_PATH,
    );
    add_shader(&mut shaders, GRADIENT_SHADER_HANDLE, GRADIENT_SHADER_PATH);
    add_shader(&mut shaders, AURORA_SHADER_HANDLE, AURORA_SHADER_PATH);
    add_shader(&mut shaders, SUN_SHADER_HANDLE, SUN_SHADER_PATH);
    add_shader(&mut shaders, STARS_SHADER_HANDLE, STARS_SHADER_PATH);
    add_shader(&mut shaders, NOISE_SHADER_HANDLE, NOISE_SHADER_PATH);
    add_shader(
        &mut shaders,
        FULL_GRADIENT_SHADER_HANDLE,
        FULL_GRADIENT_SHADER_PATH,
    );
}
