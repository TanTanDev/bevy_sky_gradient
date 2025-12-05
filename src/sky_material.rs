use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, CompareFunction, ShaderRef};

use crate::bind_groups::{GradientSettings, StarsSettings, SunSettings};

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct FullSkyMaterial {
    #[uniform(0)]
    pub gradient_settings: crate::bind_groups::GradientSettings,
    #[uniform(1)]
    pub sun: crate::bind_groups::SunSettings,
    #[uniform(2)]
    pub stars: crate::bind_groups::StarsSettings,
    ///! auto set. 0 = NO night, 1 = FULL night
    ///! a full night cycle will go from 0 -> 1 -> 0
    #[uniform(3)]
    pub night_time_distance: f32,
    ///! when in the night time to show the stars
    ///! x: when to start showing star
    ///! y: when the brightness of star is MAXED out
    ///! example: (0.0, 0.1).
    ///! 0.0: start showing sky immediately when sunset begins
    ///! 0.1: sky brightness is maxed out 10% into the night
    #[uniform(4)]
    pub night_visibility_range: Vec2,

    // noise
    #[texture(10, dimension = "3d")]
    #[sampler(11)]
    pub noise3_image: Handle<Image>,
    #[texture(12, dimension = "3d")]
    #[sampler(13)]
    pub voronoi3_image: Handle<Image>,

    #[texture(14, dimension = "2d")]
    #[sampler(15)]
    pub aurora_image: Handle<Image>,
}

impl Material for FullSkyMaterial {
    fn vertex_shader() -> ShaderRef {
        crate::assets::SKY_SHADER_HANDLE.into()
    }
    fn fragment_shader() -> ShaderRef {
        crate::assets::SKY_SHADER_HANDLE.into()
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

impl Default for FullSkyMaterial {
    fn default() -> Self {
        let color_stops = [
            Vec4::new(0.2, 0.3, 0.6, 1.0),
            Vec4::new(0.4, 0.5, 1.0, 1.0),
            Vec4::new(0.35, 0.6, 0.8, 1.0),
            Vec4::new(0.5, 0.7, 1.0, 1.0),
        ];
        FullSkyMaterial {
            gradient_settings: GradientSettings {
                color_stops,
                positions: Vec4::new(0.38, 0.47, 0.61, 1.0),
                num_stops: 4,
            },
            sun: SunSettings {
                sun_dir: Vec3::new(0.0, 0.1, -1.0),
                sun_color: Vec4::new(1.0, 1.0, 0.5, 1.0),
                sun_strength: 1.5,
                sun_sharpness: 164.0,
            },
            night_time_distance: 0.0,
            night_visibility_range: vec2(0.0, 0.1),
            stars: StarsSettings {
                sky_rotation_speed: 0.01,
                sample_scale: 9.0,
                mask_scale: 1.0,
                blink_variance_scale: 0.03,
                mask_threshold: 0.4,
                star_threshold: 0.9,
                star_threshold_blink: 0.01,
                blink_speed: 10.0,
            },
            noise3_image: Handle::default(),
            voronoi3_image: Handle::default(),
            aurora_image: Handle::default(),
        }
    }
}
