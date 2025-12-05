use bevy::{prelude::*, render::render_resource::ShaderType};

#[derive(Clone, Debug, Reflect, ShaderType)]
pub struct GradientSettings {
    #[allow(dead_code)]
    pub color_stops: [Vec4; 4],
    #[allow(dead_code)]
    pub positions: Vec4,
    #[allow(dead_code)]
    pub num_stops: u32,
}
#[derive(Clone, Debug, Reflect, ShaderType)]
pub struct StarsSettings {
    ///! how fast to rotate sky per sec in radians, recommended value around: 0.01
    pub sky_rotation_speed: f32,
    pub sample_scale: f32,
    pub star_threshold: f32,
    pub star_threshold_blink: f32,
    pub blink_speed: f32,
    pub mask_scale: f32,
    pub mask_threshold: f32,
    pub blink_variance_scale: f32,
}

#[derive(Clone, Debug, Reflect, ShaderType)]
pub struct SunSettings {
    pub sun_dir: Vec3,
    pub sun_color: Vec4,
    pub sun_strength: f32,
    pub sun_sharpness: f32,
}

#[derive(Clone, Debug, Reflect, ShaderType)]
pub struct AuroraSettings {
    pub color_bottom: LinearRgba,
    pub alpha: f32,
    pub density: f32,
    pub sharpness: f32,
    pub num_samples: i32,
    pub start_height: f32,
    pub end_height: f32,
    pub flow_scale: f32,
    pub flow_strength: f32,
    pub flow_speed: f32,
    pub wiggle_scale: f32,
    pub wiggle_strength: f32,
    pub wiggle_speed: f32,
    pub color_top: LinearRgba,
    pub undersparkle_color_primary: LinearRgba,
    pub undersparkle_color_secondary: LinearRgba,
    pub undersparkle_scale: f32,
    pub undersparkle_speed: f32,
    pub undersparkle_threshold: f32,
    pub undersparkle_height: f32,
    pub opacity_per_sample: f32,
}
