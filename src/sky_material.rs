use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, CompareFunction, ShaderRef};

use crate::bind_groups::{StarsBindGroup, SunBindGroup};

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct FullSkyMaterial {
    #[uniform(0)]
    pub sun: crate::bind_groups::SunBindGroup,
    #[uniform(1)]
    pub stars: crate::bind_groups::StarsBindGroup,
    ///! auto set. 0 = NO night, 1 = FULL night
    ///! a full night cycle will go from 0 -> 1 -> 0
    #[uniform(2)]
    pub night_time_distance: f32,
    ///! when in the night time to show the stars
    ///! x: when to start showing star
    ///! y: when the brightness of star is MAXED out
    ///! example: (0.0, 0.1).
    ///! 0.0: start showing sky immediately when sunset begins
    ///! 0.1: sky brightness is maxed out 10% into the night
    #[uniform(3)]
    pub night_visibility_range: Vec2,

    #[uniform(4)]
    pub feature_stars_enabled: i32,
    #[uniform(5)]
    pub feature_sun_enabled: i32,
    #[uniform(6)]
    pub feature_aurora_enabled: i32,

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
    #[texture(16, dimension = "2d")]
    #[sampler(17)]
    pub gradient_image: Handle<Image>,
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
        FullSkyMaterial {
            gradient_image: Handle::default(),
            sun: SunBindGroup {
                sun_dir: Vec3::new(0.0, 0.1, -1.0),
                sun_color: Vec4::new(1.0, 1.0, 0.5, 1.0),
                sun_strength: 1.5,
                sun_sharpness: 164.0,
            },
            night_time_distance: 0.0,
            night_visibility_range: vec2(0.0, 0.1),
            stars: StarsBindGroup::default(),
            noise3_image: Handle::default(),
            voronoi3_image: Handle::default(),
            aurora_image: Handle::default(),
            feature_stars_enabled: 1,
            feature_sun_enabled: 1,
            feature_aurora_enabled: 1,
        }
    }
}
