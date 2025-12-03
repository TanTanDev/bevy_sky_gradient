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
    ///! auto set. 0 = NO night, 1 = FULL night
    ///! a full night cycle will go from 0 -> 1 -> 0
    #[uniform(7)]
    pub night_time_distance: f32,
    ///! when in the night time to show the stars
    ///! x: when to start showing star
    ///! y: when the brightness of star is MAXED out
    ///! example: (0.0, 0.1).
    ///! 0.0: start showing sky immediately when sunset begins
    ///! 0.1: sky brightness is maxed out 10% into the night
    #[uniform(8)]
    pub night_visibility_range: Vec2,
    ///! how fast to rotate sky per sec in radians, recommended value around: 0.01
    #[uniform(9)]
    pub sky_rotation_speed: f32,
    #[uniform(10)]
    pub aurora_color_bottom: LinearRgba,
    #[uniform(11)]
    pub aurora_strength: f32, // default: 1.0
    #[uniform(12)]
    pub aurora_speed: f32, // default: 0.2
    #[uniform(13)]
    pub aurora_scale: f32, // default: 3.0
    #[uniform(14)]
    pub aurora_wave_count: f32, // default: 6.0
    #[uniform(15)]
    pub aurora_sharpness: f32, // default: 4.0
    #[uniform(16)]
    pub aurora_num_samples: i32, // default: 8 (quality vs performance)
    #[uniform(17)]
    pub aurora_base_height: f32, // default: 15.0
    #[uniform(18)]
    pub aurora_top_height: f32, // default: 35.0
    #[uniform(19)]
    pub aurora_rotation_scale: f32, // default: 0.3 (controls how much bands rotate)
    #[uniform(20)]
    pub aurora_rotation_strength: f32, // default: 8 (quality vs performance)
    #[uniform(21)]
    pub aurora_flow_speed: f32, // default: 8 (quality vs performance)
    #[uniform(22)]
    pub aurora_small_wiggle_scale: f32, // default: 0.3 (controls how much bands rotate)
    #[uniform(23)]
    pub aurora_small_wiggle_strength: f32, // default: 8 (quality vs performance)
    #[uniform(24)]
    pub aurora_small_wiggle_speed: f32, // default: 8 (quality vs performance)

    // new
    #[uniform(25)]
    pub aurora_color_top: LinearRgba,
    #[uniform(26)]
    pub aurora_sparkle_color1: LinearRgba,
    #[uniform(27)]
    pub aurora_sparkle_color2: LinearRgba,
    #[uniform(28)]
    pub aurora_sparkle_scale: f32,
    #[uniform(29)]
    pub aurora_sparkle_speed: f32,
    #[uniform(30)]
    pub aurora_sparkle_threshold: f32,
    #[uniform(31)]
    pub aurora_sparkle_max_height: f32,
    #[texture(32, dimension = "3d")]
    #[sampler(33)]
    pub noise3_image: Handle<Image>,
    #[texture(34, dimension = "3d")]
    #[sampler(35)]
    pub voronoi3_image: Handle<Image>,
}

impl Material for SkyGradientMaterial {
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

impl Default for SkyGradientMaterial {
    fn default() -> Self {
        let color_stops = [
            Vec4::new(0.2, 0.3, 0.6, 1.0),
            Vec4::new(0.4, 0.5, 1.0, 1.0),
            Vec4::new(0.35, 0.6, 0.8, 1.0),
            Vec4::new(0.5, 0.7, 1.0, 1.0),
        ];
        SkyGradientMaterial {
            color_stops,
            positions: Vec4::new(0.38, 0.47, 0.61, 1.0),
            num_stops: 4,
            sun_dir: Vec3::new(0.0, 0.1, -1.0),
            sun_color: Vec4::new(1.0, 1.0, 0.5, 1.0),
            sun_strength: 1.5,
            sun_sharpness: 164.0,
            night_time_distance: 0.0,
            night_visibility_range: vec2(0.0, 0.1),
            sky_rotation_speed: 0.01,
            // aurora_color: Vec4::new(0.6, 1.0, 0.8, 1.0),
            aurora_color_bottom: LinearRgba::new(0.0, 1.0, 0.2, 1.0),
            aurora_strength: 0.7,
            aurora_speed: 0.2,
            aurora_scale: 0.05,
            aurora_wave_count: 0.2,
            aurora_sharpness: 1.56,
            aurora_num_samples: 40,       // default: 15.0
            aurora_base_height: 3.1,      // default: 35.0
            aurora_top_height: 4.8,       // default: 0.3 (controls how much bands rotate)
            aurora_rotation_scale: 0.002, // default: 8 (quality vs performance)
            aurora_rotation_strength: 4.3,
            aurora_flow_speed: 0.005,
            aurora_small_wiggle_scale: 0.03,
            aurora_small_wiggle_strength: 1.05,
            aurora_small_wiggle_speed: 0.1,
            aurora_color_top: LinearRgba::new(0.0, 1.0, 0.8, 1.0),
            aurora_sparkle_color1: LinearRgba::new(0.0, 1.0, 0.0, 1.0),
            aurora_sparkle_color2: LinearRgba::new(0.8, 0.2, 1.0, 1.0),
            aurora_sparkle_scale: 0.004,
            aurora_sparkle_speed: 0.02,
            aurora_sparkle_threshold: 0.3,
            aurora_sparkle_max_height: 0.3,
            noise3_image: Handle::default(),
            voronoi3_image: Handle::default(),
        }
    }
}
