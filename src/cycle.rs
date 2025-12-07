use bevy::prelude::*;

use crate::sky_material::FullSkyMaterial;

// cycle.rs has 2 plugins: SkyCyclePlugin and SunDriverPlugin

///! this plugin controlls the colors of our skygradient
///! through our SkyTime
#[derive(Clone)]
pub struct SkyCyclePlugin {
    pub sky_time_settings: SkyTimeSettings,
    pub sky_time: SkyTime,
}

impl Default for SkyCyclePlugin {
    fn default() -> Self {
        Self {
            sky_time_settings: SkyTimeSettings::default(),
            sky_time: SkyTime::default(),
        }
    }
}

impl Plugin for SkyCyclePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.sky_time.clone());
        app.insert_resource(self.sky_time_settings.clone());
        app.add_systems(Update, (update_sky_time, drive_night_time).chain());
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

// inform sky material the time!
fn drive_night_time(
    sky_time_settings: Res<SkyTimeSettings>,
    sky_time: Res<SkyTime>,
    skyboxes: Query<&mut MeshMaterial3d<FullSkyMaterial>>,
    mut sky_materials: ResMut<Assets<FullSkyMaterial>>,
) {
    let skybox_material_handle = skyboxes
        .single()
        .expect("1 entity with SkyGradientMaterial");
    let skybox_material = sky_materials
        .get_mut(skybox_material_handle)
        .expect("SkyBoxMaterial");
    skybox_material.night_time_distance = sky_time_settings.night_time_distance(sky_time.time);
}

///! The current sky time
#[derive(Resource, Reflect, Clone)]
pub struct SkyTime {
    pub time: f32,
    pub auto_tick: bool,
}

impl Default for SkyTime {
    fn default() -> Self {
        Self {
            time: 0.0,
            auto_tick: true,
        }
    }
}

///! the sky timings
#[derive(Resource, Clone, Reflect)]
pub struct SkyTimeSettings {
    // how many seconds of day light
    pub day_time_sec: f32,
    // how many seconds of night light
    pub night_time_sec: f32,
    // seconds of sunrise, ("steals" from day time)
    pub sunrise_time_sec: f32,
    // seconds of sunset, ("steals" from night time)
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
