#import bevy_pbr::{
    mesh_view_bindings::view,
    utils::coords_to_viewport_uv,
}
#import bevy_pbr::mesh_view_bindings::globals;
#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}

#import "bevy_sky_gradient/shaders/gradient.wgsl"::{GradientSettings, gradient};

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<uniform> gradient_settings: GradientSettings;

struct VertexOutput {
    @builtin(position) frag_pos: vec4<f32>,
    @location(0) world_dir: vec3<f32>,
};

@vertex
fn vertex(@location(0) position: vec3<f32>, @builtin(instance_index) vertin: u32) -> VertexOutput {
    var out: VertexOutput;
    let world_pos = position;
    out.world_dir = normalize(world_pos);
    out.frag_pos = mesh_position_local_to_clip(get_world_from_local(vertin), vec4<f32>(position, 1.0));
    return out;
}

// only render the gradient effect
@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let view_dir = normalize(in.world_dir);
    return gradient(view_dir, gradient_settings);
}

