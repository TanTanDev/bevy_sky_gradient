use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, CompareFunction, ShaderRef};

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
    pub night_time_distance: f32,
}

impl Material for SkyGradientMaterial {
    fn vertex_shader() -> ShaderRef {
        "sky_gradient.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "sky_gradient.wgsl".into()
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

impl Default for SkyGradientMaterial {
    fn default() -> Self {
        SkyGradientMaterial {
            color_stops: [
                Vec4::new(0.2, 0.3, 0.6, 1.0),
                Vec4::new(0.4, 0.5, 1.0, 1.0),
                Vec4::new(0.35, 0.6, 0.8, 1.0),
                Vec4::new(0.5, 0.7, 1.0, 1.0),
            ],
            positions: Vec4::new(0.38, 0.47, 0.61, 1.0),
            num_stops: 4,
            sun_dir: Vec3::new(0.0, 0.1, -1.0),
            sun_color: Vec4::new(1.0, 1.0, 0.5, 1.0),
            sun_strength: 1.5,
            sun_sharpness: 164.0,
            night_time_distance: 0.0,
        }
    }
}
