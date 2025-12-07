#import bevy_pbr::{
    mesh_view_bindings::view,
    utils::coords_to_viewport_uv,
}
#import bevy_pbr::mesh_view_bindings::globals;
#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}

#import "shaders/sky_gradient.wgsl"::{GradientSettings, gradient};
#import "shaders/sun.wgsl"::{SunSettings, sun};
#import "shaders/stars.wgsl"::{StarsSettings, stars};

@group(2) @binding(0)
var<uniform> gradient_settings: GradientSettings;
@group(2) @binding(1)
var<uniform> sun_settings: SunSettings;
@group(2) @binding(2)
var<uniform> stars_settings: StarsSettings;

@group(2) @binding(3)
var<uniform> night_time_distance: f32;
@group(2) @binding(4)
var<uniform> night_visibility_range: vec2<f32>;

@group(2) @binding(5)
var<uniform> feature_stars_enabled: i32;
@group(2) @binding(6)
var<uniform> feature_sun_enabled: i32;
@group(2) @binding(7)
var<uniform> feature_aurora_enabled: i32;

@group(2) @binding(10)
var noise3_texture: texture_3d<f32>;
@group(2) @binding(11)
var noise3_texture_sampler: sampler;

@group(2) @binding(12)
var voronoi3_texture: texture_3d<f32>;
@group(2) @binding(13)
var voronoi3_texture_sampler: sampler;

@group(2) @binding(14)
var aurora_texture: texture_2d<f32>;
@group(2) @binding(15)
var aurora_texture_sampler: sampler;

struct VertexOutput {
    @builtin(position) frag_pos: vec4<f32>,
    @location(0) world_dir: vec3<f32>,
};

@vertex
fn vertex(@location(0) position: vec3<f32>, @builtin(instance_index) vertin: u32) -> VertexOutput {
    var out: VertexOutput;
    let world_pos = position; // since the sky sphere is centered on camera
    out.world_dir = normalize(world_pos);
    out.frag_pos = mesh_position_local_to_clip(get_world_from_local(vertin), vec4<f32>(position, 1.0));
    return out;
}

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    // only show star in night
    let night_visibility = smoothstep(night_visibility_range.x,
        night_visibility_range.y,
        night_time_distance);

    let view_dir = normalize(in.world_dir);

    var final_color = vec4f(0.0,0.0,0.0,1.0);

    let base_color = gradient(view_dir, gradient_settings);
    final_color += base_color;

    if feature_sun_enabled == 1 {
        // show sun in night but at less transparency
        let day_vis = max((1.0-night_visibility), 0.05);
        final_color += sun(view_dir, sun_settings) * day_vis;
    }

    if feature_stars_enabled == 1 {
        let star = stars(view_dir,
            stars_settings,
            globals.time,
            noise3_texture,
            noise3_texture_sampler,
            voronoi3_texture,
            voronoi3_texture_sampler,
        );
        final_color += star * night_visibility;
    }

    if feature_aurora_enabled == 1 {
        // aurora
        let screen_uv = in.frag_pos.xy / view.viewport.zw;
        let north = textureSample(aurora_texture, aurora_texture_sampler, screen_uv).rgba;
        final_color += north * night_visibility;
    }

    return final_color;
}


