use bevy::{
    prelude::*,
    render::render_resource::Extent3d,
    window::{PrimaryWindow, WindowResized},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    cycle::{SkyTime, SkyTimeSettings},
    plugin::GradientTextureHandle,
};

///! animates the sky gradients, REQUIRES CyclePlugin.
#[derive(Clone, Default)]
pub struct GradientDriverPlugin {
    pub sky_colors_builder: SkyColorsBuilder,
}

impl Plugin for GradientDriverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, drive_gradients);
        app.add_plugins(MaterialPlugin::<FullGradientMaterial>::default());

        // initial sky color values will be wrong, until SkyTimeSettings can be fetched in update_sky_colors_builer
        app.insert_resource(self.sky_colors_builder.build(&SkyTimeSettings::default()));
        app.add_systems(
            Update,
            update_sky_colors_builder.run_if(
                resource_changed::<SkyTimeSettings>.or(resource_changed::<SkyColorsBuilder>),
            ),
        );
        app.add_systems(PostUpdate, resize_gradient_on_window_change);
        // save the color builder so we can rebuild SkyColors on SkyTimeSettings changes
        app.insert_resource(self.sky_colors_builder.clone());
    }
}

// color stops change
fn update_sky_colors_builder(
    sky_time_settings: Res<SkyTimeSettings>,
    mut sky_colors: ResMut<SkyColors>,
    sky_colors_builder: Res<SkyColorsBuilder>,
) {
    *sky_colors = sky_colors_builder.build(&sky_time_settings);
}

///! All colors that controlls the sky gradient, OVER TIME.
///! a sky gradient has 4 colors, and we animate it based upon the "sky time"
///! gradient stops 0.0 -> 0.5 = DAY time colors
///! gradient stops 0.5 -> 1.0 = NIGHT time colors
///! Use the SkyColorsBuilder to easily construct the colors from variables like: day_color, night_color
#[derive(Resource)]
pub struct SkyColors {
    // sky color for stop1
    pub sky_color0: Gradient,
    // sky color for stop2
    pub sky_color1: Gradient,
    // sky color for stop3
    pub sky_color2: Gradient,
    // sky color for stop4
    pub sky_color3: Gradient,
}

impl Clone for SkyColors {
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

impl Default for SkyColors {
    fn default() -> Self {
        Self {
            sky_color0: Default::default(),
            sky_color1: Default::default(),
            sky_color2: Default::default(),
            sky_color3: Default::default(),
        }
    }
}

///! drive the sky materials
fn drive_gradients(
    sky_time_settings: Res<SkyTimeSettings>,
    sky_time: Res<SkyTime>,
    sky_colors: Res<SkyColors>,
    skyboxes: Query<&mut MeshMaterial3d<FullGradientMaterial>>,
    mut sky_materials: ResMut<Assets<FullGradientMaterial>>,
) {
    let skybox_material_handle = skyboxes
        .single()
        .expect("1 entity with SkyGradientMaterial");
    let skybox_material = sky_materials
        .get_mut(skybox_material_handle)
        .expect("SkyBoxMaterial");

    let percent = sky_time_settings.time_percent(sky_time.time);

    let color_from_gradient = |gradient: &Gradient| -> [f32; 4] { gradient.sample_at(percent) };
    skybox_material.gradient_bind_group.color_stops[0] =
        color_from_gradient(&sky_colors.sky_color0).into();
    skybox_material.gradient_bind_group.color_stops[1] =
        color_from_gradient(&sky_colors.sky_color1).into();
    skybox_material.gradient_bind_group.color_stops[2] =
        color_from_gradient(&sky_colors.sky_color2).into();
    skybox_material.gradient_bind_group.color_stops[3] =
        color_from_gradient(&sky_colors.sky_color3).into();
}

/// helper for designing gradients based upon time settings
///! if we want specific time of day colors. like "day_high_color"
/// the helper helps distribute these colors over a gradient based upon the SkyTimeSettings
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Resource)]
pub struct SkyColorsBuilder {
    pub sunset_color: StopsColors,
    pub sunrise_color: StopsColors,
    pub day_low_color: StopsColors,
    pub day_high_color: StopsColors,
    pub night_low_color: StopsColors,
    pub night_high_color: StopsColors,
}

impl Default for SkyColorsBuilder {
    fn default() -> Self {
        crate::presets::DEFAULT_SKY_COLORS_BUILDER
    }
}

impl SkyColorsBuilder {
    pub fn build(&self, sky_time_settings: &SkyTimeSettings) -> SkyColors {
        SkyColors {
            sky_color0: self.make_gradient_for_stop(0, &sky_time_settings),
            sky_color1: self.make_gradient_for_stop(1, &sky_time_settings),
            sky_color2: self.make_gradient_for_stop(2, &sky_time_settings),
            sky_color3: self.make_gradient_for_stop(3, &sky_time_settings),
        }
    }

