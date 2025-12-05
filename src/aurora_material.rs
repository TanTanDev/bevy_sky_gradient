use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, CompareFunction, ShaderRef};

use crate::bind_groups::AuroraSettings;

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct AuroraMaterial {
    #[uniform(0)]
    pub aurora_settings: crate::bind_groups::AuroraSettings,

    #[texture(1, dimension = "3d")]
    #[sampler(2)]
    pub noise3_image: Handle<Image>,
}

impl Material for AuroraMaterial {
    fn vertex_shader() -> ShaderRef {
        crate::assets::FULL_AURORA_SHADER_HANDLE.into()
    }
    fn fragment_shader() -> ShaderRef {
        crate::assets::FULL_AURORA_SHADER_HANDLE.into()
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

impl Default for AuroraMaterial {
    fn default() -> Self {
        AuroraMaterial {
            aurora_settings: AuroraSettings {
                color_bottom: LinearRgba::new(0.0, 1.0, 0.2, 1.0),
                alpha: 0.7,
                density: 0.05,
                sharpness: 1.56,
                num_samples: 40,
                start_height: 3.1,
                end_height: 4.8,
                flow_scale: 0.002,
                flow_strength: 4.3,
                flow_speed: 0.005,
                wiggle_scale: 0.03,
                wiggle_strength: 1.05,
                wiggle_speed: 0.1,
                color_top: LinearRgba::new(0.0, 1.0, 0.8, 1.0),
                undersparkle_color_primary: LinearRgba::new(0.0, 1.0, 0.0, 1.0),
                undersparkle_color_secondary: LinearRgba::new(0.8, 0.2, 1.0, 1.0),
                undersparkle_scale: 0.004,
                undersparkle_speed: 0.02,
                undersparkle_threshold: 0.3,
                undersparkle_height: 0.3,
                opacity_per_sample: 0.18,
            },
            noise3_image: Handle::default(),
        }
    }
}
