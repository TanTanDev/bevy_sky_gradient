#import bevy_pbr::{
    mesh_view_bindings::view,
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
@group(2) @binding(8)
var<uniform> night_visibility_range: vec2<f32>;
@group(2) @binding(9)
var<uniform> sky_rotation_speed: f32;

@group(2) @binding(10)
var<uniform> aurora_bottom_color: vec4<f32>;       // default: vec4<f32>(0.6,1.0,0.8,1.0)
@group(2) @binding(11)
var<uniform> aurora_strength: f32;         // default: 1.0
@group(2) @binding(12)
var<uniform> aurora_speed: f32;            // default: 0.2
@group(2) @binding(13)
var<uniform> aurora_scale: f32;            // default: 3.0
@group(2) @binding(14)
var<uniform> aurora_wave_count: f32;       // default: 6.0
@group(2) @binding(15)
var<uniform> aurora_sharpness: f32;        // default: 4.0

@group(2) @binding(16)
var<uniform> aurora_num_samples: i32;      // default: 8 (quality vs performance)
// GOOD
@group(2) @binding(17)
var<uniform> aurora_base_height: f32;      // default: 15.0
@group(2) @binding(18)
var<uniform> aurora_top_height: f32;       // default: 35.0
@group(2) @binding(19)
var<uniform> aurora_rotation_scale: f32;   // default: 0.3 (controls how much bands rotate)
@group(2) @binding(20)
var<uniform> aurora_rotation_strength: f32;   // default: 0.3 (controls how much bands rotate)
@group(2) @binding(21)
var<uniform> aurora_flow_speed: f32;   // default: 0.3 (controls how much bands rotate)
@group(2) @binding(22)
var<uniform> aurora_small_wiggle_scale: f32; // default: 0.3 (controls how much bands rotate)
@group(2) @binding(23)
var<uniform> aurora_small_wiggle_strength: f32; // default: 8 (quality vs performance)
@group(2) @binding(24)
var<uniform> aurora_small_wiggle_speed: f32; // default: 8 (quality vs performance)

@group(2) @binding(25)
var<uniform> aurora_color_top: vec4<f32>;
@group(2) @binding(26)
var<uniform> aurora_sparkle_color1: vec4f;
@group(2) @binding(27)
var<uniform> aurora_sparkle_color2: vec4f;
@group(2) @binding(28)
var<uniform> aurora_sparkle_scale: f32;
@group(2) @binding(29)
var<uniform> aurora_sparkle_speed: f32;
@group(2) @binding(30)
var<uniform> aurora_sparkle_threshold: f32;
@group(2) @binding(31)
var<uniform> aurora_sparkle_max_height: f32;



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

// view_dir is direction this fragment is looking towards
// returns a star color
fn stars(view_dir: vec3<f32>) -> f32 {
    // STARS
    let sky_rotation = globals.time * sky_rotation_speed;
    let c = cos(sky_rotation);
    let s = sin(sky_rotation);
    let rotation_matrix = mat3x3<f32>(
        vec3<f32>(c,-s,0.0),// new x basis vector
        vec3<f32>(s, c, 0.0),// new y basis vector
        vec3<f32>(0.0,0.0,1.0),// new z basis vector
    );
    let offset_world_dir = rotation_matrix * view_dir;

    var noise = 1.0-voronoi3(offset_world_dir*60.0).x;
    let mask = noise3(offset_world_dir*70.0);
    let variance_noise = noise3(offset_world_dir*20.0);

    // reduce star density with mask
    noise = noise * (1.0-pow(mask, 6.0));

    let blink_speed = globals.time * 20.0;
    // star blink in different speeds
    let speed_variance = (1.0 + sin(variance_noise * 30.0)*0.50);
    // stars should not blink the same time
    let blink_offset = variance_noise * 10.0;
    // 0: no blink, 1: full blink
    let blink = cos(blink_speed*speed_variance+blink_offset);
    // how large the blink is visually
    let blink_strength = 0.01;
    // reduce voroni to "dots" in range 0.93-1
    var star_intensity = smoothstep(0.93+blink*blink_strength, 1.0, noise);
    return star_intensity;
}

fn gradient(view_dir: vec3<f32>) -> vec4<f32> {
    let t = clamp(view_dir.y * 0.5 + 0.5, 0.0, 1.0);
    // GRADIENT
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
    return base_color;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let view_dir = normalize(in.world_dir);

    let base_color = gradient(view_dir);

    let sun_factor = pow(max(dot(view_dir, normalize(sun_dir)), 0.0), sun_sharpness);
    let sun_contrib = sun_color * (sun_factor * sun_strength);
  
    let star = stars(view_dir);
    // only show star in night
    let night_visibility = smoothstep(night_visibility_range.x,
        night_visibility_range.y,
        night_time_distance);

    let north = aurora(view_dir);

    return base_color + sun_contrib + star * night_visibility + north * night_visibility;
}

