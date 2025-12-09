use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, CompareFunction, ShaderRef};

use crate::bind_groups::AuroraBindGroup;

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct AuroraMaterial {
    #[uniform(0)]
    pub aurora_settings: crate::bind_groups::AuroraBindGroup,

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
            aurora_settings: AuroraBindGroup::default(),
            noise3_image: Handle::default(),
        }
    }
}
