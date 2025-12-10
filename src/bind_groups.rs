use bevy::{prelude::*, render::render_resource::ShaderType};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Reflect, ShaderType)]
pub struct GradientBindGroup {
    ///! the colors of sky
    pub color_stops: [Vec4; 4],
    ///! where the color gradients are positioned
    pub positions: Vec4,
    ///! how many sky colors to make gradient of (max 4)
    pub num_stops: u32,
}

impl Default for GradientBindGroup {
    fn default() -> Self {
        let color_stops = [
            Vec4::new(0.2, 0.3, 0.6, 1.0),
            Vec4::new(0.4, 0.5, 1.0, 1.0),
            Vec4::new(0.35, 0.6, 0.8, 1.0),
            Vec4::new(0.5, 0.7, 1.0, 1.0),
        ];
        GradientBindGroup {
            color_stops,
            positions: Vec4::new(0.38, 0.47, 0.61, 1.0),
            num_stops: 4,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Reflect, ShaderType)]
pub struct StarsBindGroup {
    ///! how fast to rotate sky per sec in radians
    pub sky_rotation_speed: f32,
    pub sample_scale: f32,
    pub star_threshold: f32,
    pub star_threshold_blink: f32,
    pub blink_speed: f32,
    pub mask_scale: f32,
    pub mask_threshold: f32,
    pub blink_variance_scale: f32,
}

impl Default for StarsBindGroup {
    fn default() -> Self {
        Self {
            sky_rotation_speed: 0.01,
            sample_scale: 9.0,
            mask_scale: 1.0,
            blink_variance_scale: 0.03,
            mask_threshold: 0.4,
            star_threshold: 0.9,
            star_threshold_blink: 0.01,
            blink_speed: 10.0,
        }
    }
}

#[derive(Clone, Debug, Reflect, ShaderType)]
pub struct SunBindGroup {
    pub sun_dir: Vec3,
    pub sun_color: Vec4,
    pub sun_strength: f32,
    pub sun_sharpness: f32,
}

impl Default for SunBindGroup {
    fn default() -> Self {
        Self {
            sun_dir: Vec3::new(0.0, 0.1, -1.0),
            sun_color: Vec4::new(1.0, 1.0, 0.5, 1.0),
            sun_strength: 1.5,
            sun_sharpness: 164.0,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Reflect, ShaderType)]
pub struct AuroraBindGroup {
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
    pub flow_x_speed: f32,
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

impl Default for AuroraBindGroup {
    fn default() -> Self {
        Self {
            color_bottom: LinearRgba::new(0.0, 1.0, 0.2, 1.0),
            alpha: 0.7,
            density: 0.05,
            sharpness: 1.56,
            num_samples: 60,
            start_height: 3.1,
            end_height: 4.8,
            flow_scale: 0.002,
            flow_strength: 4.3,
            flow_speed: 0.005,
            flow_x_speed: -0.6,
            wiggle_scale: 0.03,
            wiggle_strength: 1.05,
            wiggle_speed: 0.1,
            color_top: LinearRgba::new(0.0, 1.0, 0.8, 1.0),
            undersparkle_color_primary: LinearRgba::new(0.0, 2.3, 0.0, 1.0),
            undersparkle_color_secondary: LinearRgba::new(6.3, 0.2, 4.0, 1.0),
            undersparkle_scale: 0.004,
            undersparkle_speed: 0.02,
            undersparkle_threshold: 0.3,
            undersparkle_height: 0.3,
            opacity_per_sample: 0.18,
        }
    }
}
