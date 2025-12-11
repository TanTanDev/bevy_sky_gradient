use bevy::{
    pbr::Material,
    prelude::*,
    reflect::Reflect,
    render::render_resource::{AsBindGroup, CompareFunction, ShaderRef},
};

use crate::bind_groups::GradientBindGroup;

pub struct GradientMaterialPlugin;

impl Plugin for GradientMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<FullGradientMaterial>::default());
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct FullGradientMaterial {
    #[uniform(0)]
    pub gradient_bind_group: crate::bind_groups::GradientBindGroup,
}

impl Material for FullGradientMaterial {
    fn vertex_shader() -> ShaderRef {
        crate::assets::FULL_GRADIENT_SHADER_HANDLE.into()
    }
    fn fragment_shader() -> ShaderRef {
        crate::assets::FULL_GRADIENT_SHADER_HANDLE.into()
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

impl Default for FullGradientMaterial {
    fn default() -> Self {
        let color_stops = [
            Vec4::new(0.2, 0.3, 0.6, 1.0),
            Vec4::new(0.4, 0.5, 1.0, 1.0),
            Vec4::new(0.35, 0.6, 0.8, 1.0),
            Vec4::new(0.5, 0.7, 1.0, 1.0),
        ];
        FullGradientMaterial {
            gradient_bind_group: GradientBindGroup {
                color_stops,
                positions: Vec4::new(0.38, 0.47, 0.61, 1.0),
                num_stops: 4,
            },
        }
    }
}