// MIT License. © Stefan Gustavson, Munrocket
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


fn aurora(view_dir: vec3<f32>) -> vec4<f32> {
    // Ensure at least 2 samples
    let samples = max(aurora_num_samples, 2);
    
    var accumulated_color = vec3<f32>(0.0);
    var accumulated_alpha = 0.0;
    
    // ---- Step 1: Sample along the view ray at different heights ----
    for (var i = 0; i < samples; i++) {
        let height_factor = f32(i) / f32(samples - 1);
        let height = aurora_base_height + (aurora_top_height - aurora_base_height) * height_factor;
        
        // Find where THIS ray hits THIS specific height plane
        let denom = view_dir.y;
        if abs(denom) < 0.001 {
            return vec4<f32>(0.0);
        }
        let t = height / denom;
        if (t <= 0.0) {
            continue;
        }
        
        let p = view_dir * t;
        let world_pos = p.xz;
        
        let s_wp = world_pos * aurora_rotation_scale;
        let flow = vec2<f32>(
            noise3(vec3<f32>(s_wp.x, s_wp.y, 1.0+ globals.time * aurora_flow_speed)),
            noise3(vec3<f32>(s_wp.x, s_wp.y, 7.7+ globals.time * aurora_flow_speed)));

        let w_wo = world_pos * aurora_small_wiggle_scale;
        let time_offset = w_wo.x+w_wo.y;
        var small_wiggle = vec2<f32>(
            noise3(vec3<f32>(w_wo.x, w_wo.y, 10.0 + globals.time * aurora_small_wiggle_speed+time_offset)),
            noise3(vec3<f32>(w_wo.x, w_wo.y, 70.7 + globals.time * aurora_small_wiggle_speed+time_offset)));
        
        small_wiggle = small_wiggle * aurora_small_wiggle_strength;
        // let flow_dir = normalize(flow * 2.0 - 1.0);
        let flow_dir = normalize(flow);
       
        let warped_pos = world_pos + flow_dir * aurora_rotation_strength + small_wiggle;
        // Create bands
        let band_coord = warped_pos.x * aurora_scale;
        // let band_coord = (rotated_pos.x * aurora_scale + dance_offset + band_noise) * aurora_wave_count;
        let base_value = fract(band_coord);

        let stripe = pow(smoothstep(0.3, 0.5, base_value) * smoothstep(0.7, 0.5, base_value), aurora_sharpness);
        
        // Secondary bands 50% weaker, but 3 times thinner, and more frequent
        let secondary_value = fract(band_coord * 1.7 + 0.3);
        let secondary = pow(smoothstep(0.35, 0.5, secondary_value) * smoothstep(0.65, 0.5, secondary_value), aurora_sharpness * 3.0);
        
        let base_stripe = max(stripe, secondary * 0.5);
       
        // Vertical falloff - bright at bottom, fade at top (this creates the curtain look)
        let vertical_intensity = smoothstep(0.0, 0.15, height_factor) * 
                                 smoothstep(1.0, 0.5, height_factor);

        // sparkles
        let sparkle_intensity = 1.0 - smoothstep(0.0, aurora_sparkle_max_height, height_factor);
        let sn = warped_pos * aurora_sparkle_scale;
        let k = globals.time * aurora_sparkle_speed;
        let sparkle_noise = smoothstep(aurora_sparkle_threshold, 1.0, 1.0-noise3(vec3f(sn.x+k, sn.y+k, k*0.3)));
        let sparkle_color_noise = noise3(vec3f(sn.x*0.3+10.0, sn.y*0.3+10.0, 0.0));
        let sparkle_color = mix(aurora_sparkle_color1, aurora_sparkle_color2, smoothstep(0.4,1.0,sparkle_color_noise));
        let sparkle = pow(base_stripe,3.0) * sparkle_intensity * sparkle_color.rgb * sparkle_noise;
        
        var curtain = base_stripe * vertical_intensity;
        
        // ---- Step 4: Accumulate with proper alpha blending ----
        let sample_alpha = curtain * 0.18;
        let sample_weight = sample_alpha * (1.0 - accumulated_alpha);

        let selected_color = mix(aurora_bottom_color.rgb, aurora_color_top.rgb, height_factor);
        
        accumulated_color += selected_color * curtain * sample_weight + sparkle;
        // accumulated_color += aurora_bottom_color.rgb * curtain * sample_weight;
        accumulated_alpha += sample_alpha * (1.0 - accumulated_alpha);
        
        if (accumulated_alpha > 0.95) {
            break;
        }
    }
    
    // Fade by view angle
    let up_factor = smoothstep(0.05, 0.7, view_dir.y);
    
    let final_alpha = accumulated_alpha * up_factor * aurora_strength;
    
    return vec4<f32>(accumulated_color * up_factor * aurora_strength, final_alpha);
}