    fn make_gradient_for_stop(&self, stop_pos: i32, s: &SkyTimeSettings) -> Gradient {
        // 0.0 -> 0.5 DAY colors
        // 0.5 -> 1.0 NIGHT colors
        // if time is 0, the day is JUST starting, which is why we place sunrise at 0
        let sunrise_end = s.sunrise_percent_day() * 0.5;
        let sunset_start = 0.5 - s.sunset_percent_day() * 0.5;
        let sunset_end = 0.5 + s.sunset_percent_night() * 0.5;
        let sunrise_start = 1.0 - s.sunrise_percent_night() * 0.5;
        Gradient::new(vec![
            (0.0, self.sunrise_color.get_stop(stop_pos)),
            (sunrise_end, self.day_low_color.get_stop(stop_pos)),
            (
                // sun high is between sunrise end, and
                (sunrise_end + sunset_start) * 0.5,
                self.day_high_color.get_stop(stop_pos),
            ),
            (sunset_start, self.day_low_color.get_stop(stop_pos)),
            (0.5, self.sunset_color.get_stop(stop_pos)),
            (sunset_end, self.night_low_color.get_stop(stop_pos)),
            (
                // sun night high is between the sunset end and sunrise start
                (sunset_end + sunrise_start) * 0.5,
                self.night_high_color.get_stop(stop_pos),
            ),
            (sunrise_start, self.night_low_color.get_stop(stop_pos)),
            (1.0, self.sunrise_color.get_stop(stop_pos)),
        ])
    }
}

///! 4 colors for stops 1,2,3,4 (A: 4 color gradient)
/// think of it as a color we want for a certain time:
/// what "DAY_COLOR" or "NIGHT_COLOR" 4 color gradient we want
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub struct StopsColors {
    pub stop0: [u8; 4],
    pub stop1: [u8; 4],
    pub stop2: [u8; 4],
    pub stop3: [u8; 4],
}

impl StopsColors {
    pub fn new_splat(color: [u8; 4]) -> Self {
        Self {
            stop0: color,
            stop1: color,
            stop2: color,
            stop3: color,
        }
    }
    pub fn get_stop(&self, stop_pos: i32) -> [u8; 4] {
        match stop_pos {
            0 => [self.stop0[0], self.stop0[1], self.stop0[2], self.stop0[3]],
            1 => [self.stop1[0], self.stop1[1], self.stop1[2], self.stop1[3]],
            2 => [self.stop2[0], self.stop2[1], self.stop2[2], self.stop2[3]],
            _ => [self.stop3[0], self.stop3[1], self.stop3[2], self.stop3[3]],
        }
    }
}

use std::cmp::Ordering;

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

use bevy::render::render_resource::{AsBindGroup, CompareFunction, ShaderRef};

use crate::bind_groups::GradientBindGroup;

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct FullGradientMaterial {
    #[uniform(0)]
    pub gradient_bind_group: crate::bind_groups::GradientBindGroup,
}

impl Material for FullGradientMaterial {
    fn vertex_shader() -> ShaderRef {
        // "embedded://bevy_sky_gradient/../assets/shaders/full_gradient.wgsl".into()
        crate::assets::FULL_GRADIENT_SHADER_HANDLE.into()
    }
    fn fragment_shader() -> ShaderRef {
        crate::assets::FULL_GRADIENT_SHADER_HANDLE.into()
        // "embedded://bevy_sky_gradient/../assets/shaders/full_gradient.wgsl".into()
    }

    fn specialize(
        _pipeline: &bevy::pbr::MaterialPipeline<Self>,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        _layout: &bevy::render::mesh::MeshVertexBufferLayoutRef,
        _key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        if let Some(depth_stencil) = &mut descriptor.depth_stencil {
            depth_stencil.depth_write_enabled = false;
            depth_stencil.depth_compare = CompareFunction::Always;
        }

        Ok(())
    }
}

impl Default for FullGradientMaterial {
    fn default() -> Self {
        let color_stops = [
            Vec4::new(0.2, 0.3, 0.6, 1.0),
            Vec4::new(0.4, 0.5, 1.0, 1.0),
            Vec4::new(0.35, 0.6, 0.8, 1.0),
            Vec4::new(0.5, 0.7, 1.0, 1.0),
        ];
        FullGradientMaterial {
            gradient_bind_group: GradientBindGroup {
                color_stops,
                positions: Vec4::new(0.38, 0.47, 0.61, 1.0),
                num_stops: 4,
            },
        }
    }
}

fn resize_gradient_on_window_change(
    mut resize_events: EventReader<WindowResized>,
    mut images: ResMut<Assets<Image>>,
    aurora_handles: Res<GradientTextureHandle>,
    primary_windows: Query<&Window, With<PrimaryWindow>>,
    mut repeated_calls: Local<i32>,
) {
    let mut update_texture = false;
    for event in resize_events.read() {
        let is_primary = primary_windows.get(event.window).is_ok();
        update_texture |= is_primary;
    }
    if !update_texture {
        *repeated_calls = 0;
        return;
    }

    *repeated_calls += 1;
    if *repeated_calls > 10 {
        warn!(
            "aurora texture, was resized every last:{} frames!",
            *repeated_calls
        );
        warn!("make sure AuroraSettings doesn't mutate every frame");
    }

    let Ok(window) = primary_windows.single() else {
        return;
    };

    if let Some(image) = images.get_mut(&aurora_handles.render_target) {
        image.resize(Extent3d {
            width: (window.width() as u32).max(2),
            height: (window.height() as u32).max(2),
            depth_or_array_layers: 1,
        });
    }
}
