use bevy::prelude::*;
use bevy_inspector_egui::egui::Color32;
use egui_colorgradient::{Gradient, InterpolationMethod};

use crate::{
    cycle::{SkyTime, SkyTimeSettings},
    sky_material::FullSkyMaterial,
};

///! animates the sky gradients, REQUIRES CyclePlugin.
#[derive(Clone, Default)]
pub struct GradientDriverPlugin {
    pub sky_colors_builder: SkyColorsBuilder,
}

impl Plugin for GradientDriverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, drive_gradients);

        // initial sky color values will be wrong, until SkyTimeSettings can be fetched in update_sky_colors_builer
        app.insert_resource(self.sky_colors_builder.build(&SkyTimeSettings::default()));
        app.add_systems(
            Update,
            update_sky_colors_builder.run_if(resource_changed::<SkyTimeSettings>),
        );
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
    info!("updating sky colors from skytimesettings");
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
                interpolation_method: self.sky_color0.interpolation_method,
            },
            sky_color1: Gradient {
                stops: self.sky_color1.stops.clone(),
                interpolation_method: self.sky_color1.interpolation_method,
            },
            sky_color2: Gradient {
                stops: self.sky_color2.stops.clone(),
                interpolation_method: self.sky_color2.interpolation_method,
            },
            sky_color3: Gradient {
                stops: self.sky_color3.stops.clone(),
                interpolation_method: self.sky_color3.interpolation_method,
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
    skyboxes: Query<&mut MeshMaterial3d<FullSkyMaterial>>,
    mut sky_materials: ResMut<Assets<FullSkyMaterial>>,
) {
    let skybox_material_handle = skyboxes
        .single()
        .expect("1 entity with SkyGradientMaterial");
    let skybox_material = sky_materials
        .get_mut(skybox_material_handle)
        .expect("SkyBoxMaterial");

    let percent = sky_time_settings.time_percent(sky_time.time);

    let color_from_gradient = |gradient: &Gradient| -> [f32; 4] {
        gradient
            .interpolator()
            .sample_at(percent)
            .unwrap()
            .to_array()
    };
    skybox_material.gradient_settings.color_stops[0] =
        color_from_gradient(&sky_colors.sky_color0).into();
    skybox_material.gradient_settings.color_stops[1] =
        color_from_gradient(&sky_colors.sky_color1).into();
    skybox_material.gradient_settings.color_stops[2] =
        color_from_gradient(&sky_colors.sky_color2).into();
    skybox_material.gradient_settings.color_stops[3] =
        color_from_gradient(&sky_colors.sky_color3).into();
}

/// helper for designing gradients based upon time settings
///! if we want specific time of day colors. like "day_high_color"
/// the helper helps distribute these colors over a gradient based upon the SkyTimeSettings
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
        Gradient::new(
            InterpolationMethod::Linear,
            vec![
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
            ],
        )
    }
}

///! 4 colors for stops 1,2,3,4 (A: 4 color gradient)
/// think of it as a color we want for a certain time:
/// what "DAY_COLOR" or "NIGHT_COLOR" 4 color gradient we want
#[derive(Clone)]
pub struct StopsColors {
    pub stop0: Color32,
    pub stop1: Color32,
    pub stop2: Color32,
    pub stop3: Color32,
}

impl StopsColors {
    pub fn new_splat(color: Color32) -> Self {
        Self {
            stop0: color,
            stop1: color,
            stop2: color,
            stop3: color,
        }
    }
    pub fn get_stop(&self, stop_pos: i32) -> Color32 {
        match stop_pos {
            0 => self.stop0,
            1 => self.stop1,
            2 => self.stop2,
            _ => self.stop3,
        }
    }
}
