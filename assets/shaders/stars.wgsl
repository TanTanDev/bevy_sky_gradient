#import "shaders/noise.wgsl"::{noise};

struct StarsSettings {
    sky_rotation_speed: f32,
}

fn stars(
    view_dir: vec3<f32>,
    stars: StarsSettings,
    global_time: f32,
   // noise and voronoi noise
    n3_t: texture_3d<f32>,
    n3_s: sampler,
    v3_t: texture_3d<f32>,
    v3_s: sampler,
) -> f32 {
    let sky_rotation = global_time * stars.sky_rotation_speed;
    let c = cos(sky_rotation);
    let s = sin(sky_rotation);
    let rotation_matrix = mat3x3<f32>(
        vec3<f32>(c,-s,0.0),// new x basis vector
        vec3<f32>(s, c, 0.0),// new y basis vector
        vec3<f32>(0.0,0.0,1.0),// new z basis vector
    );
    let offset_world_dir = rotation_matrix * view_dir;

    var noise = 1.0-noise(v3_t, v3_s,offset_world_dir*9.0);
    let mask = noise(n3_t, n3_s, offset_world_dir);
    let variance_noise = noise(n3_t, n3_s, offset_world_dir*0.1);

    // reduce star density with mask
    noise = noise * (1.0-pow(mask, 6.0));

    let blink_speed = global_time * 10.0;
    // star blink in different speeds
    let speed_variance = (1.0 + sin(variance_noise * 1.3)*0.50);
    // stars should not blink the same time
    let blink_offset = variance_noise * 0.1;
    // 0: no blink, 1: full blink
    let blink = cos(blink_speed*speed_variance+blink_offset);
    // how large the blink is visually
    let blink_strength = 0.01;
    // reduce voroni to "dots" in range 0.93-1
    var star_intensity = smoothstep(0.9+blink*blink_strength, 1.0, noise);
    return star_intensity;
}

