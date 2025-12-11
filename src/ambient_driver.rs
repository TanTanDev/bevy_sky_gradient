use crate::{
    cycle::{SkyTime, SkyTimeSettings},
    gradient::{Gradient, GradientBuilder, ScalarGradient},
};
use bevy::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

///! used to build a ScalarGradient, bsed upon SkyTimeSettings
///! places the color we want based upon the timing of SkyTimeSettings
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Reflect)]
pub struct ScalarGradientBuilder {
    pub sunrise_color: f32,
    pub day_low_color: f32,
    pub day_high_color: f32,
    pub sunset_color: f32,
    pub night_low_color: f32,
    pub night_high_color: f32,
}

impl ScalarGradientBuilder {
    pub fn build_gradient(&self, s: &SkyTimeSettings) -> ScalarGradient {
        let sunrise_end = s.sunrise_percent_day() * 0.5;
        let sunset_start = 0.5 - s.sunset_percent_day() * 0.5;
        let sunset_end = 0.5 + s.sunset_percent_night() * 0.5;
        let sunrise_start = 1.0 - s.sunrise_percent_night() * 0.5;

        ScalarGradient::new(vec![
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

///! the current ambient colors
#[derive(Resource, Clone)]
pub struct AmbientColors {
    pub color_gradient: Gradient,
    pub brightness_gradient: ScalarGradient,
}

impl Default for AmbientColors {
    fn default() -> Self {
        Self {
            color_gradient: Gradient::default(),
            brightness_gradient: ScalarGradient::default(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Resource, Reflect, Clone)]
pub struct AmbientSettings {
    ///! this is the max ambient color brightness value
    ///! will be multiplied by the brightness_gradient
    pub brightness_multiplier: f32,
}

impl Default for AmbientSettings {
    fn default() -> Self {
        Self {
            brightness_multiplier: 7000.0,
        }
    }
}

#[derive(Resource, Clone, Reflect)]
pub struct AmbientColorsBuilder {
    pub color_gradient: GradientBuilder,
    pub scalar_gradient: ScalarGradientBuilder,
}

impl Default for AmbientColorsBuilder {
    fn default() -> Self {
        Self {
            color_gradient: GradientBuilder {
                sunrise_color: [255, 255, 200, 255],
                day_low_color: [255, 255, 150, 255],
                day_high_color: [255, 255, 200, 255],
                sunset_color: [240, 240, 255, 255],
                night_low_color: [150, 150, 225, 255],
                night_high_color: [100, 100, 150, 255],
            },
            scalar_gradient: ScalarGradientBuilder {
                sunrise_color: 0.4,
                day_low_color: 0.6,
                day_high_color: 1.0,
                sunset_color: 0.4,
                night_low_color: 0.3,
                night_high_color: 0.15,
            },
        }
    }
}

impl AmbientColorsBuilder {
    pub fn build(&self, sky_time_settings: &SkyTimeSettings) -> AmbientColors {
        AmbientColors {
            color_gradient: self.color_gradient.build_gradient(sky_time_settings),
            brightness_gradient: self.scalar_gradient.build_gradient(sky_time_settings),
        }
    }
}

/// Plugin that drives ambient lighting based on sky time
#[derive(Clone)]
pub struct AmbientDriverPlugin {
    pub ambient_colors_builder: AmbientColorsBuilder,
    pub ambient_settings: AmbientSettings,
}

impl Default for AmbientDriverPlugin {
    fn default() -> Self {
        Self {
            ambient_colors_builder: AmbientColorsBuilder::default(),
            ambient_settings: AmbientSettings::default(),
        }
    }
}

impl Plugin for AmbientDriverPlugin {
    fn build(&self, app: &mut App) {
        // Initial ambient colors (will be updated when SkyTimeSettings is available)
        app.insert_resource(
            self.ambient_colors_builder
                .build(&SkyTimeSettings::default()),
        );
        app.insert_resource(self.ambient_settings.clone());
        app.insert_resource(self.ambient_colors_builder.clone());

        app.register_type::<AmbientSettings>();

        // Rebuild ambient colors when settings change
        app.add_systems(
            Update,
            update_ambient_colors_builder.run_if(
                resource_changed::<SkyTimeSettings>.or(resource_changed::<AmbientColorsBuilder>),
            ),
        );

        app.add_systems(PostUpdate, drive_ambience);
    }
}

///! rebuild the ambient gradients because SkyTimeSettings changed
fn update_ambient_colors_builder(
    sky_time_settings: Res<SkyTimeSettings>,
    mut ambient_colors: ResMut<AmbientColors>,
    ambient_colors_builder: Res<AmbientColorsBuilder>,
) {
    *ambient_colors = ambient_colors_builder.build(&sky_time_settings);
}

fn drive_ambience(
    sky_time_settings: Res<SkyTimeSettings>,
    sky_time: Res<SkyTime>,
    ambient_settings: Res<AmbientSettings>,
    ambient_colors: Res<AmbientColors>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    let percent = sky_time_settings.time_percent(sky_time.time);

    let color = ambient_colors.color_gradient.sample_at(percent);
    let brightness = ambient_colors.brightness_gradient.sample_at(percent);

    ambient_light.color = Color::srgb(color[0], color[1], color[2]);
    ambient_light.brightness = brightness * ambient_settings.brightness_multiplier;
}
