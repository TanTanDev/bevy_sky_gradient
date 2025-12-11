use bevy::prelude::*;

use crate::{
    cycle::{SkyTime, SkyTimeSettings},
    gradient::Gradient,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Resource, Reflect, Clone)]
pub struct AmbientSettings {
    gradient: Gradient,
    brightness: f32,
}

impl Default for AmbientSettings {
    fn default() -> Self {
        Self {
            gradient: Gradient::new(vec![
                (0.0, [255, 255, 255, 255]),
                (0.5, [0, 0, 0, 100]),
                (1.0, [255, 255, 255, 255]),
            ]),
            brightness: 7000.0,
        }
    }
}

///! "Drives" the ambient color
#[derive(Clone)]
pub struct AmbientDriverPlugin {
    pub ambient_settings: AmbientSettings,
}

impl Default for AmbientDriverPlugin {
    fn default() -> Self {
        Self {
            ambient_settings: AmbientSettings::default(),
        }
    }
}

impl Plugin for AmbientDriverPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.ambient_settings.clone());
        app.register_type::<AmbientSettings>();
        app.add_systems(PostUpdate, drive_ambience);
    }
}

fn drive_ambience(
    sky_time_settings: Res<SkyTimeSettings>,
    sky_time: Res<SkyTime>,
    ambient_settings: Res<AmbientSettings>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    let percent = sky_time_settings.time_percent(sky_time.time);

    let color = ambient_settings.gradient.sample_at(percent);

    ambient_light.color = Color::srgb(color[0], color[1], color[2]);
    ambient_light.brightness = color[3] * ambient_settings.brightness;
}
