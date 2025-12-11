use bevy::{
    prelude::*,
    render::render_resource::Extent3d,
    window::{PrimaryWindow, WindowResized},
};

use crate::{
    cycle::{SkyTime, SkyTimeSettings},
    gradient::{Gradient, SkyGradientBuilder, SkyGradients},
    gradient_material::FullGradientMaterial,
    plugin::GradientTextureHandle,
};

///! animates the sky gradients, REQUIRES CyclePlugin.
#[derive(Clone, Default)]
pub struct GradientDriverPlugin {
    pub sky_colors_builder: SkyGradientBuilder,
}

impl Plugin for GradientDriverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, drive_gradients);

        // initial sky color values will be wrong, until SkyTimeSettings can be fetched in update_sky_colors_builer
        app.insert_resource(self.sky_colors_builder.build(&SkyTimeSettings::default()));
        app.add_systems(
            Update,
            update_sky_colors_builder.run_if(
                resource_changed::<SkyTimeSettings>.or(resource_changed::<SkyGradientBuilder>),
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
    mut sky_colors: ResMut<SkyGradients>,
    sky_colors_builder: Res<SkyGradientBuilder>,
) {
    *sky_colors = sky_colors_builder.build(&sky_time_settings);
}

///! drive the sky materials
fn drive_gradients(
    sky_time_settings: Res<SkyTimeSettings>,
    sky_time: Res<SkyTime>,
    sky_colors: Res<SkyGradients>,
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
