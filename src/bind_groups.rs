use bevy::{prelude::*, render::render_resource::ShaderType};

#[derive(Clone, Debug, Reflect, ShaderType)]
pub struct GradientSettings {
    pub color_stops: [Vec4; 4],
    pub positions: Vec4,
    pub num_stops: u32,
}
#[derive(Clone, Debug, Reflect, ShaderType)]
pub struct StarsSettings {
    ///! how fast to rotate sky per sec in radians, recommended value around: 0.01
    pub sky_rotation_speed: f32,
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
    pub alpha: f32,   // default: 1.0
    pub density: f32, // default: 3.0
    // pub aurora_wave_count: f32,            // default: 6.0
    pub sharpness: f32,       // default: 4.0
    pub num_samples: i32,     // default: 8 (quality vs performance)
    pub start_height: f32,    // default: 15.0
    pub end_height: f32,      // default: 35.0
    pub flow_scale: f32,      // default: 0.3 (controls how much bands rotate)
    pub flow_strength: f32,   // default: 8 (quality vs performance)
    pub flow_speed: f32,      // default: 8 (quality vs performance)
    pub wiggle_scale: f32,    // default: 0.3 (controls how much bands rotate)
    pub wiggle_strength: f32, // default: 8 (quality vs performance)
    pub wiggle_speed: f32,    // default: 8 (quality vs performance)
    pub color_top: LinearRgba,
    pub undersparkle_color_primary: LinearRgba,
    pub undersparkle_color_secondary: LinearRgba,
    pub undersparkle_scale: f32,
    pub undersparkle_speed: f32,
    pub undersparkle_threshold: f32,
    pub undersparkle_height: f32,
}
