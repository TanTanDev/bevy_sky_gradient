use bevy::{color::palettes::css::WHITE, pbr::light_consts::lux::AMBIENT_DAYLIGHT, prelude::*};
use bevy_inspector_egui::egui::Color32;
use egui_colorgradient::{Gradient, InterpolationMethod};

use crate::gradient_material::SkyGradientMaterial;

///  cycle has 2 plugins: SkyCyclePlugin and SunDriverPlugin

///! this plugin controlls the colors of our skygradient
///! through our SkyTime
pub struct SkyCyclePlugin {
    pub sky_time_settings: SkyTimeSettings,
    pub sky_colors_builder: SkyColorsBuilder,
}

impl Default for SkyCyclePlugin {
    fn default() -> Self {
        Self {
            sky_time_settings: Default::default(),
            sky_colors_builder: Default::default(),
        }
    }
}

impl Plugin for SkyCyclePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SkyTime {
            time: 0.0,
            auto_tick: true,
        });
        app.insert_resource(self.sky_time_settings.clone());
        app.insert_resource(
            self.sky_colors_builder
                .clone()
                .build(&self.sky_time_settings),
        );
        app.add_systems(Update, (update_sky_time, drive_gradients).chain());
    }
}

fn update_sky_time(
    mut sky_time: ResMut<SkyTime>,
    time: Res<Time>,
    sky_time_settings: Res<SkyTimeSettings>,
) {
    if !sky_time.auto_tick {
        return;
    }
    sky_time.time += time.delta_secs();
    if sky_time.time > sky_time_settings.total_time() {
        sky_time.time -= sky_time_settings.total_time();
    }
}

///! drive the sky materials
fn drive_gradients(
    sky_time_settings: Res<SkyTimeSettings>,
    sky_time: Res<SkyTime>,
    sky_colors: Res<SkyColors>,
    skyboxes: Query<&mut MeshMaterial3d<SkyGradientMaterial>>,
    mut sky_materials: ResMut<Assets<SkyGradientMaterial>>,
) {
    let skybox_material_handle = skyboxes
        .single()
        .expect("1 entity with SkyGradientMaterial");
    let skybox_material = sky_materials
        .get_mut(skybox_material_handle)
        .expect("SkyBoxMaterial");
    skybox_material.night_time_distance = sky_time_settings.night_time_distance(sky_time.time);

    let percent = sky_time_settings.time_percent(sky_time.time);

    let color_from_gradient = |gradient: &Gradient| -> [f32; 4] {
        gradient
            .interpolator()
            .sample_at(percent)
            .unwrap()
            .to_array()
    };
    skybox_material.color_stops[0] = color_from_gradient(&sky_colors.sky_color0).into();
    skybox_material.color_stops[1] = color_from_gradient(&sky_colors.sky_color1).into();
    skybox_material.color_stops[2] = color_from_gradient(&sky_colors.sky_color2).into();
    skybox_material.color_stops[3] = color_from_gradient(&sky_colors.sky_color3).into();
}

#[derive(Resource, Reflect)]
pub struct SunSettings {
    illuminance: f32,
}

// Marker for updating the position of the light, not needed unless we have multiple lights
#[derive(Component)]
pub struct SunDriverTag;

///! Drives a sun light source
///! THIS PLUGIN REQUIRES A SkyCyclePlugin
pub struct SunDriverPlugin {
    pub spawn_default_sun: bool,
}
impl Default for SunDriverPlugin {
    fn default() -> Self {
        Self {
            spawn_default_sun: true,
        }
    }
}

impl Plugin for SunDriverPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SunSettings>();
        app.insert_resource(SunSettings {
            illuminance: AMBIENT_DAYLIGHT,
        });
        app.add_systems(PostUpdate, drive_sun);
        if self.spawn_default_sun {
            app.add_systems(Startup, spawn_default_sun);
        }
    }
}
fn spawn_default_sun(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            color: WHITE.into(),
            illuminance: AMBIENT_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        SunDriverTag,
        Transform::default(),
    ));
}

