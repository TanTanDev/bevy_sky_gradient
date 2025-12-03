use bevy::{asset::weak_handle, prelude::*};

pub const SKY_SHADER_PATH: &str = "shaders/sky_shader.wgsl";
pub const SKY_SHADER_HANDLE: Handle<Shader> = weak_handle!("0aed3aa7-55d3-43be-9e04-5637b0e9ceef");

use rust_embed::RustEmbed;
#[derive(RustEmbed)]
#[folder = "assets"]
pub struct GradientSkyAssets;

pub fn initialize_shaders(mut shaders: ResMut<Assets<Shader>>) {
    shaders.insert(
        &SKY_SHADER_HANDLE,
        Shader::from_wgsl(
            String::from_utf8(
                GradientSkyAssets::get(SKY_SHADER_PATH)
                    .expect("sky shader wgsl is not embedded")
                    .data
                    .into(),
            )
            .expect("sky shader is not valid UTF-8"),
            SKY_SHADER_PATH,
        ),
    )
}
