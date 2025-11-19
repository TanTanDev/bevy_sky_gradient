#import bevy_pbr::{
    mesh_view_bindings::view,
    // forward_io::VertexOutput,
    utils::coords_to_viewport_uv,
}

#import bevy_pbr::mesh_view_bindings::globals;

#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}


@group(2) @binding(0)
var<uniform> color_stops: array<vec4<f32>, 4>;
@group(2) @binding(1)
var<uniform> positions: vec4<f32>;
@group(2) @binding(2)
var<uniform> num_stops: u32;

@group(2) @binding(3)
var<uniform> sun_dir: vec3<f32>;
@group(2) @binding(4)
var<uniform> sun_color: vec4<f32>;
@group(2) @binding(5)
var<uniform> sun_strength: f32;
@group(2) @binding(6)
var<uniform> sun_sharpness: f32;
@group(2) @binding(7)
var<uniform> night_time_distance: f32;


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

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let view_dir = normalize(in.world_dir);
    // ---- Vertical gradient ----
    let t = clamp(view_dir.y * 0.5 + 0.5, 0.0, 1.0);

    
    let angle = globals.time*0.005;
    let c = cos(angle);
    let s = sin(angle);
    let rotation_matrix = mat3x3<f32>(
        vec3<f32>(c,-s,0.0),//new x basis vector
        vec3<f32>(s, c, 0.0),//new y basis vector
        vec3<f32>(0.0,0.0,1.0),//new z basis vector
    );

    let offset_world_dir = rotation_matrix * in.world_dir.xyz;
    let mask = noise3(offset_world_dir*5.0);
    let r = noise3(offset_world_dir*20.0);
    var noise = 1.0-voronoi3(offset_world_dir*50.0).x;
    var noise2 = 1.0-voronoi3(offset_world_dir*50.0 + (cos(globals.time*50.0+r*60.0))*0.02).x;
    // noise = noise * mask;
    // noise = pow(mask, 3.0);
    // noise2 = noise2 * pow(mask, 3.0);

    var star_intensity = smoothstep(0.93, 1.0, noise);
    
    var glow = smoothstep(0.91, 1.0, noise2);
    // let star = (star_intensity + glow*0.2*((sin(globals.time*5.0 + r*6.0)*0.5)+0.5))*pow(mask, 3.0);
    let star = (star_intensity + glow*0.2*((sin(globals.time*5.0 + r*6.0)*0.5)+0.5))*pow(mask, 3.0);


    var base_color: vec4<f32> = color_stops[0];
    // if below first stop
    if (t <= positions[0]) {
        base_color = color_stops[0];
    }
    // if above last stop
    else if (t >= positions[num_stops - 1u]) {
        base_color = color_stops[num_stops - 1u];
    }
    // otherwise, find segment and interpolate
    else {
        for (var i: u32 = 1u; i < num_stops; i = i + 1u) {
            let a = positions[i - 1u];
            let b = positions[i];
            if (t >= a && t <= b) {
                let f = (t - a) / (b - a);
                base_color = mix(color_stops[i - 1u], color_stops[i], f);
                break;
            }
        }
    }
    // ---- Sun glow term ----
    let sun_factor = pow(max(dot(view_dir, normalize(sun_dir)), 0.0), sun_sharpness);
    let sun_contrib = sun_color * (sun_factor * sun_strength);

    let star_visibility = smoothstep(0.4, 0.9, night_time_distance);
    return base_color + sun_contrib + star * star_visibility;
}


// MIT License. © Stefan Gustavson, Munrocket
//
fn mod289(x: vec4f) -> vec4f { return x - floor(x * (1. / 289.)) * 289.; }
fn perm4(x: vec4f) -> vec4f { return mod289(((x * 34.) + 1.) * x); }

fn noise3(p: vec3f) -> f32 {
    let a = floor(p);
    var d: vec3f = p - a;
    d = d * d * (3. - 2. * d);

    let b = a.xxyy + vec4f(0., 1., 0., 1.);
    let k1 = perm4(b.xyxy);
    let k2 = perm4(k1.xyxy + b.zzww);

    let c = k2 + a.zzzz;
    let k3 = perm4(c);
    let k4 = perm4(c + 1.);

    let o1 = fract(k3 * (1. / 41.));
    let o2 = fract(k4 * (1. / 41.));

    let o3 = o2 * d.z + o1 * (1. - d.z);
    let o4 = o3.yw * d.x + o3.xz * (1. - d.x);

    return o4.y * d.y + o4.x * (1. - d.y);
}


fn voronoi3(p: vec3f) -> vec2f {
    let n = floor(p);
    let f = fract(p);
    
    var min_dist = 1.0;
    var second_min = 1.0;
    
    for (var k = -1; k <= 1; k++) {
        for (var j = -1; j <= 1; j++) {
            for (var i = -1; i <= 1; i++) {
                let g = vec3f(f32(i), f32(j), f32(k));
                let o = hash33(n + g);  // Random offset for this cell
                let r = g + o - f;
                let d = length(r);
                
                if d < min_dist {
                    second_min = min_dist;
                    min_dist = d;
                } else if d < second_min {
                    second_min = d;
                }
            }
        }
    }
    
    return vec2f(min_dist, second_min);
}

// Hash function for Voronoi
fn hash33(p: vec3f) -> vec3f {
    var p3 = fract(p * vec3f(0.1031, 0.1030, 0.0973));
    p3 += dot(p3, p3.yxz + 33.33);
    return fract((p3.xxy + p3.yxx) * p3.zyx);
}

fn hash31(p: vec3f) -> f32 {
    var p3 = fract(p * 0.1031);
    p3 += dot(p3, p3.zyx + 31.32);
    return fract((p3.x + p3.y) * p3.z);
}
