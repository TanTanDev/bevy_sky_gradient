use bevy::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    aurora_material::AuroraMaterial,
    gradient::{SkyColorsBuilder, StopsColors},
    sky_material::FullSkyMaterial,
    sun::SunSettings,
};

pub const DEFAULT_SKY_COLORS_BUILDER: SkyColorsBuilder = SkyColorsBuilder {
    sunset_color: StopsColors {
        stop0: [255, 70, 70, 255],
        stop1: [243, 84, 47, 255],
        stop2: [255, 242, 72, 255],
        stop3: [73, 177, 250, 255],
    },
    sunrise_color: StopsColors {
        stop0: [255, 70, 70, 255],
        stop1: [243, 84, 47, 255],
        stop2: [255, 242, 72, 255],
        stop3: [73, 177, 250, 255],
    },
    day_low_color: StopsColors {
        stop0: [157, 157, 248, 255],
        stop1: [205, 242, 255, 255],
        stop2: [182, 200, 254, 255],
        stop3: [224, 224, 255, 255],
    },
    day_high_color: StopsColors {
        stop0: [48, 48, 255, 255],
        stop1: [0, 226, 255, 255],
        stop2: [0, 170, 255, 255],
        stop3: [66, 195, 255, 255],
    },
    night_low_color: StopsColors {
        stop0: [0, 3, 40, 255],
        stop1: [47, 0, 93, 255],
        stop2: [0, 38, 97, 255],
        stop3: [74, 0, 89, 255],
    },
    night_high_color: StopsColors {
        stop0: [0, 0, 45, 255],
        stop1: [0, 32, 93, 255],
        stop2: [0, 0, 112, 255],
        stop3: [0, 0, 43, 255],
    },
};

///! data that controlls the look of a sky
///! (not aurora upsampling size, nor noise 3dTexture, performance and "look" should be seperate)
///! (None) values will not override current sky settings.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default)]
pub struct SkyPreset {
    pub aurora_settings: Option<crate::bind_groups::AuroraBindGroup>,
    pub sun_settings: Option<SunSettings>,
    pub sky_colors_builder: Option<SkyColorsBuilder>,
    pub stars: Option<crate::bind_groups::StarsBindGroup>,
}

pub struct SkyPresetPlugin;

impl Plugin for SkyPresetPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ApplyPresetEvent>();
        app.add_systems(Update, handle_apply_preset_events);
    }
}

#[derive(Event)]
pub struct ApplyPresetEvent {
    pub sky_preset: SkyPreset,
}

pub fn handle_apply_preset_events(
    mut events: EventReader<ApplyPresetEvent>,
    skyboxes: Query<&mut MeshMaterial3d<FullSkyMaterial>>,
    auroras: Query<&mut MeshMaterial3d<AuroraMaterial>>,
    mut sky_materials: ResMut<Assets<FullSkyMaterial>>,
    mut auroras_materials: ResMut<Assets<AuroraMaterial>>,
    mut sky_colors_builder_optional: Option<ResMut<SkyColorsBuilder>>,
    mut sun_settings_optional: Option<ResMut<SunSettings>>,
) {
    for event in events.read() {
        if let Some(new_sun_settings) = &event.sky_preset.sun_settings {
            if let Some(current_sun_settings) = &mut sun_settings_optional {
                **current_sun_settings = new_sun_settings.clone();
            }
        }
        if let Some(new_sky_colors_builder) = &event.sky_preset.sky_colors_builder {
            if let Some(current_sky_colors_builder) = sky_colors_builder_optional.as_mut() {
                **current_sky_colors_builder = new_sky_colors_builder.clone();
            }
        }

        if let Some(star_settings) = &event.sky_preset.stars {
            let skybox_material_handle = skyboxes
                .single()
                .expect("1 entity with SkyGradientMaterial");
            let skybox_material = sky_materials
                .get_mut(skybox_material_handle)
                .expect("SkyBoxMaterial");
            skybox_material.stars = star_settings.clone();
        }

        if let Some(aurora_settings) = &event.sky_preset.aurora_settings {
            let aurora_material_handle =
                auroras.single().expect("1 entity with SkyGradientMaterial");
            let aurora_material = auroras_materials
                .get_mut(aurora_material_handle)
                .expect("auroraMaterial");
            aurora_material.aurora_settings = aurora_settings.clone();
        }
    }
}