fn drive_sun(
    mut suns: Query<(&mut Transform, &mut DirectionalLight), With<SunDriverTag>>,
    sky_time_settings: Res<SkyTimeSettings>,
    sky_time: Res<SkyTime>,
    sun_settings: Res<SunSettings>,
    skyboxes: Query<&mut MeshMaterial3d<SkyGradientMaterial>>,
    mut sky_materials: ResMut<Assets<SkyGradientMaterial>>,
) {
    // UPDATE the sun directional light
    let time_rotation = sky_time_settings.time_2pi(sky_time.time);

    let rotation = Quat::from_rotation_x(time_rotation.sin().atan2(time_rotation.cos()));
    let illuminance = time_rotation.sin().max(0.0).powf(2.0) * sun_settings.illuminance;
    let mut sun_forward = Vec3::Z;

    for (mut light_trans, mut directional) in suns.iter_mut() {
        light_trans.rotation = rotation;
        directional.illuminance = illuminance;
        sun_forward = light_trans.forward().as_vec3();
    }

    // UPDATE SKY MATERIAL
    let skybox_material_handle = skyboxes
        .single()
        .expect("1 entity with SkyGradientMaterial");
    let skybox_material = sky_materials
        .get_mut(skybox_material_handle)
        .expect("SkyBoxMaterial");

    skybox_material.sun_dir = sun_forward;
}

///! All colors that controlls the sky gradient, OVER TIME.
///! a sky gradient has 4 colors, and we animate it based upon the "sky time"
///! gradient stops 0.0 -> 0.5 = DAY time colors
///! gradient stops 0.5 -> 1.0 = NIGHT time colors
///! Use the SkyColorsBuilder to easily construct the colors from variables like: day_color, night_color
#[derive(Resource)]
pub struct SkyColors {
    // sky color for stop1
    sky_color0: Gradient,
    // sky color for stop2
    sky_color1: Gradient,
    // sky color for stop3
    sky_color2: Gradient,
    // sky color for stop4
    sky_color3: Gradient,
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

///! The current sky time
#[derive(Resource, Reflect)]
pub struct SkyTime {
    pub time: f32,
    pub auto_tick: bool,
}

///! the sky timings
#[derive(Resource, Clone)]
pub struct SkyTimeSettings {
    // how many seconds of day light
    pub day_time_sec: f32,
    // how many seconds of night light
    pub night_time_sec: f32,

    // seconds of sunrise, INSIDE the day time
    pub sunrise_time_sec: f32,
    // seconds of sunset, INSIDE the night time
    pub sunset_time_sec: f32,
}

impl Default for SkyTimeSettings {
    fn default() -> Self {
        Self {
            day_time_sec: 15.0,
            night_time_sec: 25.0,
            sunrise_time_sec: 2.0,
            sunset_time_sec: 2.0,
        }
    }
}

impl SkyTimeSettings {
    #[inline]
    pub fn day_percent(&self, time: f32) -> f32 {
        (time / self.day_time_sec).min(1.0)
    }
    #[inline]
    pub fn night_percent(&self, time: f32) -> f32 {
        ((time - self.day_time_sec) / self.night_time_sec).max(0.0)
    }
    #[inline]
    pub fn time_percent(&self, time: f32) -> f32 {
        (self.day_percent(time) + self.night_percent(time)) * 0.5
    }
    #[inline]
    ///! 0: Not close to night time
    ///! 1: fully night
    pub fn night_time_distance(&self, time: f32) -> f32 {
        1.0 - (self.night_percent(time) - 0.5).abs() * 2.0
    }

    #[inline]
    /// convert time to full rotation
    pub fn time_2pi(&self, time: f32) -> f32 {
        self.day_percent(time) * std::f32::consts::PI
            + self.night_percent(time) * std::f32::consts::PI
    }

    #[inline]
    pub fn total_time(&self) -> f32 {
        self.day_time_sec + self.night_time_sec
    }

    #[inline]
    pub fn sunrise_percent_day(&self) -> f32 {
        self.sunrise_time_sec / self.day_time_sec
    }
    #[inline]
    pub fn sunrise_percent_night(&self) -> f32 {
        self.sunrise_time_sec / self.night_time_sec
    }
    #[inline]
    pub fn sunset_percent_day(&self) -> f32 {
        self.sunset_time_sec / self.day_time_sec
    }
    #[inline]
    pub fn sunset_percent_night(&self) -> f32 {
        self.sunset_time_sec / self.night_time_sec
    }
}

/// helper for designing gradients based upon time settings
///! if we want specific time of day colors. like "day_high_color"
/// the helper helps distribute these colors over a gradient based upon the SkyTimeSettings
#[derive(Clone)]
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
    pub fn build(self, sky_time_settings: &SkyTimeSettings) -> SkyColors {
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
