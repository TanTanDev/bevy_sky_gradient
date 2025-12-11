use crate::cycle::SkyTimeSettings;
use bevy::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

///! All the current colors that controlls the sky gradient
///! a sky gradient has 4 colors, and we animate it based upon the "sky time"
///! gradient stops 0.0 -> 0.5 = DAY time colors
///! gradient stops 0.5 -> 1.0 = NIGHT time colors
///! Use the SkyColorsBuilder to easily construct the colors from variables like: day_color, night_color
#[derive(Resource)]
pub struct SkyGradients {
    pub sky_color0: Gradient,
    pub sky_color1: Gradient,
    pub sky_color2: Gradient,
    pub sky_color3: Gradient,
}

impl Clone for SkyGradients {
    fn clone(&self) -> Self {
        Self {
            sky_color0: Gradient {
                stops: self.sky_color0.stops.clone(),
            },
            sky_color1: Gradient {
                stops: self.sky_color1.stops.clone(),
            },
            sky_color2: Gradient {
                stops: self.sky_color2.stops.clone(),
            },
            sky_color3: Gradient {
                stops: self.sky_color3.stops.clone(),
            },
        }
    }
}

impl Default for SkyGradients {
    fn default() -> Self {
        Self {
            sky_color0: Default::default(),
            sky_color1: Default::default(),
            sky_color2: Default::default(),
            sky_color3: Default::default(),
        }
    }
}

///! day/night time specific colors mapped onto a gradient from SkyTimeSettings
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Reflect)]
pub struct GradientBuilder {
    pub sunrise_color: [u8; 4],
    pub day_low_color: [u8; 4],
    pub day_high_color: [u8; 4],
    pub sunset_color: [u8; 4],
    pub night_low_color: [u8; 4],
    pub night_high_color: [u8; 4],
}

impl GradientBuilder {
    pub fn build_gradient(&self, s: &SkyTimeSettings) -> Gradient {
        let sunrise_end = s.sunrise_percent_day() * 0.5;
        let sunset_start = 0.5 - s.sunset_percent_day() * 0.5;
        let sunset_end = 0.5 + s.sunset_percent_night() * 0.5;
        let sunrise_start = 1.0 - s.sunrise_percent_night() * 0.5;

        Gradient::new(vec![
            (0.0, self.sunrise_color),
            (sunrise_end, self.day_low_color),
            ((sunrise_end + sunset_start) * 0.5, self.day_high_color),
            (sunset_start, self.day_low_color),
            (0.5, self.sunset_color),
            (sunset_end, self.night_low_color),
            ((sunset_end + sunrise_start) * 0.5, self.night_high_color),
            (sunrise_start, self.night_low_color),
            (1.0, self.sunrise_color),
        ])
    }
}

/// helper for designing gradients based upon time settings
///! if we want specific time of day colors. like "day_high_color"
/// the helper helps distribute these colors over a gradient based upon the SkyTimeSettings
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Resource)]
pub struct SkyGradientBuilder {
    pub gradient_builder_stop0: GradientBuilder,
    pub gradient_builder_stop1: GradientBuilder,
    pub gradient_builder_stop2: GradientBuilder,
    pub gradient_builder_stop3: GradientBuilder,
}

impl Default for SkyGradientBuilder {
    fn default() -> Self {
        crate::presets::DEFAULT_SKY_COLORS_BUILDER
    }
}

impl SkyGradientBuilder {
    pub fn build(&self, sky_time_settings: &SkyTimeSettings) -> SkyGradients {
        SkyGradients {
            sky_color0: self
                .gradient_builder_stop0
                .build_gradient(&sky_time_settings),
            sky_color1: self
                .gradient_builder_stop1
                .build_gradient(&sky_time_settings),
            sky_color2: self
                .gradient_builder_stop2
                .build_gradient(&sky_time_settings),
            sky_color3: self
                .gradient_builder_stop3
                .build_gradient(&sky_time_settings),
        }
    }
}

/// A color gradient with linear interpolation.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct Gradient {
    pub stops: Vec<(f32, [u8; 4])>,
}

impl Gradient {
    /// Create a new gradient. Stops are automatically sorted by position.
    pub fn new(mut stops: Vec<(f32, [u8; 4])>) -> Self {
        stops.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        Self { stops }
    }

    /// Sort the stops. Call this if you manually modify the `stops` vector.
    pub fn sort(&mut self) {
        self.stops
            .sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    }

    /// Sample the gradient at position `t` (0.0 to 1.0).
    /// Returns a normalized color [f32; 4] where channels are 0.0 to 1.0.
    pub fn sample_at(&self, t: f32) -> [f32; 4] {
        if self.stops.is_empty() {
            return [0.0, 0.0, 0.0, 1.0];
        }

        // Find insertion point
        let idx = self.stops.partition_point(|(x, _)| *x < t);

        // Helper to convert u8 [0-255] to f32 [0.0-1.0]
        let to_f32 = |c: [u8; 4]| {
            [
                c[0] as f32 / 255.0,
                c[1] as f32 / 255.0,
                c[2] as f32 / 255.0,
                c[3] as f32 / 255.0,
            ]
        };
        if idx == 0 {
            return to_f32(self.stops[0].1);
        }
        if idx >= self.stops.len() {
            return to_f32(self.stops.last().unwrap().1);
        }
        let (t0, c0_u8) = self.stops[idx - 1];
        let (t1, c1_u8) = self.stops[idx];
        let c0 = to_f32(c0_u8);
        let c1 = to_f32(c1_u8);
        // Linear interpolation
        let ratio = ((t - t0) / (t1 - t0)).clamp(0.0, 1.0);
        let lerp = |a: f32, b: f32| a + (b - a) * ratio;

        [
            lerp(c0[0], c1[0]),
            lerp(c0[1], c1[1]),
            lerp(c0[2], c1[2]),
            lerp(c0[3], c1[3]),
        ]
    }
}

impl Default for Gradient {
    fn default() -> Self {
        Self::new(vec![(0.0, [0, 0, 0, 255]), (1.0, [255, 255, 255, 255])])
    }
}

/// A scalar gradient with linear interpolation for single float values.
/// More memory efficient than Gradient when you only need one channel.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct ScalarGradient {
    pub stops: Vec<(f32, f32)>,
}

impl ScalarGradient {
    /// Create a new scalar gradient. Stops are automatically sorted by position.
    pub fn new(mut stops: Vec<(f32, f32)>) -> Self {
        stops.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        Self { stops }
    }

    /// Sort the stops. Call this if you manually modify the `stops` vector.
    pub fn sort(&mut self) {
        self.stops
            .sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    }

    /// Sample the gradient at position `t` (0.0 to 1.0).
    pub fn sample_at(&self, t: f32) -> f32 {
        if self.stops.is_empty() {
            return 0.0;
        }
        let idx = self.stops.partition_point(|(x, _)| *x < t);
        if idx == 0 {
            return self.stops[0].1;
        }
        if idx >= self.stops.len() {
            return self.stops.last().unwrap().1;
        }

        let (t0, v0) = self.stops[idx - 1];
        let (t1, v1) = self.stops[idx];
        // Linear interpolation
        let ratio = ((t - t0) / (t1 - t0)).clamp(0.0, 1.0);
        v0 + (v1 - v0) * ratio
    }
}

impl Default for ScalarGradient {
    fn default() -> Self {
        Self::new(vec![(0.0, 0.0), (1.0, 1.0)])
    }
}
