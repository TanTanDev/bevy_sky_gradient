#import bevy_pbr::{
    mesh_view_bindings::view,
    utils::coords_to_viewport_uv,
}
#import bevy_pbr::mesh_view_bindings::globals;
#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}

#import "shaders/aurora.wgsl"::{AuroraSettings, aurora};

@group(2) @binding(0)
var<uniform> aurora_settings: AuroraSettings;

@group(2) @binding(1)
var noise3_texture: texture_3d<f32>;
@group(2) @binding(2)
var noise3_texture_sampler: sampler;

@group(2) @binding(3)
var voronoi3_texture: texture_3d<f32>;
@group(2) @binding(4)
var voronoi3_texture_sampler: sampler;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_dir: vec3<f32>,
};

@vertex
fn vertex(@location(0) position: vec3<f32>, @builtin(instance_index) vertin: u32) -> VertexOutput {
    var out: VertexOutput;
    let world_pos = position; // since the sky sphere is centered on camera
    out.world_dir = normalize(world_pos);
    out.clip_position = mesh_position_local_to_clip(get_world_from_local(vertin), vec4<f32>(position, 1.0));
    return out;
}

// AURORA ONLY render
@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let view_dir = normalize(in.world_dir);
    let north = aurora(view_dir, aurora_settings, globals.time, noise3_texture, noise3_texture_sampler);
    return north;
}


