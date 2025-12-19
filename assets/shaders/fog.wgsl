#import bevy_pbr::{
    mesh_view_bindings::view,
    utils::coords_to_viewport_uv,
    forward_io::VertexOutput,
}

#import bevy_pbr::mesh_view_bindings::globals;
#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}

struct FogSettings {
    color: vec3f,
    distance_start: f32,
    distance_end: f32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<uniform> fog_settings: FogSettings;

@group(#{MATERIAL_BIND_GROUP}) @binding(1)
var sky_gradient_texture: texture_2d<f32>;

@group(#{MATERIAL_BIND_GROUP}) @binding(2)
var sky_gradient_sampler: sampler;

// this is an example how your own shaders that needs fog, can sample the sky gradient texture
@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var final_color = fog_settings.color;
    let screen_uv = coords_to_viewport_uv(in.position.xy, view.viewport);
    let sky_color_sample = textureSample(sky_gradient_texture, sky_gradient_sampler, screen_uv).rgb;

    let fog_color = sky_color_sample;
    
    let distance = length(in.world_position.xyz - view.world_position.xyz);
    let fog_factor = smoothstep(
        fog_settings.distance_start,
        fog_settings.distance_end,
        distance
    );

    final_color = mix(final_color, fog_color, fog_factor);
    
    return vec4<f32>(final_color, 1.0);
}
