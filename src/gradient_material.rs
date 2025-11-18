use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

// #[derive(TypePath, Asset, AsBindGroup, Debug, Clone)]
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct SkyGradientMaterial {
    #[uniform(0)]
    pub color_stops: [Vec4; 4],
    #[uniform(1)]
    pub positions: Vec4,
    #[uniform(2)]
    pub num_stops: u32,
    #[uniform(3)]
    pub sun_dir: Vec3,
    #[uniform(4)]
    pub sun_color: Vec4,
    #[uniform(5)]
    pub sun_strength: f32,
    #[uniform(6)]
    pub sun_sharpness: f32,
    #[uniform(7)]
    pub time_percent: f32,
}

impl Material for SkyGradientMaterial {
    fn vertex_shader() -> ShaderRef {
        "sky_gradient.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "sky_gradient.wgsl".into()
    }

    // fn alpha_mode(&self) -> AlphaMode {
    //     AlphaMode::Opaque
    // }

    // fn opaque_render_method(&self) -> bevy::pbr::OpaqueRendererMethod {
    //     bevy::pbr::OpaqueRendererMethod::Forward
    // }

    fn specialize(
        pipeline: &bevy::pbr::MaterialPipeline<Self>,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        layout: &bevy::render::mesh::MeshVertexBufferLayoutRef,
        key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        if let Some(depth_stencil) = &mut descriptor.depth_stencil {
            depth_stencil.depth_write_enabled = false;
            depth_stencil.depth_compare = bevy::render::render_resource::CompareFunction::Always;
        }
        Ok(())
    }
    // fn specialize(
    //     _pipeline: &bevy::pbr::MaterialPipeline<Self>,
    //     descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
    //     _layout: &bevy::render::mesh::MeshVertexBufferLayout,
    //     _key: bevy::pbr::MaterialPipelineKey<Self>,
    // ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
    //     if let Some(depth_stencil) = &mut descriptor.depth_stencil {
    //         depth_stencil.depth_write_enabled = false;
    //         depth_stencil.depth_compare = bevy::render::render_resource::CompareFunction::Always;
    //     }
    //     Ok(())
    // }
}
