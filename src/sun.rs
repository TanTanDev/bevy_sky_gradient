use std::f32::consts::PI;

use bevy::{color::palettes::css::WHITE, light::light_consts::lux::AMBIENT_DAYLIGHT, prelude::*};

use crate::{
    cycle::{SkyTime, SkyTimeSettings},
    sky_material::FullSkyMaterial,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Marker for updating the position of the light
#[derive(Component)]
pub struct SunDriverTag;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Resource, Reflect, Clone)]
pub struct SunSettings {
    pub illuminance: f32,
    pub sun_color: Vec4,
    pub sun_strength: f32,
    pub sun_sharpness: f32,
}

impl Default for SunSettings {
    fn default() -> Self {
        Self {
            illuminance: AMBIENT_DAYLIGHT,
            sun_color: Vec4::new(1.0, 1.0, 0.5, 1.0),
            sun_strength: 1.5,
            sun_sharpness: 364.0,
        }
    }
}

/// "Drives" a sun light source
/// and updates the sun values of full_sky material
#[derive(Clone)]
pub struct SunDriverPlugin {
    pub spawn_default_sun_light: bool,
    pub sun_settings: SunSettings,
}

impl Default for SunDriverPlugin {
    fn default() -> Self {
        Self {
            spawn_default_sun_light: true,
            sun_settings: SunSettings::default(),
        }
    }
}

impl Plugin for SunDriverPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SunSettings>();
        app.insert_resource(self.sun_settings.clone());
        app.add_systems(PostUpdate, drive_sun);
        if self.spawn_default_sun_light {
            app.add_systems(Startup, spawn_default_sun);
        }
    }
}
fn spawn_default_sun(mut commands: Commands) {
    commands.spawn((
        Name::new("sky_gradient_sun"),
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
    skyboxes: Query<&mut MeshMaterial3d<FullSkyMaterial>>,
    mut sky_materials: ResMut<Assets<FullSkyMaterial>>,
) {
    // UPDATE the sun directional light
    let time_rotation = sky_time_settings.time_2pi(sky_time.time);

    // this rotation is looking at the sun
    let rotation_to_sun = Quat::from_rotation_x(time_rotation.sin().atan2(time_rotation.cos()));
    let look_at_sun = rotation_to_sun * Vec3::NEG_Z;
    let look_away_sun = rotation_to_sun * Quat::from_rotation_x(PI);
    let illuminance = time_rotation.sin().max(0.0).powf(2.0) * sun_settings.illuminance;

    for (mut light_trans, mut directional) in suns.iter_mut() {
        light_trans.rotation = look_away_sun;
        directional.illuminance = illuminance;
    }

    // UPDATE SKY MATERIAL
    let skybox_material_handle = skyboxes
        .single()
        .expect("1 entity with SkyGradientMaterial");
    let skybox_material = sky_materials
        .get_mut(skybox_material_handle)
        .expect("SkyBoxMaterial");

    skybox_material.sun.sun_dir = look_at_sun;

    skybox_material.sun.sun_color = sun_settings.sun_color;
    skybox_material.sun.sun_strength = sun_settings.sun_strength;
    skybox_material.sun.sun_sharpness = sun_settings.sun_sharpness;
}
